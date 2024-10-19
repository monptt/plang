use super::operation;
use super::value::Value;

pub struct RationalNumber {
    numerator: i32,
    denominator: i32,
}

impl Value for RationalNumber {}

impl operation::Add for RationalNumber {
    fn add(&self, x :Self) -> Box<dyn Value> {
        return Box::new(RationalNumber {
            numerator: 0,
            denominator: 0,
        });
    }
}

impl operation::Sub for RationalNumber {
    fn sub(&self, x :Self) -> Box<dyn Value> {
        return Box::new(RationalNumber {
            numerator: 0,
            denominator: 0,
        });
    }
}

impl operation::Mul for RationalNumber {
    fn mul(&self, x :Self) -> Box<dyn Value> {
        return Box::new(RationalNumber {
            numerator: 0,
            denominator: 0,
        });
    }
}

impl operation::Div for RationalNumber {
    fn div(&self, x :Self) -> Box<dyn Value> {
        return Box::new(RationalNumber {
            numerator: 0,
            denominator: 0,
        });
    }
}
