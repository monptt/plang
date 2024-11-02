use crate::object::{morphism::Morphism, number::rational_number::RationalNumber};

pub trait Function {
    fn calc(&self, x: RationalNumber) -> RationalNumber;
}

impl Morphism for dyn Function {
    
}