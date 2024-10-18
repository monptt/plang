use super::object::ObjectTrait;

struct Proposition {}

impl Proposition {
    fn eval() -> TruthValue {
        return TruthValue { value: true };
    }
}

struct TruthValue {
    value: bool,
}

impl ObjectTrait for TruthValue {
    fn to_string(&self) -> String {
        return String::from(if self.value { "True" } else { "False" });
    }
}
