use std::ops::{Deref, DerefMut};
pub use super::node::{InnerNode as InnerNodeActual, LeafNode as LeafNodeActual};

// no packed enums and no way to force lower alignment -> need ugly hacks
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Unalign<T>(T);

impl<T> Deref for Unalign<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Unalign<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> From<T> for Unalign<T> {
    fn from(t: T) -> Unalign<T> {
        Unalign(t)
    }
}

pub type InnerNode = Unalign<InnerNodeActual>;
pub type LeafNode = Unalign<LeafNodeActual>;
