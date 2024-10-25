use super::value::Value;

pub trait Add {
    fn add(self, x :Self) -> Self;
}

pub trait Sub {
    fn sub(self, x :Self) -> Self;
}

pub trait Mul {
    fn mul(self, x :Self) -> Self;
}

pub trait Div {
    fn div(self, x :Self) -> Self;
}
