use std::fmt;
use super::operation;

use super::integer::Integer;
use super::rational_number::RationalNumber;
use super::super::vector::vector::NumericalVector;

#[derive(Clone)]
pub enum Value {
    Number(RationalNumber),
    Integer(Integer),
    Vector(NumericalVector)
}

pub trait AsValue {
    fn as_value(&self) -> Value;
}