use crate::field::{SizedPrimeField};
use crate::representation::ElementRepr;
use crate::traits::{FieldElement, BitIterator, FieldExtension};
use super::fp6_as_3_over_2::{Fp6, Extension3Over2};
use super::fp2::Fp2;

// this implementation assumes extension using polynomial w^2 - v = 0
pub struct Fp12<'a, E: ElementRepr, F: SizedPrimeField<Repr = E> >{
    pub c0: Fp6<'a, E, F>,
    pub c1: Fp6<'a, E, F>,
    pub extension_field: &'a Extension2Over3Over2<'a, E, F>
}

impl<'a, E: ElementRepr, F: SizedPrimeField<Repr = E> >std::fmt::Display for Fp12<'a, E, F> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Fq12({} + {} * w)", self.c0, self.c1)
    }
}

impl<'a, E: ElementRepr, F: SizedPrimeField<Repr = E> >std::fmt::Debug for Fp12<'a, E, F> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Fq12({} + {} * w)", self.c0, self.c1)
    }
}

impl<'a, E: ElementRepr, F: SizedPrimeField<Repr = E> > Clone for Fp12<'a, E, F> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self{
            c0: self.c0.clone(),
            c1: self.c1.clone(),
            extension_field: self.extension_field
        }
    }
}

impl<'a, E: ElementRepr, F: SizedPrimeField<Repr = E> > PartialEq for Fp12<'a, E, F> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.c0 == other.c0 && 
        self.c1 == other.c1
    }
}

impl<'a, E: ElementRepr, F: SizedPrimeField<Repr = E> > Eq for Fp12<'a, E, F> {
}

impl<'a, E: ElementRepr, F: SizedPrimeField<Repr = E> > Fp12<'a, E, F> {
    pub fn zero(extension_field: &'a Extension2Over3Over2<'a, E, F>) -> Self {
        let zero = Fp6::zero(extension_field.field);
        
        Self {
            c0: zero.clone(),
            c1: zero,
            extension_field: extension_field
        }
    }

    pub fn one(extension_field: &'a Extension2Over3Over2<'a, E, F>) -> Self {
        let zero = Fp6::zero(extension_field.field);
        let one = Fp6::one(extension_field.field);
        
        Self {
            c0: one,
            c1: zero,
            extension_field: extension_field
        }
    }

    pub fn mul_by_034(
        &mut self,
        c0: & Fp2<'a, E, F>,
        c3: & Fp2<'a, E, F>,
        c4: & Fp2<'a, E, F>,
    ) {
        let mut a = self.c0.clone();
        a.c0.mul_assign(c0);
        a.c1.mul_assign(c0);
        a.c2.mul_assign(c0);

        let mut b = self.c1.clone();
        b.mul_by_01(&c3, &c4);

        let mut t0 = c0.clone();
        t0.add_assign(c3);

        let mut e = self.c0.clone();
        e.add_assign(&self.c1);
        e.mul_by_01(&t0, &c4);

        self.c1 = e;
        self.c1.sub_assign(&a);
        self.c1.sub_assign(&b);


        let mut t1 = b.clone();
        t1.mul_by_nonresidue(self.extension_field);
        self.c0 = a;
        self.c0.add_assign(&t1);
    }

    pub fn mul_by_014(
        &mut self,
        c0: & Fp2<'a, E, F>,
        c1: & Fp2<'a, E, F>,
        c4: & Fp2<'a, E, F>,
    ) {
        let mut aa = self.c0.clone();
        aa.mul_by_01(c0, c1);
        let mut bb = self.c1.clone();
        bb.mul_by_1(c4);
        let mut o = c1.clone();
        o.add_assign(c4);
        self.c1.add_assign(&self.c0);
        self.c1.mul_by_01(c0, &o);
        self.c1.sub_assign(&aa);
        self.c1.sub_assign(&bb);
        self.c0 = bb;
        // println!("Mul 014 C0 = {}", self.c0);
        self.c0.mul_by_nonresidue(self.extension_field);
        // println!("Mul 014 C0 = {}", self.c0);
        self.c0.add_assign(&aa);
        // println!("Mul 014 C0 = {}", self.c0);
    }

