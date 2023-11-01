//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

<<<<<<< HEAD
use crate::task::TASK_MANAGER;

=======
<<<<<<< HEAD
use crate::task::TASK_MANAGER; // 导入 TASK_MANAGER

=======
>>>>>>> dd1707305386a03ef6edce6d662ff8681d092d9e
>>>>>>> 757505691864657d732af58dd2d73c55a76c99eb
/// write syscall
const SYSCALL_WRITE: usize = 64;
/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GET_TIME: usize = 169;
/// taskinfo syscall
const SYSCALL_TASK_INFO: usize = 410;

mod fs;
mod process;

use fs::*;
use process::*;
/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
<<<<<<< HEAD
    // 更新系统调用计数
    {
        let inner = TASK_MANAGER.inner.exclusive_access();
        inner.tasks[inner.current_task].syscall_times[syscall_id] += 1;
    }

=======
<<<<<<< HEAD
    // 在处理系统调用之前，更新当前任务的系统调用计数
    let current_task = &mut TASK_MANAGER.inner.exclusive_access().tasks[TASK_MANAGER.inner.exclusive_access().current_task];
    if syscall_id < current_task.syscall_times.len() {
        current_task.syscall_times[syscall_id] += 1;
    }

=======
>>>>>>> dd1707305386a03ef6edce6d662ff8681d092d9e
>>>>>>> 757505691864657d732af58dd2d73c55a76c99eb
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TASK_INFO => sys_task_info(args[0] as *mut TaskInfo),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
