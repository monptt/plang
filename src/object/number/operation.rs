use super::value::Value;

pub trait Add {
    fn add(x :Self) -> Box<dyn Value>;
}

pub trait Sub {
    fn sub(x :Self) -> Box<dyn Value>;
}

pub trait Mul {
    fn mul(x :Self) -> Box<dyn Value>;
}

pub trait Div {
    fn div(x :Self) -> Box<dyn Value>;
}
