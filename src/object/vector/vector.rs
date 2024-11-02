use std::ops;
use std::cmp;
use std::fmt;

use crate::object::number::{rational_number::RationalNumber, value};

// 
#[derive(Clone)]
pub struct NumericalVector {
    dimension: usize,
    vec: Vec<RationalNumber>,
}

impl NumericalVector {
    pub fn new(dim: usize) -> NumericalVector {
        let mut ret_vec = NumericalVector {
            dimension: dim,
            vec: Vec::with_capacity(dim),
        };

        for _ in 0..dim {
            let value_zero = RationalNumber::from(0);
            ret_vec.vec.push(value_zero);
        }
        return ret_vec;
    }

    pub fn set_value(&mut self, idx: usize, value: RationalNumber){
        self.vec[idx] = value;
    }

    pub fn get_value(&self, i: usize) -> RationalNumber{
        return self.vec[i];
    }
}

impl cmp::PartialEq for NumericalVector {
    fn eq(&self, other: &Self) -> bool {
        if self.dimension != other.dimension {
            return false;
        }

        for i in 0..self.dimension {
            if self.get_value(i) != other.get_value(i) {
                return false;
            }
        }

        return  true;
    }
}

impl fmt::Display for NumericalVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::from("");
        for i in 0..self.dimension {
            if i != 0 {
                str += ",";
            }
            str += &self.vec[i].to_string();
        }
        return write!(f, "[{}]_{{\\in \\mathbb{{R}} ^ {{ {} }} }}", str, self.dimension);
    }
}

#[cfg(test)]
mod tests {
    use crate::object::number::rational_number::RationalNumber;

    use super::NumericalVector;

    #[test]
    fn test_new_vector() {
        // ベクトルの初期化テスト
        let n = 3;
        let vec = NumericalVector::new(n);
        for i in 0..n {
            assert!(vec.get_value(i) == RationalNumber::from(0));
        }
    }

    #[test]
    fn test_vector_eq() {
        let n = 3;
        let mut vec_a = NumericalVector::new(n);
        let mut vec_b = NumericalVector::new(n);
        let mut vec_c = NumericalVector::new(n);

        vec_a.set_value(0, RationalNumber::from(1));
        vec_a.set_value(1, RationalNumber::from(2));
        vec_a.set_value(2, RationalNumber::from(3));

        vec_b.set_value(0, RationalNumber::from(1));
        vec_b.set_value(1, RationalNumber::from(2));
        vec_b.set_value(2, RationalNumber::from(3));

        vec_c.set_value(0, RationalNumber::from(3));
        vec_c.set_value(1, RationalNumber::from(4));
        vec_c.set_value(2, RationalNumber::from(5));

        assert!(vec_a == vec_b);
        assert!(vec_a != vec_c);
    }
}
