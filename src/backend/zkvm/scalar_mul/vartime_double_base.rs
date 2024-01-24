use edwards::EdwardsPoint;
use scalar::Scalar;

/// Compute \\(aA + bB\\) in variable time, where \\(B\\) is the Ed25519 basepoint.
pub fn mul(a: &Scalar, A: &EdwardsPoint, b: &Scalar) -> EdwardsPoint {
    unimplemented!("Pre-computed straus is not supported yet for zkvm")
}
