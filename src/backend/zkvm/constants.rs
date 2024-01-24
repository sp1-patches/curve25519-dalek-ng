use super::edwards::AffinePoint;

pub(super) const IDENTITY: AffinePoint =
    AffinePoint::from_limbs([0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
