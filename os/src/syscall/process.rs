//! Process management syscalls
#![allow(unused_imports)] // 这将允许未使用的导入
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,
        TaskStatus, current_user_token,now_task_time,get_syscall_count,get_task_time,
        TASK_MANAGER,mmap, munmap,
    },
    timer::{get_time_us, get_time_ms},
    mm::{
        page_table::{translated_byte_buffer,PageTable,PTEFlags},
        address::{VirtAddr,SimpleRange,VPNRange,VirtPageNum},
        frame_allocator::frame_alloc
    },
    syscall::SYSCALL_TASK_INFO,
    syscall::SYSCALL_YIELD,
    syscall::SYSCALL_GETTIMEOFDAY,
};

/// 表示时间的结构体。
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// 表示秒数。
    pub sec: usize,
    /// 表示微秒数（1秒 = 1,000,000微秒）。
    pub usec: usize,
}


impl TimeVal {
    /// 将TimeVal转换为字节数组
    pub fn to_bytes(&self) -> [u8; 2 * core::mem::size_of::<usize>()] {
        let mut bytes = [0u8; 2 * core::mem::size_of::<usize>()];
        // 将 sec 和 usec 转换为字节数组
        let sec_bytes = self.usize_to_bytes(self.sec);
        let usec_bytes = self.usize_to_bytes(self.usec);
        
        let usize_size = core::mem::size_of::<usize>();
        // 将 sec 和 usec 的字节值填充到 bytes 数组中
        for i in 0..usize_size {
            bytes[i] = sec_bytes[i];
            bytes[i + usize_size] = usec_bytes[i];
        }
        
        bytes
    }

    /// 将usize值转换为字节数组
    fn usize_to_bytes(&self, val: usize) -> [u8; core::mem::size_of::<usize>()] {
        let mut arr = [0u8; core::mem::size_of::<usize>()];
        // 将 usize 值转换为字节数组
        for i in 0..core::mem::size_of::<usize>() {
            arr[i] = ((val >> (i * 8)) & 0xFF) as u8;
        }
        arr
    }
    
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

impl TaskInfo {
    /// 创建一个新的TaskInfo实例
    pub fn new(status: TaskStatus, syscall_times: [u32; MAX_SYSCALL_NUM], time: usize) -> Self {
        TaskInfo {
            status,
            syscall_times,
            time,
        }
    }
    /// 将TaskInfo转换为字节数组
    pub fn to_bytes(&self) -> [u8; core::mem::size_of::<TaskInfo>()] {
        let ptr = self as *const _ as *const u8;
        let mut bytes = [0u8; core::mem::size_of::<TaskInfo>()];
        unsafe {
            for i in 0..core::mem::size_of::<TaskInfo>() {
                bytes[i] = *ptr.add(i);
            }
        }
        bytes
    }

    /// 从字节数组中创建TaskInfo
    pub fn from_bytes(bytes: &[u8; core::mem::size_of::<TaskInfo>()]) -> TaskInfo {
        unsafe {
            core::mem::transmute::<[u8; core::mem::size_of::<TaskInfo>()], TaskInfo>(*bytes)
        }
    }
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    
    // 获取 TASK_MANAGER 的独占访问权
    let mut task_manager_inner = TASK_MANAGER.inner.exclusive_access();
    // 获取当前任务的索引
    let current_task_index = task_manager_inner.current_task;
    // 获取当前任务的可变引用
    let current_task = &mut task_manager_inner.tasks[current_task_index];

    // 增加当前任务的系统调用计数
    // 注意：这里假设您有一个名为 `SYSCALL_YIELD` 的常量来表示 sys_yield 调用
    current_task.increase_syscall_count(SYSCALL_YIELD);

    // 释放对 TASK_MANAGER 的独占访问权
    drop(task_manager_inner);

    // 挂起当前任务并运行下一个任务
    suspend_current_and_run_next();
    
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    // 获取 TASK_MANAGER 的独占访问权
    let mut task_manager_inner = TASK_MANAGER.inner.exclusive_access();
    // 获取当前任务的索引
    let current_task_index = task_manager_inner.current_task;
    // 获取当前任务的可变引用
    let current_task = &mut task_manager_inner.tasks[current_task_index];

    // 增加对应的系统调用计数
    current_task.increase_syscall_count(SYSCALL_GETTIMEOFDAY);

    // 释放对 TASK_MANAGER 的独占访问权
    drop(task_manager_inner);
    
