use std::ops::{Add, Mul};

use crate::constants::{A, B};
use crate::errors::PointNotInTheCurve;
use crate::field_element::FieldElement;
use crate::{bigint, felt};
use num_bigint_dig::BigInt;
use num_traits::identities::Zero;
// Point of y**2 = x**2 + a*x + b eliptic curve
#[derive(Clone, Debug, PartialEq)]
pub enum Point {
    Point(FieldElement, FieldElement),
    Infinity,
}

impl Point {
    pub fn new_point(x: FieldElement, y: FieldElement) -> Result<Self, PointNotInTheCurve> {
        if y.pow(bigint!(2_usize)) != x.pow(bigint!(3_usize)) + &*A * &x + &*B {
            return Err(PointNotInTheCurve);
        }

        Ok(Point::Point(x, y))
    }
    pub fn new_infinity() -> Self {
        Point::Infinity
    }

    pub fn new_point_from_ref(
        x: &FieldElement,
        y: &FieldElement,
    ) -> Result<Self, PointNotInTheCurve> {
        if y.pow(bigint!(2_usize)) != x.pow(bigint!(3_usize)) + &*A * x + &*B {
            return Err(PointNotInTheCurve);
        }

        Ok(Point::Point(x.clone(), y.clone()))
    }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, other_point: Point) -> Self {
        match (&self, &other_point) {
            (Self::Infinity, _) => other_point,
            (_, Self::Infinity) => self,
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && y1 != y2 => {
                Self::new_infinity()
            }
            (Self::Point(x1, y1), Self::Point(x2, y2))
                if x1 == x2 && y1 == y2 && y1 == &felt!(0_usize) =>
            {
                Self::Infinity
            }
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 != x2 => {
                let slope = (y2 - y1) / (x2 - x1);
                let x3 = slope.pow(bigint!(2_usize)) - (x1 - x2);
                let y3 = slope * (x1 - &x3) - y1;
                Self::Point(x3, y3)
            }
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && y1 == y2 => {
                println!("SLOPE I");
                let slope =
                    felt!(3_usize) * x1.pow(bigint!(2_usize)) + &*A / &(&felt!(2_usize) * y1);
                println!("SLOPE II");
                let x3 = slope.pow(bigint!(2_usize)) - &felt!(2_usize) * x1;
                let y3 = slope * (x1 - &x3) - y1;
                Self::Point(x3, y3)
            }

            _ => panic!("Can not handle field element addition"),
        }
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other_point: &Point) -> Point {
        match (self, other_point) {
            (Point::Infinity, _) => {
                println!("ADD MATCH 1");
                other_point.clone()
            }

            (_, Point::Infinity) => {
                println!("ADD MATCH 2");
                self.clone()
            }
            (Point::Point(x1, y1), Point::Point(x2, y2)) if x1 == x2 && y1 != y2 => {
                println!("ADD MATCH 3");
                Point::new_infinity()
            }
            (Point::Point(x1, y1), Point::Point(x2, y2))
                if x1 == x2 && y1 == y2 && y1 == &felt!(0_usize) =>
            {
                println!("ADD MATCH 4");
                Point::Infinity
            }
            (Point::Point(x1, y1), Point::Point(x2, y2)) if x1 != x2 => {
                println!("ADD MATCH 5");
                let slope = (y2 - y1) / (x2 - x1);
                let x3 = slope.pow(bigint!(2_usize)) - (x1 - x2);
                let y3 = slope * (x1 - &x3) - y1;
                Point::Point(x3, y3)
            }
            (Point::Point(x1, y1), Point::Point(x2, y2)) if x1 == x2 && y1 == y2 => {
                println!("ADD MATCH 6");

                println!("SLOPE I");
                // println!("numerador: {:?}", felt!(3) * x1.pow(bigint!(2)) );
                println!("numerador: {:?}", &*A);
                println!("denominador: {:?}", &(&felt!(2_usize) * y1));
                let slope =
                    felt!(3_usize) * x1.pow(bigint!(2_usize)) + &*A / &(&felt!(2_usize) * y1);
                println!("SLOPE II");
                let x3 = slope.pow(bigint!(2_usize)) - &felt!(2_usize) * x1;
                let y3 = slope * (x1 - &x3) - y1;
                Point::Point(x3, y3)
            }

            _ => panic!("Can not handle field element addition"),
        }
    }
}

