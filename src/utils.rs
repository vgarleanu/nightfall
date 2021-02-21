cfg_if::cfg_if! {
    if #[cfg(not(target_os = "windows"))] {
        use nix::sys::signal;
        use nix::sys::signal::Signal;
        use nix::unistd::Pid;

        pub fn pause_proc(pid: i32) {
            signal::kill(Pid::from_raw(pid), Signal::SIGSTOP);
        }

        pub fn cont_proc(pid: i32) {
            signal::kill(Pid::from_raw(pid), Signal::SIGCONT);
        }
    } else {
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::winnt::HANDLE;
        use ntapi::ntpsapi::NtSuspendProcess;

        pub fn pause_proc(pid: i32) {
            unsafe {
                let process_handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid as u32);
                NtSuspendProcess(process_handle)
            }
        }

        pub fn cont_proc(pid: i32) {
            unsafe {
                let process_handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid as u32);
                NtResumeProcess(process_handle)
            }
        }
    }
}