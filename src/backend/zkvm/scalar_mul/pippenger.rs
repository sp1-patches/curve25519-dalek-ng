use core::borrow::Borrow;
use edwards::EdwardsPoint;
use scalar::Scalar;
use traits::VartimeMultiscalarMul;

pub struct Pippenger;

#[cfg(any(feature = "alloc", feature = "std"))]
impl VartimeMultiscalarMul for Pippenger {
    type Point = EdwardsPoint;

    fn optional_multiscalar_mul<I, J>(_scalars: I, _points: J) -> Option<EdwardsPoint>
    where
        I: IntoIterator,
        I::Item: Borrow<Scalar>,
        J: IntoIterator<Item = Option<EdwardsPoint>>,
    {
        unimplemented!("Pippenger is not supported yet for zkvm")
    }
}