impl Mul<usize> for Point {
    type Output = Self;

    fn mul(self, scalar: usize) -> Self {
        assert!(scalar != 0, "Cant multiply by 0");

        // Naive implementation
        // for _ in 1..scalar  {
        //     result = result + self;
        //     println!("mult result: {:?}", result);
        // }

        let mut current = self;
        let mut result = Point::new_infinity();
        let mut coef = scalar;
        while coef != 0 {
            if coef & 1 != 0 {
                result = &result + &current;
            }
            current = &current + &current;
            coef >>= 1;
        }
        result
    }
}

impl Mul<usize> for &Point {
    type Output = Point;

    fn mul(self, scalar: usize) -> Point {
        assert!(scalar != 0, "Cant multiply by 0");

        let mut current = self.clone();
        let mut result = Point::new_infinity();
        let mut coef = scalar;
        while coef != 0 {
            if coef & 1 != 0 {
                result = &result + &current;
            }
            current = &current + &current;
            coef >>= 1;
        }
        result
    }
}

impl Mul<&BigInt> for &Point {
    type Output = Point;

    fn mul(self, scalar: &BigInt) -> Point {
        assert!(!scalar.is_zero(), "Cant multiply by 0");

        let mut current = self.clone();
        let mut result = Point::new_infinity();
        let mut coef = scalar.clone();
        while !&coef.is_zero() {
            println!("coef: {:?}", coef.clone());
            println!("coef & 1: {:?}", coef.clone() & bigint!(1_usize));
            if !((&coef & bigint!(1_usize)).is_zero()) {
                println!("if 1");
                result = &result + &current;
                println!("if 2");
            }
            println!("a");
            current = &current + &current;
            println!("b");
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod point_tests {
    use crate::{
        constants::{G, N},
        felt, point, point_str, felt_str,
    };

    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(
            &*G + &point_str!(b"89565891926547004231252920425935692360644145829622209833684329913297188986597", b"12158399299693830322967808612713398636155367887041628176798871954788371653930"),
            point_str!(b"112711660439710606056748659173929673102114977341539408544630613555209775888121", b"25583027980570883691656905877401976406448868254816295069919888960541586679410")


        )
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(
            &*G * &bigint!(1245_usize),
            point_str!(b"16216027976566519154773591522616661330055461199350620083574692312367257363719", b"38836197644273850215659763828493766955110790713669041342445922559152527550892")
        )
    }

    #[test]
    fn test_generator_point() {
        let result = &*G * &*N;
        assert_eq!(result, Point::Infinity)
    }

    // #[test]
    // fn point_addition() {
    //     let point1 = point!(170, 142);
    //     let point2 = point!(60, 139);
    //     assert_eq!(point1 + point2, point!(220, 181));

    //     let point3 = point!(47, 71);
    //     let point4 = point!(17, 56);
    //     assert_eq!(point3 + point4, point!(215, 68));

    //     let point5 = point!(143, 98);
    //     let point6 = point!(76, 66);
    //     assert_eq!(point5 + point6, point!(47, 71));
    // }

    // #[test]
    // fn point_scalar_multiplication() {
    //     let point1 = point!(192, 105);
    //     assert_eq!(point1 * 2, point!(49, 71));

    //     let point2 = point!(143, 98);
    //     assert_eq!(point2 * 2, point!(64, 168));

    //     let point3 = point!(47, 71);
    //     assert_eq!(&point3 * 2, point!(36, 111));

    //     assert_eq!(&point3 * 4, point!(194, 51));

    //     assert_eq!(&point3 * 8, point!(116, 55));

    //     assert_eq!(&point3 * 21, Point::new_infinity());
    // }
}
