//! brainpoolP256r1 curve arithmetic implementation.

use super::BrainpoolP256r1;
use crate::{FieldElement, Scalar};
use elliptic_curve::{CurveArithmetic, PrimeCurveArithmetic};
use primeorder::{point_arithmetic, PrimeCurveParams};

/// Elliptic curve point in affine coordinates.
pub type AffinePoint = primeorder::AffinePoint<BrainpoolP256r1>;

/// Elliptic curve point in projective coordinates.
pub type ProjectivePoint = primeorder::ProjectivePoint<BrainpoolP256r1>;

/// Primitive scalar type.
pub type ScalarPrimitive = elliptic_curve::ScalarPrimitive<BrainpoolP256r1>;

impl CurveArithmetic for BrainpoolP256r1 {
    type AffinePoint = AffinePoint;
    type ProjectivePoint = ProjectivePoint;
    type Scalar = Scalar;
}

impl PrimeCurveArithmetic for BrainpoolP256r1 {
    type CurveGroup = ProjectivePoint;
}

impl PrimeCurveParams for BrainpoolP256r1 {
    type FieldElement = FieldElement;
    type PointArithmetic = point_arithmetic::EquationAIsGeneric;

    const EQUATION_A: FieldElement =
        FieldElement::from_hex("7d5a0975fc2c3057eef67530417affe7fb8055c126dc5c6ce94a4b44f330b5d9");
    const EQUATION_B: FieldElement =
        FieldElement::from_hex("26dc5c6ce94a4b44f330b5d9bbd77cbf958416295cf7e1ce6bccdc18ff8c07b6");
    const GENERATOR: (FieldElement, FieldElement) = (
        FieldElement::from_hex("8bd2aeb9cb7e57cb2c4b482ffc81b7afb9de27e1e3bd23c23a4453bd9ace3262"),
        FieldElement::from_hex("547ef835c3dac4fd97f8461a14611dc9c27745132ded8e545c1d54c72f046997"),
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
    use crate::test_vectors::r1::MUL_TEST_VECTORS;
    extern crate std;
    use std::println;
    use elliptic_curve::group::GroupEncoding;
    use elliptic_curve::sec1::ToEncodedPoint;


    /*
    #[test]
    fn brainpool_test() {
        // from brainpool, https://datatracker.ietf.org/doc/rfc8734/

        let d_a: Scalar = Scalar::from_hex("81DB1EE100150FF2EA338D708271BE38300CB54241D79950F77B063039804F1D");
        let x_qa: FieldElement = FieldElement::from_hex("44106E913F92BC02A1705D9953A8414DB95E1AAA49E81D9E85F929A8E3100BE5");
        let y_qa :FieldElement = FieldElement::from_hex("8AB4846F11CACCB73CE49CBDD120F5A900A69FD32C272223F789EF10EB089BDC");

        let d_b : Scalar = Scalar::from_hex("55E40BC41E37E3E2AD25C3C6654511FFA8474A91A0032087593852D3E7D76BD3");
        let x_qb :FieldElement = FieldElement::from_hex("8D2D688C6CF93E1160AD04CC4429117DC2C41825E1E9FCA0ADDD34E6F1B39F7B");
        let y_qb :FieldElement = FieldElement::from_hex("990C57520812BE512641E47034832106BC7D3E8DD0E4C7F1136D7006547CEC6A");

        let x_z :FieldElement  = FieldElement::from_hex("89AFC39D41D3B327814B80940B042590F96556EC91E6AE7939BCE31F3A18BF2B");
        let y_z :FieldElement  = FieldElement::from_hex("49C27868F4ECA2179BFD7D59B1E3BF34C1DBDE61AE12931648F43E59632504DE");

        let q_a: AffinePoint = P.mul(&d_a).to_affine();
        assert_eq!(q_a.x, x_qa);
        assert_eq!(q_a.y, y_qa);

        let q_b: AffinePoint = P.mul(&d_b).to_affine();
        assert_eq!(q_b.x, x_qb);
        assert_eq!(q_b.y, y_qb);

        let d_ab: Scalar = d_a.multiply(&d_b);
        let q_ab: AffinePoint = P.mul(&d_ab).to_affine();
        assert_eq!(q_ab.x, x_z);
        assert_eq!(q_ab.y, y_z);
    }

    */
    
    // TO DO: Go on here, recover the bytes!!
    
    #[test]
    fn playground(){
        let p: ProjectivePoint = ProjectivePoint::GENERATOR;
        let a: Scalar = Scalar::from_u64(1);
        println!("{:?}",p.to_bytes());
        println!("{:?}",p.to_encoded_point(false).to_bytes());
    }
    
    
    
    #[test]
    fn scalar_multiplication() {
        let mut counter = 0;
        for i in 0..MUL_TEST_VECTORS.len() {
            let a: Scalar = Scalar::from_slice(&MUL_TEST_VECTORS[0].0).unwrap();
            let x: FieldElement = FieldElement::from_slice(&MUL_TEST_VECTORS[0].1).unwrap();
            let y: FieldElement = FieldElement::from_slice(&MUL_TEST_VECTORS[0].2).unwrap();
            let p: AffinePoint = ProjectivePoint::GENERATOR.mul(&a).to_affine();
            let q: AffinePoint = AffinePoint { x, y, infinity: 0 };
            assert_eq!(p, q);
            if (p == q) {
                counter += 1;
            }
        }
        println!("sample size  = {:?}", MUL_TEST_VECTORS.len());
        println!(
            "success rate = {:?}%",
            (100 * counter) as f64 / MUL_TEST_VECTORS.len() as f64
        );
    }
}
