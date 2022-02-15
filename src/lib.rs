use std::os::unix::raw::pid_t;
use procfs;
use procfs::ProcResult;

pub struct Memory{
    total: usize,
    free: usize,
    available: usize,
    used: usize,
    process_vm: usize,
    process_vm_peak: usize,
    process_rss: usize,
    process_hwm: usize
}

pub struct Watcher{
    proc: procfs::process::Process,
    cpu: procfs::CpuInfo,

    memory: Memory,
    name: String,
    state: String,
    n_threads: usize
}

impl Watcher {
    pub fn new(op_pid: Option<pid_t>) -> Watcher {
        let proc = match op_pid{
            None => { procfs::process::Process::myself().expect("Unable to load myself!") }
            Some(pid) => { procfs::process::Process::new(pid).expect(&*format!("Unable to load {}!", pid)) }
        };
        Watcher{
            proc,
            cpu: procfs::CpuInfo::new().expect("Unable to load cpu_info"),
            memory: Memory {
                total: 0,
                free: 0,
                available: 0,
                used: 0,
                process_vm: 0,
                process_vm_peak: 0,
                process_rss: 0,
                process_hwm: 0
            },
            name: "".to_string(),
            state: "".to_string(),
            n_threads: 0
        }
    }

    pub fn look(&mut self) {
        let mem = procfs::Meminfo::new().expect("Unable to load mem_info");
        self.memory.total = mem.mem_total as usize / 1048576;
        self.memory.free  = mem.mem_free as usize / 1048576;
        self.memory.used = (mem.mem_total - mem.mem_free - mem.cached - mem.buffers - mem.s_reclaimable.unwrap_or(0)  + mem.shmem.unwrap_or(0)) as usize/ 1048576;
        self.memory.available = mem.mem_available.unwrap_or(0) as usize / 1048576;


        match self.proc.status() {
            Ok(status) => {
                self.memory.process_vm = status.vmsize.unwrap_or(0) as usize;
                self.memory.process_rss = status.vmrss.unwrap_or(0) as usize;
                self.memory.process_vm_peak = status.vmpeak.unwrap_or(0) as usize;
                self.memory.process_hwm = status.vmhwm.unwrap_or(0) as usize;
                
                self.state = status.state;
                self.name = status.name;
                self.n_threads = status.threads as usize;
            }
            Err(_) => {}
        }
    }

    pub fn pretty_print(&self) {
        println!("==========Resources usage output:==============");
        println!("Name:         {}", self.name);
        println!("State:        {}", self.state);
        println!("Proc_VM:      {} [KiB]", self.memory.process_vm);
        println!("Proc_VM_PEAK: {} [KiB]", self.memory.process_vm_peak);
        println!("Proc_RSS:     {} [KiB]", self.memory.process_rss);
        println!("Proc_HWM:     {} [KiB]", self.memory.process_hwm);
        println!("Mem_total:    {} [MiB]", self.memory.total);
        println!("Mem_used:     {} [MiB]", self.memory.used);
        println!("Mem_free:     {} [MiB]", self.memory.free);
        println!("Mem_avail:    {} [MiB]", self.memory.available);
        println!("Threads_N:    {}", self.n_threads);
        println!("===============================================");
    }
}