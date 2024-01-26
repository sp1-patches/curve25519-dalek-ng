use backend::zkvm::edwards::AffinePoint;
use edwards::EdwardsPoint;
use scalar::Scalar;

use traits::Identity;

pub(crate) fn mul(point: &EdwardsPoint, scalar: &Scalar) -> EdwardsPoint {
    let point = AffinePoint::from(*point);

    double_and_add(&point, scalar).into()
}

fn double_and_add(point: &AffinePoint, scalar: &Scalar) -> AffinePoint {
    let mut res = AffinePoint::identity();
    let mut temp = *point;

    for bit in scalar.bits() {
        if bit == 1 {
            res += &temp;
        }

        temp.double();
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use backend::serial::u32::constants::ED25519_BASEPOINT_POINT;
    use backend::zkvm::edwards::tests::serial_scalar_mul;

    #[test]
    fn test_zkvm_variable_base_mul() {
        let mut rng = rand::thread_rng();
        let num_iters = 100;

        let base = ED25519_BASEPOINT_POINT;
        let id = EdwardsPoint::identity();
        for _ in 0..num_iters {
            let scalar = Scalar::random(&mut rng);
            let id_times_scalar = mul(&id, &scalar);
            assert_eq!(id_times_scalar, id);

            let point_scalar = Scalar::random(&mut rng);
            let point = serial_scalar_mul(&base, &point_scalar);
            let multiple = mul(&point, &scalar);
            let expected_mul = serial_scalar_mul(&base, &(point_scalar * scalar));
            assert_eq!(multiple, expected_mul);
        }
    }
}
