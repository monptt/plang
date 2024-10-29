use std::fmt;
use super::operation;

use super::rational_number::RationalNumber;
use super::super::vector::vector::NumericalVector;

#[derive(Clone)]
pub enum Value {
    Number(RationalNumber),
    Vector(NumericalVector)
}