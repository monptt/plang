use std::{cmp::max, ops};

use crate::object::number::{integer::Integer, rational_number::RationalNumber};

use super::{function::Function, monomial::Monomial};

// 多項式関数
struct Polynomial {
    coefficients: Vec<RationalNumber>,
}

impl Function for Polynomial {
    fn calc(&self, x: RationalNumber) -> RationalNumber {
        let mut ret_value = RationalNumber::from(0);

        let mut temp_pow = RationalNumber::from(1);

        for coefficient in &self.coefficients {
            ret_value = ret_value + *coefficient * temp_pow;
            temp_pow = temp_pow * x;
        }

        return ret_value;
    }
}

impl Polynomial {
    fn new(coefficients: Vec<RationalNumber>) -> Polynomial {
        return Polynomial {
            coefficients: coefficients,
        };
    }

    fn get_degree(&self) -> Integer {
        return Integer::from(self.coefficients.len() as i32);
    }
}

impl From<&Monomial> for Polynomial {
    fn from(monomial: &Monomial) -> Polynomial {
        let mut coefficients: Vec<RationalNumber> = Vec::new();
        for i in 0..monomial.degree.value + 1 {
            if i == monomial.degree.value {
                coefficients.push(monomial.coefficient);
            } else {
                coefficients.push(RationalNumber::from(0));
            }
        }

        return Polynomial::new(coefficients);
    }
}

impl ops::Add for Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: Self) -> Polynomial {
        let degree = max(self.get_degree().value, rhs.get_degree().value);
        let mut ret_coefficients: Vec<RationalNumber> = Vec::new();
        for i in 0..degree {
            let mut coefficitent = RationalNumber::from(0);
            if i < self.get_degree().value {
                coefficitent = coefficitent + self.coefficients[i as usize];
            }
            if i < rhs.get_degree().value {
                coefficitent = coefficitent + rhs.coefficients[i as usize];
            }
            ret_coefficients.push(coefficitent);
        }

        return Polynomial::new(ret_coefficients);
    }
}