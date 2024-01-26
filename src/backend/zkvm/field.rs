use core::{array::TryFromSliceError, convert::TryFrom};

use crate::field::FieldElement;

/// A `FieldElement` of the prime field of modulus `2^255-19` represented as u32 limbs.
///
/// The field element is represented in radix 2^32 and is stored in little-endian order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElemetLimbs32(pub(super) [u32; 8]);

impl From<[u32; 8]> for FieldElemetLimbs32 {
    fn from(limbs: [u32; 8]) -> Self {
        FieldElemetLimbs32(limbs)
    }
}

impl TryFrom<&[u32]> for FieldElemetLimbs32 {
    type Error = TryFromSliceError;

    #[inline]
    fn try_from(value: &[u32]) -> Result<Self, Self::Error> {
        Ok(Self(<[u32; 8]>::try_from(value)?))
    }
}

impl From<FieldElemetLimbs32> for FieldElement {
    fn from(limbs: FieldElemetLimbs32) -> Self {
        let mut bytes = [0u8; 32];
        for (i, limb) in limbs.0.iter().enumerate() {
            bytes[i * 4..(i + 1) * 4].copy_from_slice(&limb.to_le_bytes());
        }

        FieldElement::from_bytes(&bytes)
    }
}
