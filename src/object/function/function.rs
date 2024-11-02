use crate::object::{morphism::Morphism, number::rational_number::RationalNumber};

pub enum Function {
    Monomial,
    Polynomial
}

pub trait FunctionTrait {
    fn calc(&self, x: RationalNumber) -> RationalNumber;
}

impl Morphism for dyn FunctionTrait {
    
}