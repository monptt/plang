use crate::object::{morphism::Morphism, number::rational_number::RationalNumber};

pub trait Function {
    fn map(&self, x: RationalNumber) -> RationalNumber;
}

impl Morphism for dyn Function {
    
}