//! App management syscalls
use crate::{batch::run_next_app, debug};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    debug!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}