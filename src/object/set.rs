use crate::object::object;

use super::object::ObjectTrait;

pub trait SetTrait : object::ObjectTrait{

}

pub struct Set{

}

impl  ObjectTrait for Set {
    fn to_string(&self) -> String {
        return String::from("Set");
    }
}

impl SetTrait for Set{

}