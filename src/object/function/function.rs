use core::fmt;

use crate::object::{morphism::Morphism, number::rational_number::RationalNumber};

use super::monomial::Monomial;
use super::polynomial::Polynomial;

#[derive(Clone)]
pub enum Function {
    Monomial(Monomial),
    Polynomial(Polynomial),
}

impl FunctionTrait for Function {
    fn calc(&self, x: RationalNumber) -> RationalNumber {
        match self {
            Function::Monomial(func) => {
                return func.calc(x);
            }
            Function::Polynomial(func) => {
                return func.calc(x);
            }
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Monomial(func) => {
                return func.fmt(f);
            }
            Function::Polynomial(func) => {
                return func.fmt(f);
            }
        }
    }
}

pub trait FunctionTrait: fmt::Display {
    fn calc(&self, x: RationalNumber) -> RationalNumber;
}

impl Morphism for dyn FunctionTrait {}
