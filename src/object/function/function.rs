use core::fmt;

use crate::object::{morphism::Morphism, number::rational_number::RationalNumber};

use super::constant::Constant;
use super::monomial::Monomial;
use super::polynomial::Polynomial;

#[derive(Clone)]
pub enum Function {
    Constant(Constant),
    Monomial(Monomial),
    Polynomial(Polynomial),
}

impl Function {
    fn get_func(&self) -> Box<dyn FunctionTrait> {
        match self {
            Function::Constant(func) => {
                return Box::new(func.clone());
            }
            Function::Monomial(func) => {
                return Box::new(func.clone());
            }
            Function::Polynomial(func) => {
                return Box::new(func.clone());
            }
        }
    }
}

impl FunctionTrait for Function {
    fn calc(&self, x: RationalNumber) -> RationalNumber {
        return self.get_func().calc(x);
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Constant(func) => {
                return func.fmt(f);
            }
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
