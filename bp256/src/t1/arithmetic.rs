//! brainpoolP256t1 curve arithmetic implementation.

use super::BrainpoolP256t1;
use crate::{FieldElement, Scalar};
use elliptic_curve::{CurveArithmetic, PrimeCurveArithmetic};
use primeorder::{point_arithmetic, PrimeCurveParams};

/// Elliptic curve point in affine coordinates.
pub type AffinePoint = primeorder::AffinePoint<BrainpoolP256t1>;

/// Elliptic curve point in projective coordinates.
pub type ProjectivePoint = primeorder::ProjectivePoint<BrainpoolP256t1>;

/// Primitive scalar type.
pub type ScalarPrimitive = elliptic_curve::ScalarPrimitive<BrainpoolP256t1>;

impl CurveArithmetic for BrainpoolP256t1 {
    type AffinePoint = AffinePoint;
    type ProjectivePoint = ProjectivePoint;
    type Scalar = Scalar;
}

impl PrimeCurveArithmetic for BrainpoolP256t1 {
    type CurveGroup = ProjectivePoint;
}

impl PrimeCurveParams for BrainpoolP256t1 {
    type FieldElement = FieldElement;
    type PointArithmetic = point_arithmetic::EquationAIsGeneric;

    const EQUATION_A: FieldElement =
        FieldElement::from_hex("a9fb57dba1eea9bc3e660a909d838d726e3bf623d52620282013481d1f6e5374");
    const EQUATION_B: FieldElement =
        FieldElement::from_hex("662c61c430d84ea4fe66a7733d0b76b7bf93ebc4af2f49256ae58101fee92b04");
    const GENERATOR: (FieldElement, FieldElement) = (
        FieldElement::from_hex("a3e8eb3cc1cfe7b7732213b23a656149afa142c47aafbc2b79a191562e1305f4"),
        FieldElement::from_hex("2d996c823439c56d7f7b22e14644417e69bcb6de39d027001dabe8f35b25c9be"),
    );
}

impl From<ScalarPrimitive> for Scalar {
    fn from(w: ScalarPrimitive) -> Self {
        Scalar::from(&w)
    }
}

impl From<&ScalarPrimitive> for Scalar {
    fn from(w: &ScalarPrimitive) -> Scalar {
        Scalar::from_uint_unchecked(*w.as_uint())
    }
}

impl From<Scalar> for ScalarPrimitive {
    fn from(scalar: Scalar) -> ScalarPrimitive {
        ScalarPrimitive::from(&scalar)
    }
}

impl From<&Scalar> for ScalarPrimitive {
    fn from(scalar: &Scalar) -> ScalarPrimitive {
        ScalarPrimitive::new(scalar.into()).unwrap()
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    extern crate std;
    use std::println;
    use crate::test_vectors::r1::{ADD_TEST_VECTORS, MUL_TEST_VECTORS};
    use core::ops::Mul;

    #[test]
    fn playground() {
        let x: FieldElement = FieldElement::from_hex(
            "44106E913F92BC02A1705D9953A8414DB95E1AAA49E81D9E85F929A8E3100BE5",
        );
        let y: FieldElement = FieldElement::from_hex(
            "8AB4846F11CACCB73CE49CBDD120F5A900A69FD32C272223F789EF10EB089BDC",
        );
        let p: AffinePoint = AffinePoint::from_affine_coordinates(x, y);
        println!("playground ✔︎");
    }

    #[test]
    fn scalar_multiplication() {
        let mut counter = 0;
        for i in 0..MUL_TEST_VECTORS.len() {
            let a: Scalar = Scalar::from_slice(&MUL_TEST_VECTORS[i].0).unwrap();
            let x: FieldElement = FieldElement::from_slice(&MUL_TEST_VECTORS[i].1).unwrap();
            let y: FieldElement = FieldElement::from_slice(&MUL_TEST_VECTORS[i].2).unwrap();
            let p: AffinePoint = ProjectivePoint::GENERATOR.mul(&a).to_affine(); // use the implemented scalar multiplication
            let p_ref: AffinePoint = AffinePoint::from_affine_coordinates(x, y);
            assert_eq!(p, p_ref);
            if p == p_ref {
                counter += 1;
            }
        }
        println!("scalar_multiplication ✔︎");
        println!("sample size  = {:?}", MUL_TEST_VECTORS.len());
        println!(
            "success rate = {:?} %",
            (100 * counter) as f64 / MUL_TEST_VECTORS.len() as f64
        );
    }
}
