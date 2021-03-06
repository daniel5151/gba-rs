use std::fmt;
use std::ops::{Deref, DerefMut};
use std::ptr;

/// Class to help with the weak references between singletons in the GBA
/// structure.  Totally throwing safety out the window here.
pub struct Shared<T> {
    t: *mut T,
}

impl<T> Copy for Shared<T> {}
impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Shared<T> {
    pub fn empty() -> Self {
        Shared { t: ptr::null_mut() }
    }

    pub fn new(val: &mut T) -> Self {
        Shared { t: val as *mut T }
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;

    fn deref(&self) -> &T {
        if self.t.is_null() {
            unreachable!("Dereferencing uninitialized shared");
        }
        unsafe { &(*self.t) }
    }
}

impl<T> DerefMut for Shared<T> {
    fn deref_mut(&mut self) -> &mut T {
        if self.t.is_null() {
            panic!("Dereferencing uninitialized shared");
        }
        unsafe { &mut (*self.t) }
    }
}

impl<T: fmt::Debug> fmt::Debug for Shared<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut dbg = fmt.debug_struct("Shared");
        let ndbg = dbg.field("t: ", &self.t);
        if !self.t.is_null() {
            ndbg.field("data: ", self.deref()).finish()
        } else {
            ndbg.finish()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Shared;

    #[test]
    fn test_shared() {
        let mut a = 5;
        let mut b = Shared::new(&mut a);

        (*b) = 3;

        assert_eq!(a, 3);
    }

    #[test]
    #[should_panic(expected = "Dereferencing uninitialized shared")]
    fn test_uninitialized() {
        let b: Shared<i32> = Shared::empty();

        assert_eq!(*b, 3);
    }
}
