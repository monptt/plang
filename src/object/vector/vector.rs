use std::collections::btree_map::Range;

use crate::object::number::{rational_number::RationalNumber, value};

// 数ベクトル
struct NumericalVector {
    dimension: usize,
    vec: Vec<RationalNumber>,
}

impl NumericalVector {
    fn new(dim: usize) -> NumericalVector {
        let mut ret_vec = NumericalVector {
            dimension: dim,
            vec: Vec::with_capacity(dim),
        };

        for i in 0..dim {
            let value_zero = RationalNumber::from(0);
            ret_vec.vec.push(value_zero);
        }
        return ret_vec;
    }

    fn set_value(&mut self, idx: usize, value: RationalNumber){
        self.vec[idx] = value;
    }

    fn get_value(&self, i: usize) -> RationalNumber{
        return self.vec[i];
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
}
