//! Edwards arithmetic that exports the elliptic curve operations to a host environment. This
//! allows different implementations of a zero-knowledge virtual machine to choose the course
//! of action that best suits their needs, e.g. using a dedicated circuit for the elliptic curve
//! operations, or using a native implementation of the curve.
//!
//! # Point representation
//! As the inversion operation is considered inepensive in the context of zk-SNARKs, we choose to
//! represent points as affine coordinates, i.e. as a pair of field elements $(x, y)$.

use core::{convert::TryInto, ops::AddAssign};

use crate::{edwards::EdwardsPoint, field::FieldElement};

use super::{constants, field::FieldElemetLimbs32};

use traits::Identity;

extern "C" {
    /// Add-assign `P += Q` two affine points with given raw slice pointers 'p' and 'q'.
    fn syscall_ed_add(p: *mut u32, q: *const u32);
}

/// An affine point on the Edwards curve.
///
/// The point is represented internally by bytes in order to ensure a contiguous memory layout.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AffinePoint {
    limbs: [u32; 16],
}

impl AffinePoint {
    pub const fn from_limbs(limbs: [u32; 16]) -> Self {
        Self { limbs }
    }
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

    pub fn ed_add_assign(&mut self, other: &AffinePoint) {
        unsafe {
            syscall_ed_add(self.limbs.as_mut_ptr(), other.limbs.as_ptr());
        }
    }

    pub fn double(&mut self) {
        unsafe {
            syscall_ed_add(self.limbs.as_mut_ptr(), self.limbs.as_ptr());
        }
    }

    pub fn mul_by_pow_2(&self, k: u32) -> Self {
        let mut tmp: AffinePoint = *self;
        for _ in 0..k {
            tmp.double();
        }
        tmp
    }
}

impl From<EdwardsPoint> for AffinePoint {
    fn from(value: EdwardsPoint) -> Self {
        let mut limbs = [0u32; 16];

        assert_eq!(value.Z, FieldElement::one());

        for (x_limb, x_bytes) in limbs[..8]
            .iter_mut()
            .zip(value.X.to_bytes().chunks_exact(4))
        {
            *x_limb = u32::from_le_bytes(x_bytes.try_into().unwrap());
        }

        for (y_limb, y_bytes) in limbs[8..]
            .iter_mut()
            .zip(value.Y.to_bytes().chunks_exact(4))
        {
            *y_limb = u32::from_le_bytes(y_bytes.try_into().unwrap());
        }

        Self { limbs }
    }
}

impl From<AffinePoint> for EdwardsPoint {
    #[inline]
    #[allow(non_snake_case)]
    fn from(value: AffinePoint) -> Self {
        let X = FieldElement::from(value.x());
        let Y = FieldElement::from(value.y());
        let Z = FieldElement::one();
        let T = &X * &Y;

        Self { X, Y, Z, T }
    }
}

impl Identity for AffinePoint {
    #[inline]
    fn identity() -> AffinePoint {
        constants::IDENTITY
    }
}

impl<'a> AddAssign for &'a mut AffinePoint {
    fn add_assign(&mut self, rhs: Self) {
        self.ed_add_assign(rhs);
    }
}

impl<'a> AddAssign<&'a AffinePoint> for &'a mut AffinePoint {
    fn add_assign(&mut self, rhs: &'a AffinePoint) {
        self.ed_add_assign(rhs);
    }
}

impl AddAssign for AffinePoint {
    fn add_assign(&mut self, rhs: Self) {
        self.ed_add_assign(&rhs);
    }
}

impl AddAssign<&AffinePoint> for AffinePoint {
    fn add_assign(&mut self, rhs: &AffinePoint) {
        self.ed_add_assign(rhs);
    }
}

#[allow(non_snake_case)]
pub fn normalize(p: &EdwardsPoint) -> EdwardsPoint {
    let EdwardsPoint { X, Y, Z, T } = p;

    let Z_inv = Z.invert();

    EdwardsPoint {
        X: X * &Z_inv,
        Y: Y * &Z_inv,
        Z: FieldElement::one(),
        T: T * &Z_inv,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::{
        backend::{self, serial::u32::constants::ED25519_BASEPOINT_POINT},
        scalar::Scalar,
    };

    use super::*;

    #[no_mangle]
    extern "C" fn syscall_ed_add(p: *mut u32, q: *const u32) {
        let p_affine = AffinePoint::from_limbs(unsafe {
            core::slice::from_raw_parts_mut(p, 16).try_into().unwrap()
        });
        let q_affine = AffinePoint::from_limbs(unsafe {
            core::slice::from_raw_parts(q, 16).try_into().unwrap()
        });

        let p_edwards = EdwardsPoint::from(p_affine);
        let q_edwards = EdwardsPoint::from(q_affine);
        let p_plus_q = AffinePoint::from(normalize(&(p_edwards + q_edwards)));

        let limbs: &mut [u32] =
            unsafe { core::slice::from_raw_parts_mut(p, 16).try_into().unwrap() };
        limbs.copy_from_slice(&p_plus_q.limbs);
    }

    // Computes `scalar * p` using the serial backend and normalizes the `Z` coordinate.
    pub fn serial_scalar_mul(p: &EdwardsPoint, scalar: &Scalar) -> EdwardsPoint {
        normalize(&backend::serial::scalar_mul::variable_base::mul(p, scalar))
    }

    #[test]
    fn test_affine_edwards_conversion() {
        let mut rng = rand::thread_rng();
        let num_iters = 100;

        // Test the identity conversion.
        let identity = EdwardsPoint::identity();
        let identity_affine = AffinePoint::from(identity);
        assert_eq!(identity_affine, AffinePoint::identity());

        let identity_affine = AffinePoint::identity();
        let identity_back = EdwardsPoint::from(identity_affine);
        assert_eq!(identity_back, EdwardsPoint::identity());

        // Test convertions are inverses of each other.
        let base = ED25519_BASEPOINT_POINT;
        let bas_affine = AffinePoint::from(base);
        let base_back = EdwardsPoint::from(bas_affine);
        assert_eq!(base, base_back);
        for _ in 0..num_iters {
            let scalar = Scalar::random(&mut rng);
            let p = serial_scalar_mul(&base, &scalar);
            let p_affine = AffinePoint::from(p);
            let p_back = EdwardsPoint::from(p_affine);
            assert_eq!(p, p_back);
            let p_back_affine = AffinePoint::from(p_back);
            assert_eq!(p_affine, p_back_affine);
        }
    }

    #[test]
    fn test_zkvm_add_assign() {
        let mut rng = rand::thread_rng();
        let num_iters = 100;

        // Test identity + identity = identity.
        let mut p = AffinePoint::identity();
        let q = AffinePoint::identity();
        p += q;
        assert_eq!(p, AffinePoint::identity());

        // Test compatibility with Edwards arithmetic.
        let base = ED25519_BASEPOINT_POINT;
        for _ in 0..num_iters {
            let scalar_p = Scalar::random(&mut rng);
            let scalar_q = Scalar::random(&mut rng);
            let p = serial_scalar_mul(&base, &scalar_p);
            let q = serial_scalar_mul(&base, &scalar_q);
            let mut p_affine = AffinePoint::from(p);
            let q_affine = AffinePoint::from(q);
            p_affine += q_affine;

            let p_plus_q = EdwardsPoint::from(p_affine);
            let p_plus_q_edwards = normalize(&(p + q));
            assert_eq!(p_plus_q, p_plus_q_edwards);
        }
    }
}
