use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy)]
pub struct Hex<const N: usize>(pub [u8; N]);
impl<const N: usize> Debug for Hex<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;
        for (i, b) in self.0.iter().enumerate() {
            if i + 1 != self.0.len() {
                write!(f, "{b:02x} ")?;
            } else {
                write!(f, "{b:02x}")?;
            }
        }
        write!(f, ">")?;
        Ok(())
    }
}
impl<const N: usize> Deref for Hex<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const N: usize> DerefMut for Hex<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
