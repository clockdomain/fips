struct MemoryRegions {
    rom: &'static mut [u8],
    mbox: &'static mut [u8],
    iccm: &'static mut [u8],
    dccm: &'static mut [u8],
    man1: &'static mut [u8],
    man2: &'static mut [u8],
    fht: &'static mut [u8],
    ldevid_tbs: &'static mut [u8],
    fmcalias_tbs: &'static mut [u8],
    pcr_log: &'static mut [u8],
    fuse_log: &'static mut [u8],
    data: &'static mut [u8],
    stack: &'static mut [u8],
    estack: &'static mut [u8],
    nstack: &'static mut [u8],
}

impl MemoryRegions {
    // Create a new instance of MemoryRegions with slices based on memory addresses and sizes
    fn new() -> Self {
        Self {
            man1: unsafe { create_slice(MAN1_ORG, MAN1_SIZE) },
            man2: unsafe { create_slice(MAN2_ORG, MAN2_SIZE) },
            fht: unsafe { create_slice(FHT_ORG, FHT_SIZE) },
            ldevid_tbs: unsafe { create_slice(LDEVID_TBS_ORG, LDEVID_TBS_SIZE) },
            fmcalias_tbs: unsafe { create_slice(FMCALIAS_TBS_ORG, FMCALIAS_TBS_SIZE) },
            pcr_log: unsafe { create_slice(PCR_LOG_ORG, PCR_LOG_SIZE) },
            fuse_log: unsafe { create_slice(FUSE_LOG_ORG, FUSE_LOG_SIZE) },
        }
    }
}

// Helper function to create a mutable slice from a memory region
unsafe fn create_slice(org: usize, size: usize) -> &mut [u8] {
    let ptr = org as *mut u8;
    core::slice::from_raw_parts_mut(ptr, size)
}
