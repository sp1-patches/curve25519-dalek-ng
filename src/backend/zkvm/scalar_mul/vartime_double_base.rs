use backend::serial::u32::constants::ED25519_BASEPOINT_POINT;
use backend::zkvm::edwards::AffinePoint;
use edwards::EdwardsPoint;
use scalar::Scalar;
use traits::Identity;

/// Compute \\(aA + bB\\) in variable time, where \\(B\\) is the Ed25519 basepoint.
#[allow(non_snake_case)]
pub fn mul(a: &Scalar, A: &EdwardsPoint, b: &Scalar) -> EdwardsPoint {
    let A = AffinePoint::from(*A);

    double_and_add_base(a, &A, b).into()
}

#[allow(non_snake_case)]
fn double_and_add_base(a: &Scalar, A: &AffinePoint, b: &Scalar) -> AffinePoint {
    let mut res = AffinePoint::identity();
    let mut temp_A = *A;
    let mut temp_B = AffinePoint::from(ED25519_BASEPOINT_POINT);

    for (a_bit, b_bit) in a.bits().iter().zip(b.bits()) {
        if *a_bit == 1 {
            res += &temp_A;
        }

        if b_bit == 1 {
            res += &temp_B;
        }

        temp_A.double();
        temp_B.double();
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use backend::serial::u32::constants::ED25519_BASEPOINT_POINT;
    use backend::zkvm::edwards::normalize;
    use backend::zkvm::edwards::tests::serial_scalar_mul;

    #[test]
    #[allow(non_snake_case)]
    fn test_zkvm_variable_double_base_mul() {
        let mut rng = rand::thread_rng();
        let num_iters = 100;

        let base = ED25519_BASEPOINT_POINT;
        for _ in 0..num_iters {
            let a_scalar = Scalar::random(&mut rng);
            let A = serial_scalar_mul(&base, &a_scalar);

            let a = Scalar::random(&mut rng);
            let b = Scalar::random(&mut rng);

            let a_A_plus_b_B = mul(&a, &A, &b);
            let expected = normalize(&(serial_scalar_mul(&A, &a) + &serial_scalar_mul(&base, &b)));
            assert_eq!(a_A_plus_b_B, expected);
        }
    }
}
