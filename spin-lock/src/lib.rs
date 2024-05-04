#![no_std]

use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

pub mod raw {
    use core::sync::atomic::{fence, AtomicBool, Ordering::*};
    pub struct SpinLock {
        locked: AtomicBool,
    }

    impl SpinLock {
        pub const fn new() -> Self {
            Self {
                locked: AtomicBool::new(false),
            }
        }

        pub fn lock(&self) {
            while self.locked.swap(true, Relaxed) {
                core::hint::spin_loop();
            }
            fence(Acquire);
        }

        pub fn unlock(&self) {
            self.locked.store(false, Release);
        }
    }

    impl Default for SpinLock {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub struct SpinLock<T: ?Sized> {
    lock: raw::SpinLock,
    inner: UnsafeCell<T>,
}

unsafe impl<T: ?Sized> Send for SpinLock<T> where T: Send {}
unsafe impl<T: ?Sized> Sync for SpinLock<T> where T: Send {}

pub struct SpinLockGuard<'a, T: ?Sized> {
    locked: &'a SpinLock<T>,
}

impl<T: ?Sized> Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.locked.inner.get() }
    }
}

impl<T: ?Sized> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.locked.inner.get() }
    }
}

// unsafe impl<T: ?Sized> Send for SpinLockGuard<'_, T> where T: Send {}
unsafe impl<T: ?Sized> Sync for SpinLockGuard<'_, T> where T: Sync {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            lock: raw::SpinLock::new(),
            inner: UnsafeCell::new(value),
        }
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T: ?Sized> SpinLock<T> {
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        self.lock.lock();
        SpinLockGuard { locked: self }
    }
}

impl<T: ?Sized> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.locked.lock.unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