    // 获取当前时间（微秒）
    let us = get_time_us();
    // 创建 TimeVal 结构体
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    // 将 TimeVal 结构体转换为字节数组
    let serialized = time_val.to_bytes();
    // 将用户空间的指针转换为内核空间的字节缓冲区
    let mut buffers = translated_byte_buffer(current_user_token(), ts as *const u8, serialized.len());
    // 将 serialized 的内容复制到用户空间
    // 手动复制 serialized 的内容到用户空间
    for i in 0..buffers.len() {
        let buffer = &mut buffers[i];
        for j in 0..buffer.len() {
            buffer[j] = serialized[i * buffer.len() + j];
        }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    // 获取 TASK_MANAGER 的独占访问权
    let mut task_manager_inner = TASK_MANAGER.inner.exclusive_access();
    // 获取当前任务的索引
    let current_task_index = task_manager_inner.current_task;
    // 获取当前任务的可变引用
    let current_task = &mut task_manager_inner.tasks[current_task_index];

    // 现在可以在 current_task 上调用方法来增加系统调用计数
    current_task.increase_syscall_count(SYSCALL_TASK_INFO);

    // 释放对 TASK_MANAGER 的独占访问权
    drop(task_manager_inner);

    // 获取当前任务的信息
    let syscall_times = get_syscall_count();
    let _time = now_task_time() - get_task_time();
    let task_info = TaskInfo::new(TaskStatus::Running, syscall_times, get_time_ms());
    
    let serialized = task_info.to_bytes();

    let mut buffers = translated_byte_buffer(current_user_token(), ti as *const u8, serialized.len());

    for (i, buffer) in buffers.iter_mut().enumerate() {
        let buffer_len = buffer.len();
        for (j, byte) in buffer.iter_mut().enumerate() {
            *byte = serialized[i * buffer_len + j];
        }
    }    
    0
}

/// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!"); 
    // 将起始地址转换为虚拟地址
    let start_vaddr: VirtAddr = start.into();
    // 检查起始地址是否已对齐
    if !start_vaddr.aligned() {
        debug!("映射失败：起始地址未对齐");
        return -1;
    }
    // 检查port的有效性
    if port & !0x7 != 0 || port & 0x7 == 0 {
        return -1;
    }
    // 如果长度为0，则直接返回
    if len == 0 {
        return 0;
    }
    // 计算结束地址
    let end_vaddr: VirtAddr = (start + len).into();
    let start_vpn: VirtPageNum = start_vaddr.into();
    let end_vpn: VirtPageNum = (end_vaddr).ceil();
    
    mmap(start_vpn, end_vpn, port)
    
}

// YOUR JOB: Implement mmap.
// pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
//     trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
//     // 在函数内部创建PageTable实例
//     let mut page_table = PageTable::new();
//     // 将起始地址转换为虚拟地址
//     let start_vaddr: VirtAddr = start.into();
//     // 检查起始地址是否已对齐
//     if !start_vaddr.aligned() {
//         debug!("映射失败：起始地址未对齐");
//         return -1;
//     }
//     // 检查port的有效性
//     if port & !0x7 != 0 || port & 0x7 == 0 {
//         return -1;
//     }
//     // 如果长度为0，则直接返回
//     if len == 0 {
//         return 0;
//     }
//     // 计算结束地址
//     let end_vaddr: VirtAddr = (start + len).into();
//     let start_vpn: VirtPageNum = start_vaddr.into();
//     let end_vpn: VirtPageNum = (end_vaddr).ceil();
//     // 根据port设置页表条目标志
//     let mut flags = PTEFlags::V; // V表示有效
//     if port & 0x1 == 0x1 {
//         flags |= PTEFlags::R; // R表示可读
//     }
//     if port & 0x2 == 0x2 {
//         flags |= PTEFlags::W; // W表示可写
//     }
//     if port & 0x4 == 0x4 {
//         flags |= PTEFlags::X; // X表示可执行
//     }
//     // 遍历虚拟页号范围，并进行映射
//     for vpn in start_vpn..end_vpn {
//         // 使用translate方法获取物理页号
//         if let Some(pte) = page_table.translate(vpn) {
//             let ppn = pte.ppn();
//             page_table.map(vpn, ppn, flags);
//         } else {
//             // 如果没有找到对应的物理页号，分配一个新的物理页
//             let frame_tracker = frame_alloc().expect("Failed to allocate a new frame");
//             let new_ppn = frame_tracker.ppn;
//             page_table.map(vpn, new_ppn, flags);
//         }
//     }
//     0
// }


/// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
     // 将起始地址转换为虚拟地址
     let start_vaddr: VirtAddr = _start.into();
     // 检查起始地址是否已对齐
     if !start_vaddr.aligned() {
         debug!("映射失败：起始地址未对齐");
         return -1;
     }
     // 计算结束地址
     let end_vaddr: VirtAddr = (_start + _len).into();
     let start_vpn: VirtPageNum = start_vaddr.into();
     let end_vpn: VirtPageNum = (end_vaddr).ceil();
    munmap(start_vpn, end_vpn)
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
