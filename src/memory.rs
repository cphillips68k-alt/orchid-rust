use bootloader::BootInfo;
use bootloader::bootinfo::MemoryMap;

static mut MEMORY_MAP: Option<&'static MemoryMap> = None;

pub fn init(boot_info: &'static BootInfo) {
    unsafe {
        MEMORY_MAP = Some(boot_info.memory_regions());
    }
}

pub fn region_count() -> usize {
    unsafe { MEMORY_MAP.map(|map| map.iter().count()).unwrap_or(0) }
}
