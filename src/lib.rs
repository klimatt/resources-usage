
use std::os::unix::raw::pid_t;
use procinfo as proc;
use sysinfo::SystemExt;
use std::process;

pub struct RUsage{
    pid: i32,
    system: sysinfo::System,
    vm_peak: usize,
    vm_size: usize,
    vm_locked: usize,
    vm_hwm: usize,
    vm_rss: usize,
    vm_swap: usize,
    n_threads: u32,

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
            system: sysinfo::System::new(),
            vm_peak: 0,
            vm_size: 0,
            vm_locked: 0,
            vm_hwm: 0,
            vm_rss: 0,
            n_threads: 0,
            cnt: 1
        }
    }

    pub fn calculate(&mut self) {
        let status = proc::pid::status(self.pid).unwrap();
        self.system.refresh_system();

        self.n_threads = status.threads;
        self.
    }

}





#[cfg(test)]
mod tests {

}
