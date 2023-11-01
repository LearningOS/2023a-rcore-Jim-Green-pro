//! Uniprocessor interior mutability primitives
use core::cell::{RefCell, RefMut};
use core::sync::atomic::{AtomicBool, Ordering};
/// Wrap a static data structure inside it so that we are
/// able to access it without any `unsafe`.
///
/// We should only use it in uniprocessor.
///
/// In order to get mutable reference of inner data, call
/// `exclusive_access`.

/// 一个简单的自旋锁实现。
struct Spinlock {
    lock: AtomicBool,
}

impl Spinlock {
    const fn new() -> Self {
        Spinlock {
            lock: AtomicBool::new(false),
        }
    }

    /// 尝试获取锁，如果锁已被占用则自旋直到获取。
    fn lock(&self) {
        while self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {}

    }

    /// 释放锁。
    fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}
/// 使用自旋锁保护的单处理器内部可变性原语。
pub struct UPSafeCell<T> {
    /// inner data
    inner: RefCell<T>,
    lock: Spinlock,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in
    /// uniprocessor.
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
            lock: Spinlock::new(),
        }
    }
    /// Panic if the data has been borrowed.
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.lock.lock();
        let result = self.inner.borrow_mut();
        self.lock.unlock();
        result
    }
}