    pub fn cyclotomic_square(&mut self) {
        let z0 = self.c0.c0.clone();
        let z4 = self.c0.c1.clone();
        let z3 = self.c0.c2.clone();
        let z2 = self.c1.c0.clone();
        let z1 = self.c1.c1.clone();
        let z5 = self.c1.c2.clone();

        // t0 + t1*y = (z0 + z1*y)^2 = a^2
        let mut tmp = z0.clone();
        tmp.mul_assign(&z1);

        let mut a0 = z0.clone();
        a0.add_assign(&z1);
        let mut a1 = z1.clone();
        a1.mul_by_nonresidue(self.extension_field.field);
        a1.add_assign(&z0);

        let mut a2 = tmp.clone();
        a2.mul_by_nonresidue(self.extension_field.field);

        let mut t0 = a0;
        t0.mul_assign(&a1);
        t0.sub_assign(&tmp);
        t0.sub_assign(&a2);
        let mut t1 = tmp;
        t1.double();

        // t2 + t3*y = (z2 + z3*y)^2 = b^2
        let mut tmp = z2.clone();
        tmp.mul_assign(&z3);

        let mut a0 = z2.clone();
        a0.add_assign(&z3);
        let mut a1 = z3.clone();
        a1.mul_by_nonresidue(self.extension_field.field);
        a1.add_assign(&z2);

        let mut a2 = tmp.clone();
        a2.mul_by_nonresidue(self.extension_field.field);

        let mut t2 = a0;
        t2.mul_assign(&a1);
        t2.sub_assign(&tmp);
        t2.sub_assign(&a2);

        let mut t3 = tmp;
        t3.double();

        // t4 + t5*y = (z4 + z5*y)^2 = c^2
        let mut tmp = z4.clone();
        tmp.mul_assign(&z5);

        let mut a0 = z4.clone();
        a0.add_assign(&z5);
        let mut a1 = z5.clone();
        a1.mul_by_nonresidue(self.extension_field.field);
        a1.add_assign(&z4);

        let mut a2 = tmp.clone();
        a2.mul_by_nonresidue(self.extension_field.field);

        let mut t4 = a0;
        t4.mul_assign(&a1);
        t4.sub_assign(&tmp);
        t4.sub_assign(&a2);

        let mut t5 = tmp.clone();
        t5.double();

        // for A

        // g0 = 3 * t0 - 2 * z0
        let mut g0 = t0.clone();
        g0.sub_assign(&z0);
        g0.double();
        g0.add_assign(&t0);

        self.c0.c0 = g0;

        // g1 = 3 * t1 + 2 * z1
        let mut g1 = t1.clone();
        g1.add_assign(&z1);
        g1.double();
        g1.add_assign(&t1);
        self.c1.c1 = g1;

        // for B

        // g2 = 3 * (xi * t5) + 2 * z2
        let mut tmp = t5.clone();
        tmp.mul_by_nonresidue(self.extension_field.field);
        let mut g2 = tmp.clone();
        g2.add_assign(&z2);
        g2.double();
        g2.add_assign(&tmp);
        self.c1.c0 = g2;

        // g3 = 3 * t4 - 2 * z3
        let mut g3 = t4.clone();
        g3.sub_assign(&z3);
        g3.double();
        g3.add_assign(&t4);
        self.c0.c2 = g3;

        // for C

        // g4 = 3 * t2 - 2 * z4
        let mut g4 = t2.clone();
        g4.sub_assign(&z4);
        g4.double();
        g4.add_assign(&t2);
        self.c0.c1 = g4;

        // g5 = 3 * t3 + 2 * z5
        let mut g5 = t3.clone();
        g5.add_assign(&z5);
        g5.double();
        g5.add_assign(&t3);
        self.c1.c2 = g5;
    }

    pub fn cyclotomic_exp<S: AsRef<[u64]>>(&self, exp: S) -> Self {
        let mut res = Self::one(&self.extension_field);

        let mut found_one = false;

        for i in BitIterator::new(exp) {
            if found_one {
                res.cyclotomic_square();
            } else {
                found_one = i;
            }

            if i {
                res.mul_assign(self);
            }
        }

        res
    }
}

impl<'a, E: ElementRepr, F: SizedPrimeField<Repr = E> > FieldElement for Fp12<'a, E, F> {
    /// Returns true iff this element is zero.
    fn is_zero(&self) -> bool {
        self.c0.is_zero() && 
        self.c1.is_zero()
    }

    fn add_assign(&mut self, other: &Self) {
        self.c0.add_assign(&other.c0);
        self.c1.add_assign(&other.c1);
    }

    fn double(&mut self) {
        self.c0.double();
        self.c1.double();
    }

