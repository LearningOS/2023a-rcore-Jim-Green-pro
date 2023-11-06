//! File and filesystem-related syscalls

use crate::mm::translated_byte_buffer;
use crate::task::current_user_token;


use crate::{
    task::{
        TASK_MANAGER,
    },
    syscall::SYSCALL_WRITE,
};

const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel: sys_write");
    // 获取当前任务的可变引用，并增加系统调用计数
    let mut task_manager_inner = TASK_MANAGER.inner.exclusive_access();
    let current_task_index = task_manager_inner.current_task;
    let current_task = &mut task_manager_inner.tasks[current_task_index];
    current_task.increase_syscall_count(SYSCALL_WRITE);
    // 释放对 TASK_MANAGER 的独占访问权
    drop(task_manager_inner);

    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(current_user_token(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
