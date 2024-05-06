use core::ops::{Deref, DerefMut};

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
