use core::fmt;

use crate::object::number::rational_number::RationalNumber;

use super::function::FunctionTrait;

#[derive(Clone)]
pub struct Constant{
    pub value: RationalNumber
}

impl Constant {
    fn new(num: RationalNumber) -> Constant {
        return Constant{
            value: num
        };
    }
}

impl FunctionTrait for Constant {
    fn calc(&self, x: RationalNumber) -> RationalNumber {
        return self.value;
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.value);
    }
}