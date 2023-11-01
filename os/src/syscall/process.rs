//! Process management syscalls
<<<<<<< HEAD

use crate::task::{TASK_MANAGER, TaskControlBlock};

=======
>>>>>>> dd1707305386a03ef6edce6d662ff8681d092d9e
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
<<<<<<< HEAD
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
=======
<<<<<<< HEAD
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");

    // 获取当前任务的TaskControlBlock
    let tcb = {
        let inner = TASK_MANAGER.inner.exclusive_access();
        &inner.tasks[inner.current_task]
    };

    // 检查指针的有效性
    if ti.is_null() {
        return -1;
    }

    // 安全地填充TaskInfo结构体
    unsafe {
        (*ti).status = tcb.task_status;
        (*ti).syscall_times = tcb.syscall_times;
        (*ti).time = get_time_us() / 1_000 - tcb.start_time; // 假设get_time_us返回的是微秒
    }

    0
}

=======
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
>>>>>>> 757505691864657d732af58dd2d73c55a76c99eb
    trace!("kernel: sys_task_info");

    // 获取当前任务的TaskControlBlock
    let tcb = {
        let inner = TASK_MANAGER.inner.exclusive_access();
        &inner.tasks[inner.current_task]
    };

    // 检查指针的有效性
    if ti.is_null() {
        return -1;
    }

    // 安全地填充TaskInfo结构体
    unsafe {
        (*ti).status = tcb.task_status;
        (*ti).syscall_times = tcb.syscall_times;
        (*ti).time = get_time_us() / 1_000 - tcb.start_time; // 假设get_time_us返回的是微秒
    }

    0
}
>>>>>>> dd1707305386a03ef6edce6d662ff8681d092d9e
