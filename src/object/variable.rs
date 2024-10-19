use crate::object::number::value::Value;

use super::number::integer::Integer;

pub struct Variable {
    pub value: Box<dyn Value>
}

impl Variable {
    fn new(value: Box<Integer>) -> Variable{
        return  Variable{value: value};
    }
}