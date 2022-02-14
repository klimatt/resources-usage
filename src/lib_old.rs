use std::collections::HashMap;
use std::os::unix::raw::pid_t;
use std::process;
use std::fs;
pub use procfs;

pub struct RUsage{
    pid: i32,
    vm_peak: String,
    vm_size: String,
    vm_locked: String,
    vm_hwm: String,
    vm_rss: String,
    vm_swap: String,
    n_threads: String,

    cmd: String,
    cnt: u32,
}

impl RUsage {
    pub fn new(pid: Option<i32>) -> RUsage{
        let pid = match pid{
            None => { process::id() as i32 }
            Some(pid) => { pid }
        };
        RUsage{
            pid,
            vm_peak: "".to_string(),
            vm_size: "".to_string(),
            vm_locked: "".to_string(),
            vm_hwm: "".to_string(),
            vm_rss: "".to_string(),
            vm_swap: "".to_string(),
            n_threads: "".to_string(),
            cmd: "".to_string(),
            cnt: 1
        }
    }

    pub fn refresh(&mut self) {
        let str = fs::read_to_string(format!("/proc/{}/status", self.pid)).unwrap();
        let hmap = str
            .lines()
            .map(|s|
                s.split(":\t"))
            .map(|mut s| (s.next().unwrap(), s.next().unwrap())).collect::<HashMap<&str, &str>>();

        self.cmd = hmap.get("Name").unwrap().to_string();
        self.vm_peak = hmap.get("VmPeak").unwrap().to_string();
        self.vm_size = hmap.get("VmSize").unwrap().to_string();
        self.vm_hwm = hmap.get("VmHWM").unwrap().to_string();
        self.vm_swap = hmap.get("VmSwap").unwrap().to_string();
        self.vm_locked = hmap.get("VmLck").unwrap().to_string();
        self.vm_rss = hmap.get("VmRSS").unwrap().to_string();
        self.n_threads = hmap.get("Threads").unwrap().to_string();

    }

    pub fn pretty_print(&self) {
        println!("==========Resources usage output:==============");
        println!("Name:       {}", self.cmd);
        println!("Vm_Peak:    {}", self.vm_peak);
        println!("Vm_Size:    {}", self.vm_size);
        println!("Vm_RSS:     {}", self.vm_rss);
        println!("Vm_HWM:     {}", self.vm_hwm);
        println!("Vm_Swap:    {}", self.vm_swap);
        println!("Vm_Locked:  {}", self.vm_locked);
        println!();
        println!("Threads_N:  {}", self.n_threads);
        println!("===============================================");
    }

}

