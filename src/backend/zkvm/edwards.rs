//! Edwards arithmetic that exports the elliptic curve operations to a host environment. This
//! allows different implementations of a zero-knowledge virtual machine to choose the course
//! of action that best suits their needs, e.g. using a dedicated circuit for the elliptic curve
//! operations, or using a native implementation of the curve.
//!
//! # Point representation
//! As the inversion operation is considered inepensive in the context of zk-SNARKs, we choose to
//! represent points as affine coordinates, i.e. as a pair of field elements $(x, y)$.

use core::convert::TryInto;

use crate::edwards;

use super::field::FieldElemetLimbs32;

extern "C" {
    fn syscall_ed_add(p: *mut u32, q: *const u32);
}

/// An affine point on the Edwards curve.
///
/// The point is represented internally by bytes in order to ensure a contiguous memory layout.
#[derive(Copy, Clone, Debug)]
pub struct AffinePoint {
    limbs: [u32; 16],
}

impl AffinePoint {
    /// Get the x-coordinate of the point.
    #[inline]
    pub fn x(&self) -> FieldElemetLimbs32 {
        self.limbs[..8].try_into().unwrap()
    }

    /// Get the y-coordinate of the point.
    #[inline]
    pub fn y(&self) -> FieldElemetLimbs32 {
        self.limbs[8..].try_into().unwrap()
    }
}

impl From<edwards::EdwardsPoint> for AffinePoint {
    fn from(value: edwards::EdwardsPoint) -> Self {
        let mut limbs = [0u32; 16];

        todo!()
    }
}
