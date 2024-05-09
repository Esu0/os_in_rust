use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

use crate::{SpinLock, SpinLockGuard};

pub struct OnceMutex<T> {
    inner: SpinLock<Option<T>>,
}

pub struct OnceMutexGuard<'a, T> {
    guard: SpinLockGuard<'a, Option<T>>,
}

impl<T> Deref for OnceMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.guard.as_ref().unwrap_unchecked() }
    }
}

impl<T> DerefMut for OnceMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.guard.as_mut().unwrap_unchecked() }
    }
}

impl<T> OnceMutex<T> {
    pub const fn new() -> Self {
        Self {
            inner: SpinLock::new(None),
        }
    }

    pub fn lock_or_init<F>(&self, init: F) -> OnceMutexGuard<'_, T>
    where
        F: FnOnce() -> T,
    {
        let mut guard = self.inner.lock();
        if guard.is_none() {
            *guard = Some(init());
        }
        OnceMutexGuard { guard }
    }

    pub fn lock(&self) -> OnceMutexGuard<'_, T>
    where
        T: Default,
    {
        self.lock_or_init(Default::default)
    }
}

impl<T> Default for OnceMutex<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Lazy<T, F> {
    lock: SpinLock<Option<T>>,
    init: UnsafeCell<MaybeUninit<F>>,
}

unsafe impl<T, F> Send for Lazy<T, F>
where
    T: Send,
    F: Send,
{
}
unsafe impl<T, F> Sync for Lazy<T, F>
where
    T: Send,
    F: Send,
{
}

pub struct LazyGuard<'a, T>(SpinLockGuard<'a, Option<T>>);

impl<T> Deref for LazyGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref().unwrap_unchecked() }
    }
}

impl<T> DerefMut for LazyGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut().unwrap_unchecked() }
    }
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub const fn new(init: F) -> Self {
        Self {
            lock: SpinLock::new(None),
            init: UnsafeCell::new(MaybeUninit::new(init)),
        }
    }

    pub fn lock(&self) -> LazyGuard<'_, T> {
        let mut guard = self.lock.lock();
        if guard.is_none() {
            let init = unsafe { (*self.init.get()).assume_init_read() };
            *guard = Some(init());
        }
        LazyGuard(guard)
    }
}

impl<T, F> Drop for Lazy<T, F> {
    fn drop(&mut self) {
        let guard = self.lock.lock();
        if guard.is_none() {
            unsafe { self.init.get_mut().assume_init_drop() };
        }
    }
}
