use super::value::Value;

pub trait Add {
    fn add(&self, x :Self) -> Box<dyn Value>;
}

pub trait Sub {
    fn sub(&self, x :Self) -> Box<dyn Value>;
}

pub trait Mul {
    fn mul(&self, x :Self) -> Box<dyn Value>;
}

pub trait Div {
    fn div(&self, x :Self) -> Box<dyn Value>;
}
