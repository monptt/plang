// 数ベクトル
struct NumericalVector{
    dimension: i32
}

impl NumericalVector{
    fn new(dim: i32) -> NumericalVector {
        return NumericalVector{dimension: dim};
    }
}

#[cfg(test)]
mod tests{
    use super::NumericalVector;

    #[test]
    fn test_new_vector(){
        let n = 3;
        NumericalVector::new(n);
    }
}