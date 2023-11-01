//! Types related to task management

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
<<<<<<< HEAD
    /// 任务所调用的系统调用的次数
    pub syscall_times: [u32; super::MAX_SYSCALL_NUM],
    /// 任务的开始时间
    pub start_time: usize,
=======
>>>>>>> dd1707305386a03ef6edce6d662ff8681d092d9e
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
