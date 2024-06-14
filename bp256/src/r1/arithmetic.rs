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
    extern crate std;
    use crate::test_vectors::r1::MUL_TEST_VECTORS;
    use elliptic_curve::{group::GroupEncoding, sec1::ToEncodedPoint};
    use hex_literal::hex;
    use std::println;

    type EncodedPoint = elliptic_curve::sec1::EncodedPoint<BrainpoolP256r1>;

    #[test]
    fn playground() {
        let x: FieldElement =
            FieldElement::from_hex("44106E913F92BC02A1705D9953A8414DB95E1AAA49E81D9E85F929A8E3100BE5");
        let y: FieldElement =
            FieldElement::from_hex("8AB4846F11CACCB73CE49CBDD120F5A900A69FD32C272223F789EF10EB089BDC");
        let p: ProjectivePoint =
            ProjectivePoint::from_affine_coordinates(&x, &y);
        println!("playground ✔︎");
    }

    #[test]
    // test values from brainpool, https://datatracker.ietf.org/doc/rfc8734/
    fn brainpool_test() {
        let p: ProjectivePoint = ProjectivePoint::GENERATOR;

        let d_a: Scalar = Scalar::from_hex("81DB1EE100150FF2EA338D708271BE38300CB54241D79950F77B063039804F1D");
        let x_qa: FieldElement = FieldElement::from_hex("44106E913F92BC02A1705D9953A8414DB95E1AAA49E81D9E85F929A8E3100BE5");
        let y_qa: FieldElement = FieldElement::from_hex("8AB4846F11CACCB73CE49CBDD120F5A900A69FD32C272223F789EF10EB089BDC");
        let q_a: ProjectivePoint = p.mul(&d_a);
        let q_a_ref: ProjectivePoint = ProjectivePoint::from_affine_coordinates(&x_qa, &y_qa);
        assert_eq!(q_a, q_a_ref);

        let d_b: Scalar = Scalar::from_hex("55E40BC41E37E3E2AD25C3C6654511FFA8474A91A0032087593852D3E7D76BD3");
        let x_qb: FieldElement = FieldElement::from_hex("8D2D688C6CF93E1160AD04CC4429117DC2C41825E1E9FCA0ADDD34E6F1B39F7B");
        let y_qb: FieldElement = FieldElement::from_hex("990C57520812BE512641E47034832106BC7D3E8DD0E4C7F1136D7006547CEC6A");
        let q_b: ProjectivePoint = p.mul(&d_b);
        let q_b_ref: ProjectivePoint = ProjectivePoint::from_affine_coordinates(&x_qb, &y_qb);
        assert_eq!(q_b, q_b_ref);

        let d_ab: Scalar = d_a.multiply(&d_b);
        let x_z: FieldElement = FieldElement::from_hex("89AFC39D41D3B327814B80940B042590F96556EC91E6AE7939BCE31F3A18BF2B");
        let y_z: FieldElement = FieldElement::from_hex("49C27868F4ECA2179BFD7D59B1E3BF34C1DBDE61AE12931648F43E59632504DE");
        let q_ab: ProjectivePoint = p.mul(&d_ab);
        let q_ab_ref: ProjectivePoint = ProjectivePoint::from_affine_coordinates(&x_z, &y_z);
        assert_eq!(q_b, q_b_ref);

        println!("brainpool_test ✔︎");
    }

    #[test]
    fn scalar_multiplication() {
        let mut counter = 0;
        for i in 0..MUL_TEST_VECTORS.len() {
            //MUL_TEST_VECTORS.len()
            let a: Scalar = Scalar::from_slice(&MUL_TEST_VECTORS[i].0).unwrap();
            let x: FieldElement = FieldElement::from_slice(&MUL_TEST_VECTORS[i].1).unwrap();
            let y: FieldElement = FieldElement::from_slice(&MUL_TEST_VECTORS[i].2).unwrap();
            let p: ProjectivePoint = ProjectivePoint::GENERATOR.mul(&a); // use the implemented scalar multiplication
            let p_ref: ProjectivePoint = ProjectivePoint::from_affine_coordinates(&x, &y);
            assert_eq!(p, p_ref);
            if (p == p_ref) {
                counter += 1;
            }
        }
        println!("scalar_multiplication ✔︎");
        println!("sample size  = {:?}", MUL_TEST_VECTORS.len());
        println!("success rate = {:?} %", (100 * counter) as f64 / MUL_TEST_VECTORS.len() as f64);
    }

    /*
    #[test]
    fn scalar_multiplication_enc() {
        let mut counter = 0;
        for i in 0..MUL_TEST_VECTORS.len() { //MUL_TEST_VECTORS.len()
            let a: Scalar = Scalar::from_slice(&MUL_TEST_VECTORS[i].0).unwrap();
            let p: AffinePoint = ProjectivePoint::GENERATOR.mul(&a).to_affine();  // use the implemented scalar multiplication
            let p_enc: EncodedPoint = p.to_encoded_point(false); // TO DO: Make this conversion obsolete
            let p_ref: EncodedPoint = EncodedPoint::from_affine_coordinates(&Array(MUL_TEST_VECTORS[i].1), &Array(MUL_TEST_VECTORS[i].2), false); // this might be replaced once there is a method for AffinePoint
            assert_eq!(p_enc, p_ref);
            if(p_enc == p_ref){
                counter += 1;
            }
        }
        println!("sample size  = {:?}", MUL_TEST_VECTORS.len());
        println!("success rate = {:?} %", (100 * counter) as f64 / MUL_TEST_VECTORS.len() as f64);
    }
     */

    /*
    #[test]
    fn scalar_multiplication_old() {
        use crate::test_vectors::r1::MUL_TEST_VECTORS;
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

    */
}
