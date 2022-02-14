use std::os::unix::raw::pid_t;
use procfs;
use procfs::ProcResult;

pub struct Memory{
    total: usize,
    free: usize,
    available: usize,
    used: usize,
    process_vm: usize,
    process_rss: usize
}

pub struct Watcher{
    proc: procfs::process::Process,
    cpu: procfs::CpuInfo,

    memory: Memory,
    name: String,
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
                process_rss: 0
            },
            name: "".to_string(),
            n_threads: 0
        }
    }

    pub fn look(&mut self) {
        let mem = procfs::Meminfo::new().expect("Unable to load mem_info");
        self.memory.total = mem.mem_total as usize / 1048576;
        self.memory.free  = mem.mem_free as usize / 1048576;
        self.memory.used = (mem.mem_total - mem.mem_free - mem.buffers - mem.cached - mem.s_reclaimable.unwrap_or(0)) as usize/ 1048576;
        self.memory.available = mem.mem_available.unwrap_or(0) as usize / 1048576;

        match self.proc.stat() {
            Ok(stat) => {
                self.memory.process_vm = stat.vsize as usize;
                self.memory.process_rss = stat.rss as usize;
                self.name = stat.comm;
                self.n_threads = stat.num_threads as usize;
            }
            Err(_) => {}
        }
    }

    pub fn pretty_print(&self) {
        println!("==========Resources usage output:==============");
        println!("Name:       {}", self.name);
        println!("Proc_VM:    {} [KiB]",  self.memory.process_vm / 1024);
        println!("Proc_RSS:   {} [pages]", self.memory.process_rss);
        println!("Mem_total:  {} [MiB]", self.memory.total);
        println!("Mem_used:   {} [MiB]", self.memory.used);
        println!("Mem_free:   {} [MiB]", self.memory.free);
        println!("Mem_avail:  {} [MiB]", self.memory.available);
        println!("Threads_N:  {}", self.n_threads);
        println!("===============================================");
    }
}