    fn sub_assign(&mut self, other: &Self) {
        self.c0.sub_assign(&other.c0);
        self.c1.sub_assign(&other.c1);
    }

    fn negate(&mut self) {
        self.c0.negate();
        self.c1.negate();
    }

    fn inverse(&self) -> Option<Self> {
        let mut c0s = self.c0.clone();
        c0s.square();
        let mut c1s = self.c1.clone();
        c1s.square();
        c1s.mul_by_nonresidue(self.extension_field);
        c0s.sub_assign(&c1s);

        c0s.inverse().map(|t| {
            let mut tmp = Fp12 { 
                c0: t.clone(), 
                c1: t.clone(),
                extension_field: self.extension_field
            };
            tmp.c0.mul_assign(&self.c0);
            tmp.c1.mul_assign(&self.c1);
            tmp.c1.negate();

            tmp
        })
    }

    fn mul_assign(&mut self, other: &Self)
    {
        let mut aa = self.c0.clone();
        aa.mul_assign(&other.c0);
        let mut bb = self.c1.clone();
        bb.mul_assign(&other.c1);
        let mut o = other.c0.clone();
        o.add_assign(&other.c1);
        self.c1.add_assign(&self.c0);
        self.c1.mul_assign(&o);
        self.c1.sub_assign(&aa);
        self.c1.sub_assign(&bb);
        self.c0 = bb;
        self.c0.mul_by_nonresidue(self.extension_field);
        self.c0.add_assign(&aa);
    }

    fn square(&mut self)
    {
        let mut ab = self.c0.clone();
        ab.mul_assign(&self.c1);
        let mut c0c1 = self.c0.clone();
        c0c1.add_assign(&self.c1);
        let mut c0 = self.c1.clone();
        c0.mul_by_nonresidue(self.extension_field);
        c0.add_assign(&self.c0);
        c0.mul_assign(&c0c1);
        c0.sub_assign(&ab);
        self.c1 = ab.clone();
        self.c1.add_assign(&ab);
        ab.mul_by_nonresidue(self.extension_field);
        c0.sub_assign(&ab);
        self.c0 = c0;
    }

    fn conjugate(&mut self) {
        self.c1.negate();
    }

    fn pow<S: AsRef<[u64]>>(&self, exp: S) -> Self {
        let mut res = Self::one(&self.extension_field);

        let mut found_one = false;

        for i in BitIterator::new(exp) {
            if found_one {
                res.square();
            } else {
                found_one = i;
            }

            if i {
                res.mul_assign(self);
            }
        }

        res
    }

    fn mul_by_nonresidue<EXT: FieldExtension<Element = Self>>(&mut self, for_extesion: &EXT) {
        unreachable!();
        // for_extesion.multiply_by_non_residue(self);
    }

    fn frobenius_map(&mut self, power: usize) {
        self.c0.frobenius_map(power);
        self.c1.frobenius_map(power);

        self.c1
            .c0
            .mul_assign(&self.extension_field.frobenius_coeffs_c1[power % 12]);
        self.c1
            .c1
            .mul_assign(&self.extension_field.frobenius_coeffs_c1[power % 12]);
        self.c1
            .c2
            .mul_assign(&self.extension_field.frobenius_coeffs_c1[power % 12]);
    }
}

pub struct Extension2Over3Over2<'a, E: ElementRepr, F: SizedPrimeField<Repr = E> > {
    pub non_residue: Fp6<'a, E, F>,
    pub field: &'a Extension3Over2<'a, E, F>,
    pub frobenius_coeffs_c1: [Fp2<'a, E, F>; 12],
}

impl<'a, E: ElementRepr, F: SizedPrimeField<Repr = E> > FieldExtension for Extension2Over3Over2<'a, E, F> {
    const EXTENSION_DEGREE: usize = 2;
    
    type Element = Fp6<'a, E, F>;

    fn multiply_by_non_residue(&self, el: &mut Self::Element) {
        // IMPORTANT: This only works cause the structure of extension field for Fp12
        // is w^2 - v = 0!
        // take an element in Fp6 that is 3 over 2 and multiply by non-residue
        // (c0 + c1 * v + c2 * v^2)*v with v^3 - xi = 0 -> (c2*xi + c0 * v + c1 * v^2)
        let mut new_c0 = el.c2.clone();
        new_c0.mul_by_nonresidue(&*el.extension_field);
        el.c2 = el.c1.clone();
        el.c1 = el.c0.clone();
        el.c0 = new_c0;
    }

}
