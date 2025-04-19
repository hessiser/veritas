use anyhow::Result;
use std::{
    ffi::c_void,
    mem::{self},
    ptr::null_mut,
};
use windows::{
    core::{w, Interface},
    Win32::{
        Foundation::HMODULE,
        Graphics::{
            Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0},
            Direct3D11::{
                D3D11CreateDeviceAndSwapChain, ID3D11Device, ID3D11DeviceContext,
                D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION,
            },
            Dxgi::{
                Common::{
                    DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC,
                    DXGI_MODE_SCALING_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    DXGI_RATIONAL, DXGI_SAMPLE_DESC,
                },
                IDXGISwapChain, DXGI_SWAP_CHAIN_DESC,
                DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_EFFECT_DISCARD,
                DXGI_USAGE_RENDER_TARGET_OUTPUT,
            },
        },
        UI::
            WindowsAndMessaging::{
                CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassExW,
                UnregisterClassW, CS_HREDRAW, CS_VREDRAW, WINDOW_EX_STYLE, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
            }
        ,
    },
};

use crate::ui::app::App;

// rdbo Kiero
// https://github.com/eugen15/directx-present-hook

// This can be done in shorter calls
// Should we tho?
pub fn get_vtable() -> Box<[usize; 205]> {
    // Initializes a dummy swapchain to get the vtable
    unsafe {
        let window_class = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as _,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(mem::transmute(DefWindowProcW as *const c_void)),
            lpszClassName: w!("veritas"),
            ..Default::default()
        };

        RegisterClassExW(&window_class);

        let window = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class.lpszClassName,
            w!("veritas DirectX Window"),
            WS_OVERLAPPEDWINDOW,
            0,
            0,
            100,
            100,
            None,
            None,
            Some(window_class.hInstance),
            None,
        )
        .unwrap();

        let mut feature_levels = [D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0];

        let refresh_rate = DXGI_RATIONAL {
            Numerator: 60,
            Denominator: 1,
        };

        let buffer_desc = DXGI_MODE_DESC {
            Width: 100,
            Height: 100,
            RefreshRate: refresh_rate,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
            Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
        };

        let sample_desc = DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        };

        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: buffer_desc,
            SampleDesc: sample_desc,
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 1,
            OutputWindow: window,
            Windowed: true.into(),
            SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
            Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0 as u32,
        };

        let mut swap_chain: Option<IDXGISwapChain> = None;
        let mut device: Option<ID3D11Device> = None;
        let mut context: Option<ID3D11DeviceContext> = None;

        D3D11CreateDeviceAndSwapChain(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            HMODULE(null_mut()),
            D3D11_CREATE_DEVICE_FLAG(0),
            Some(&feature_levels.clone()),
            D3D11_SDK_VERSION,
            Some(&swap_chain_desc),
            Some(&mut swap_chain),
            Some(&mut device),
            Some(feature_levels.as_mut_ptr()),
            Some(&mut context),
        )
        .unwrap();

        let mut vtable = Box::new([0usize; 205]);

        let swap_chain_ptr = &swap_chain.unwrap();
        let swap_chain_vtable = Interface::vtable(swap_chain_ptr);

        let device_ptr = &device.unwrap();
        let device_vtable = Interface::vtable(device_ptr);

        let context_ptr = &context.unwrap();
        let context_vtable = Interface::vtable(context_ptr);

        std::ptr::copy_nonoverlapping(mem::transmute(swap_chain_vtable), vtable.as_mut_ptr(), 18);

        std::ptr::copy_nonoverlapping(
            mem::transmute(&device_vtable),
            vtable[18..].as_mut_ptr(),
            43,
        );

        std::ptr::copy_nonoverlapping(
            mem::transmute(context_vtable),
            vtable[18 + 43..].as_mut_ptr(),
            144,
        );

        DestroyWindow(window).unwrap();
        UnregisterClassW(window_class.lpszClassName, Some(window_class.hInstance)).unwrap();

        vtable
    }
}


pub fn initialize() -> Result<()> {
    let vtable = get_vtable();
    unsafe {
        edio11::set_overlay(Box::new(|ctx| {
            let mut app = App::new(ctx);
            app.set_menu_keybind(egui::Key::M, Some(egui::Modifiers {
                ctrl: true,
                ..Default::default()
            }));
            Box::new(app)
        }), mem::transmute(vtable[8]), mem::transmute(vtable[13])).unwrap();
    }
    Ok(())
}
