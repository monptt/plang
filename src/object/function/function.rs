use crate::object::number::rational_number::RationalNumber;

pub trait Function {
    fn map(&self, x: RationalNumber) -> RationalNumber;
}