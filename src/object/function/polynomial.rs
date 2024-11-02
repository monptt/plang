use crate::object::number::rational_number::RationalNumber;

use super::function::Function;

// 多項式関数
struct Polynomial {
    coefficients: Vec<RationalNumber>,
}

impl Function for Polynomial {
    fn map(&self, x: RationalNumber) -> RationalNumber {
        let mut ret_value = RationalNumber::from(0);

        let mut temp_pow = RationalNumber::from(1);

        for coefficient in &self.coefficients {
            ret_value = ret_value + *coefficient * temp_pow;
            temp_pow = temp_pow * x;
        }

        return ret_value;
    }
}
