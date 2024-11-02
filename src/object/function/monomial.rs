use crate::object::number::{integer::Integer, rational_number::RationalNumber};

pub struct Monomial {
    coefficient: RationalNumber, // 係数
    degree: Integer // 次数
}

impl Monomial {
    pub fn new(coefficient: RationalNumber, degree: Integer) -> Monomial {
        return Monomial{
            coefficient: coefficient,
            degree: degree
        };
    }
}