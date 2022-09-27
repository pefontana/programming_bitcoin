use crate::{bigint, constants::PRIME, errors::NotPrimeError};
use num_bigint::BigInt;
use num_integer::Integer;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    pub num: BigInt,
}

impl FieldElement {
    pub fn new(num: BigInt) -> Self {
        Self { num }
    }
}

impl Add<FieldElement> for FieldElement {
    type Output = Self;

    fn add(self, other_field_elem: Self) -> Self {
        Self {
            num: (self.num + other_field_elem.num).mod_floor(&PRIME),
        }
    }
}

impl Sub<FieldElement> for FieldElement {
    type Output = Self;

    fn sub(self, other_field_elem: Self) -> Self {
        Self {
            num: (self.num - other_field_elem.num).mod_floor(&PRIME),
        }
    }
}

impl Mul<FieldElement> for FieldElement {
    type Output = Self;

    fn mul(self, other_field_elem: Self) -> Self {
        Self {
            num: (self.num * other_field_elem.num).mod_floor(&PRIME),
        }
    }
}
// TODO
//Check perfomance
impl FieldElement {
    pub fn pow(&self, n: BigInt) -> Self {
        let exp = bigint!(n).mod_floor(&(&*PRIME - 1_usize));
        let mut num = self.num.clone();
        for _ in num_iter::range_inclusive(bigint!(1), exp) {
            num *= &self.num;
            num = num.mod_floor(&PRIME);
        }
        Self {
            num: num.mod_floor(&PRIME),
        }
    }
}

impl Div<FieldElement> for FieldElement {
    type Output = Self;

    fn div(self, other_field_elem: Self) -> Self {
        self * other_field_elem.pow(&*PRIME - 2)
    }
}

impl AddAssign<FieldElement> for FieldElement {
    fn add_assign(&mut self, other_number: Self) {
        self.num = (&self.num + other_number.num).mod_floor(&PRIME);
    }
}

impl SubAssign<FieldElement> for FieldElement {
    fn sub_assign(&mut self, other_number: Self) {
        self.num = (&self.num - other_number.num).mod_floor(&PRIME);
    }
}

impl MulAssign<FieldElement> for FieldElement {
    fn mul_assign(&mut self, other_number: Self) {
        self.num = (&self.num * other_number.num).mod_floor(&PRIME);
    }
}

#[cfg(test)]
mod tests {
    use crate::felt;

    use super::*;

    #[test]
    fn test_add_two_finite_field_elements() {
        let first_field_element = felt!(11); //FieldElement::<11>::new(1).unwrap();
        let second_field_element = felt!(11); //FieldElement::<11>::new(20).unwrap();

        assert_eq!(first_field_element + second_field_element, felt!(22));
    }

    #[test]
    fn test_sub_two_finite_field_elements() {
        let first_field_element = felt!(20);
        let second_field_element = felt!(3);

        assert_eq!(first_field_element - second_field_element, felt!(17));
    }
    // TODO
    // ADD add and sub tests with num > PRIME

    // #[test]
    // fn test_mul_two_finite_field_elements() {
    //     let first_field_element = FieldElement::<11>::new(1).unwrap();
    //     let second_field_element = FieldElement::<11>::new(20).unwrap();

    //     assert_eq!(
    //         first_field_element * second_field_element,
    //         FieldElement::<11>::new(9).unwrap()
    //     );
    // }

    // #[test]
    // fn test_pow_a_finite_field_with_a_number() {
    //     let first_field_element = FieldElement::<11>::new(3).unwrap();

    //     assert_eq!(
    //         first_field_element.pow(3),
    //         FieldElement::<11>::new(5).unwrap()
    //     );
    // }

    // #[test]
    // fn test_div_two_finite_field_elements() {
    //     let first_field_element = FieldElement::<11>::new(1).unwrap();
    //     let second_field_element = FieldElement::<11>::new(20).unwrap();

    //     assert_eq!(
    //         first_field_element / second_field_element,
    //         FieldElement::<11>::new(5).unwrap()
    //     );
    // }
}
