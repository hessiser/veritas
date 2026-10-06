use std::{collections::HashMap, ffi::c_void};

use iced_x86::{
    Decoder, DecoderOptions, Instruction, InstructionInfoFactory, OpAccess, OpKind, Register,
};
use windows::Win32::System::{Diagnostics::Debug::ReadProcessMemory, Threading::GetCurrentProcess};
use anyhow::Result;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct MemorySource {
    base: Register,
    index: Register,
    scale: u32,
    displacement: u64,
}

#[derive(Clone, Copy, Debug)]
struct RegisterAlias {
    source: MemorySource,
    loaded_at: usize,
}

#[derive(Clone, Copy, Debug)]
struct ObjectFieldStore {
    instruction_index: usize,
    object_loaded_at: usize,
    object_source: MemorySource,
    offset: u32,
}

fn canonical_register(register: Register) -> Register {
    register.full_register()
}

fn memory_source(memory: &iced_x86::UsedMemory) -> MemorySource {
    MemorySource {
        base: canonical_register(memory.base()),
        index: canonical_register(memory.index()),
        scale: memory.scale(),
        displacement: memory.displacement(),
    }
}

fn is_memory_read(access: OpAccess) -> bool {
    matches!(access, OpAccess::Read | OpAccess::ReadWrite)
}

fn is_memory_write(access: OpAccess) -> bool {
    matches!(access, OpAccess::Write | OpAccess::ReadWrite)
}

fn decode_field_stores(bytes: &[u8]) -> Vec<ObjectFieldStore> {
    let mut decoder = Decoder::with_ip(64, bytes, 0, DecoderOptions::NONE);
    let mut info_factory = InstructionInfoFactory::new();
    let mut aliases = HashMap::<Register, RegisterAlias>::new();
    let mut stores = Vec::new();
    let mut instruction = Instruction::default();
    let mut instruction_index = 0;

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        let info = info_factory.info(&instruction);
        let used_memory = info.used_memory();

        let read_memory = used_memory
            .iter()
            .filter(|memory| is_memory_read(memory.access()))
            .collect::<Vec<_>>();
        let write_memory = used_memory
            .iter()
            .filter(|memory| is_memory_write(memory.access()))
            .collect::<Vec<_>>();

        let written_registers = info
            .used_registers()
            .iter()
            .filter(|used| matches!(used.access(), OpAccess::Write | OpAccess::ReadWrite))
            .map(|used| canonical_register(used.register()))
            .filter(|register| *register != Register::None)
            .collect::<Vec<_>>();

        for register in &written_registers {
            aliases.remove(register);
        }

        // A memory read which writes exactly one register is the lightweight
        // data-flow fact used by both object acquisition and value loading.
        let is_register_from_memory_load = instruction.op_count() == 2
            && instruction.op_kind(0) == OpKind::Register
            && instruction.op_kind(1) == OpKind::Memory;
        if is_register_from_memory_load && read_memory.len() == 1 && written_registers.len() == 1 {
            let destination = written_registers[0];
            aliases.insert(
                destination,
                RegisterAlias {
                    source: memory_source(read_memory[0]),
                    loaded_at: instruction_index,
                },
            );
        }

        let is_register_to_memory_store = instruction.op_count() == 2
            && instruction.op_kind(0) == OpKind::Memory
            && instruction.op_kind(1) == OpKind::Register;
        if is_register_to_memory_store
            && write_memory.len() == 1
            && write_memory[0].base() != Register::None
            && write_memory[0].index() == Register::None
            && write_memory[0].displacement() <= u32::MAX as u64
        {
            let object_register = canonical_register(write_memory[0].base());
            let object_alias = aliases.get(&object_register).copied();
            let value_register = info
                .used_registers()
                .iter()
                .filter(|used| {
                    used.access() == OpAccess::Read
                        && canonical_register(used.register()) != object_register
                })
                .map(|used| canonical_register(used.register()))
                .find(|register| aliases.contains_key(register));

            if let (Some(object_alias), Some(value_register)) = (object_alias, value_register) {
                let value_alias = aliases[&value_register];
                if value_alias.loaded_at >= object_alias.loaded_at
                    && instruction_index - object_alias.loaded_at <= 8
                    && instruction_index - value_alias.loaded_at <= 8
                {
                    stores.push(ObjectFieldStore {
                        instruction_index,
                        object_loaded_at: object_alias.loaded_at,
                        object_source: object_alias.source,
                        offset: write_memory[0].displacement() as u32,
                    });
                }
            }
        }

        instruction_index += 1;
    }

    stores
}

/// Extracts all four field offsets from the repeated object-store block.
///
/// `bytes` must contain only the already-resolved target function, not the
/// surrounding module. The matcher requires four stores sharing the same
/// object-source memory expression, with a fresh object acquisition before
/// every store after the first.
/// Disclaimer: This function was generated by an AI assistant
pub fn extract_repeated_object_field_offsets(bytes: &[u8]) -> Option<(u32, u32, u32, u32)> {
    let stores = decode_field_stores(bytes);

    for window in stores.windows(4) {
        let source = window[0].object_source;
        if window.iter().any(|store| store.object_source != source) {
            continue;
        }
        if window
            .iter()
            .any(|store| store.instruction_index - window[0].instruction_index > 24)
        {
            continue;
        }
        if window
            .windows(2)
            .any(|pair| pair[1].object_loaded_at <= pair[0].instruction_index)
        {
            continue;
        }
        if window[0].offset == window[1].offset
            || window[1].offset == window[2].offset
            || window[2].offset == window[3].offset
        {
            continue;
        }
        return Some((
            window[0].offset,
            window[1].offset,
            window[2].offset,
            window[3].offset,
        ));
    }

    None
}

pub unsafe fn get_fn_bytes(fn_ptr: *const c_void, fn_size: usize) -> Result<Vec<u8>> {
    let process_handle = unsafe { GetCurrentProcess() };
    let buffer = vec![0u8; fn_size];
    let mut bytes_read = 0usize;

    ReadProcessMemory(
        process_handle,
        fn_ptr,
        buffer.as_ptr() as _,
        fn_size,
        Some(&mut bytes_read),
    )?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::extract_repeated_object_field_offsets;

    #[test]
    fn finds_offsets_without_matching_register_names() {
        let bytes = [
            0x4C, 0x8B, 0x17, // mov r10,[rdi]
            0x4C, 0x8B, 0x1D, 0, 0, 0, 0, // mov r11,[rip]
            0x4D, 0x89, 0x9A, 0xD0, 0x06, 0, 0, // mov [r10+6D0h],r11
            0x4C, 0x8B, 0x27, // mov r12,[rdi]
            0x4C, 0x8B, 0x2D, 0, 0, 0, 0, // mov r13,[rip]
            0x4D, 0x89, 0xAC, 0x24, 0x60, 0x03, 0, 0, // mov [r12+360h],r13
            0x4C, 0x8B, 0x07, // mov r8,[rdi]
            0x4C, 0x8B, 0x0D, 0, 0, 0, 0, // mov r9,[rip]
            0x4D, 0x89, 0x88, 0xD8, 0x06, 0, 0, // mov [r8+6D8h],r9
            0x4C, 0x8B, 0x37, // mov r14,[rdi]
            0x4C, 0x8B, 0x3D, 0, 0, 0, 0, // mov r15,[rip]
            0x4D, 0x89, 0xBE, 0xC8, 0x01, 0, 0, // mov [r14+1C8h],r15
        ];

        assert_eq!(
            extract_repeated_object_field_offsets(&bytes),
            Some((0x6D0, 0x360, 0x6D8, 0x1C8))
        );
    }
}
