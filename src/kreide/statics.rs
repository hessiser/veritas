use std::sync::LazyLock;

// This is to keep track of static offsets that need to be manually updated
pub static MODULEMANAGER_FIELD_OFFSET: isize = 0x1138;

pub static MODULES_PTR_OFFSET: LazyLock<usize> = lazy_initialize_address!(0x4c0d688);
