use std::sync::LazyLock;

// This is to keep track of static offsets that need to be manually updated
pub static TEXTID_TYPE_PTR_OFFSET: LazyLock<usize> = lazy_initialize_address!(0x46c3380);

pub static MODULES_PTR_OFFSET: LazyLock<usize> = lazy_initialize_address!(0x40ee438);

pub static MODULEMANAGER_FIELD_OFFSET: isize = 0xe78;