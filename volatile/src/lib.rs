#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Volatile<T>(T);

impl<T: Copy> Volatile<T> {
    pub fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(&self.0) }
    }

    pub fn write(&mut self, value: T) {
        unsafe {
            core::ptr::write_volatile(&mut self.0, value);
        }
    }
}
