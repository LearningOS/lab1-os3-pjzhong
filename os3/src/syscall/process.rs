use crate::task::exit_cuurent_and_run_next;
use crate::task::suspend_cuurent_and_run_next;

pub fn sys_yield() -> isize {
    suspend_cuurent_and_run_next();
    0
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_cuurent_and_run_next();
    panic!("Unreachable in sys_exit!");
}
