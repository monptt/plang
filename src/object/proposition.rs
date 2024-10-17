struct Proposition {}

impl Proposition {
    fn eval() -> TruthValue{
        return TruthValue{value:true};
    }
}

struct TruthValue {
    value: bool
}