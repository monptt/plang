use super::super::object::ObjectTrait;

pub struct Integer {
    pub value: i32
}

impl ObjectTrait for Integer {
    fn to_string(&self) -> String {
        let value = self.value;
        let str = value.to_string();
        return str;
    }
}