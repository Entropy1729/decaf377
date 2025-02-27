use std::hash::Hash;

use ark_ed_on_bls12_377::EdwardsAffine;
use ark_ff::Zero;
use ark_std::fmt::{Display, Formatter, Result as FmtResult};

use zeroize::Zeroize;

use crate::Element;

#[derive(Copy, Clone)]
pub struct AffineElement {
    pub(crate) inner: EdwardsAffine,
}

impl Hash for AffineElement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl Default for AffineElement {
    fn default() -> Self {
        Element::default().into()
    }
}

impl Zero for AffineElement {
    fn zero() -> Self {
        Self::default()
    }

    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }
}

impl core::iter::Sum<Self> for AffineElement {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), core::ops::Add::add)
    }
}

impl<'a> core::iter::Sum<&'a AffineElement> for AffineElement {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), core::ops::Add::add)
    }
}

impl PartialEq for AffineElement {
    fn eq(&self, other: &AffineElement) -> bool {
        // Section 4.5 of Decaf paper
        self.inner.x * other.inner.y == self.inner.y * other.inner.x
    }
}

impl Eq for AffineElement {}

impl std::fmt::Debug for AffineElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let element: Element = self.into();
        f.write_fmt(format_args!(
            "decaf377::AffineElement({})",
            hex::encode(&element.vartime_compress().0[..])
        ))
    }
}

impl Display for AffineElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let element: Element = self.into();
        write!(
            f,
            "decaf377::AffineElement({})",
            hex::encode(&element.vartime_compress().0[..])
        )
    }
}

impl Zeroize for AffineElement {
    fn zeroize(&mut self) {
        self.inner.zeroize()
    }
}
