/// A simple job queue that can be used to queue up jobs to be 
/// executed by the runtime.
/// # Examples
/// ```
/// use caliptra_runtime::JobQueue;
/// let mut job_queue = JobQueue::new();
/// job_queue.add(|| {
///    println!("Hello from a job!");
///   Ok(())
/// });
/// job_queue.execute_next();
/// ```
/// # Notes
/// 
pub struct JobQueue {
    /// The queue of jobs
    queue: [Option<fn() -> CaliptraResult<()>>; 8],
    /// The index of the next job to execute
    next: usize,
    /// The index of the next job to add
    add: usize,
}

impl JobQueue {
    /// Create a new instance of the job queue
    pub fn new() -> Self {
        Self {
            queue: [None; 8],
            next: 0,
            add: 0,
        }
    }
    /// Increment next index, wrap around if needed
    fn increment_next(&mut self) {
        self.next = (self.next + 1) % self.queue.len();
    }
    /// Increment add index, wrap around if needed
    fn increment_add(&mut self) {
        self.add = (self.add + 1) % self.queue.len();
    }

    /// Add a job to the queue
    pub fn add(&mut self, job: fn() -> CaliptraResult<()>) -> CaliptraResult<()> {
        // Check if the queue is full, return CaliptraError if it is
        if self.queue[self.add].is_some() {
            cprintln!("[rt] Job queue is full");
            return Err(CaliptraError::RUNTIME_JOB_QUEUE_FULL);
        }

        self.queue[self.add] = Some(job);
        self.increment_add();
        Ok(())
    }

    /// Execute the next job in the queue
    pub fn execute_next(&mut self) -> CaliptraResult<()> {
        if let Some(job) = self.queue[self.next] {
            job()?;
            self.increment_next();
        }
        Ok(())
    }
}


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
