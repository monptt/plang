use super::symbol::SymbolName;
use super::tokenizer::TokenList;
use super::tokenizer::{self, Token};

use std::collections::HashMap;

use crate::object::number::operation::Add;
use crate::object::number::operation::Div;
use crate::object::number::operation::Mul;
use crate::object::number::operation::Sub;
use crate::object::number::rational_number::RationalNumber;
use crate::object::number::value::{self, Value};

use crate::object::number::integer::Integer;
use crate::object::vector::vector::NumericalVector;

use crate::object::function::function::{Function, FunctionTrait};
use crate::object::function::monomial::Monomial;

pub struct Interpreter {
    variables: HashMap<SymbolName, Value>,
    functions: HashMap<SymbolName, Function>,
    output: String,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        return Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
            output: String::from(""),
        };
    }

    pub fn interpret(&mut self, code: &str) -> String {
        for line in code.lines() {
            // ここで行ごとに処理する
            let token_list = tokenizer::TokenList::new(&line.to_string());

            let first_token = token_list.get_token(0);

            if *first_token == Token::Let && *token_list.get_token(2) == Token::Eq {
                // 変数宣言
                self.assign_variable(&token_list);
            } else if *first_token == Token::Eval {
                // 値を評価する
                let eval_token_list = token_list.get_slice(1, token_list.get_length());

                let value: &RationalNumber = &self.evaluate(&token_list.get_vec()[1..].to_vec());

                for token in token_list.get_vec()[1..].to_vec() {
                    self.output.push_str(&String::from(format!("{}", token)));
                }
                self.output.push_str(&String::from(format!("={}", value)));
            } else if *first_token == Token::Vec && *token_list.get_token(2) == Token::Eq {
                self.parse_vector(&token_list);
            } else if *first_token == Token::Func && *token_list.get_token(2) == Token::Eq {
                // 関数を定義
                self.assign_function(&token_list);
            } else if *first_token == Token::Calc {
                // 関数の値を計算
                let func_name = SymbolName::FunctionName(token_list.get_token(1).get_word());
                let func = self.functions.get(&func_name).unwrap();
                let arg = Interpreter::eval_number(&token_list.get_token(3));
                let result = func.calc(arg);
                // 出力
                self.output.push_str(&String::from(format!("{} ( {} ) = {}", func_name, arg, result)));
            } else {
                // 何も当てはまらない場合はとりあえずそのまま出す
                self.output.push_str(line);
            }

            self.output.push_str("\n");
        }
        return self.output.clone();
    }

    fn assign_function(&mut self, token_list: &TokenList) {
        let function_name = SymbolName::FunctionName(token_list.get_token(1).get_word());

        let func_token_list = token_list.get_slice(3, token_list.get_length());

        let func: Function = self.parse_function(&func_token_list);
        self.functions
            .insert(function_name.clone(), func.clone());

        // 出力
        self.output
            .push_str(&String::from(format!("{}={}", function_name, func)));
    }

    fn parse_function(&self, token_list: &TokenList) -> Function {
        let arg_char = "x";

        if token_list.get_length() == 1 {
            let token = token_list.get_token(0);
            if token.get_word() == arg_char {
                return Function::Monomial(Monomial::new(RationalNumber::from(1), Integer::from(1)));
            } else {
                let num = Interpreter::eval_number(&token);
                return Function::Monomial(Monomial::new(num, Integer::from(0)));
            }
        }

        if token_list.get_length() == 3 && *token_list.get_token(1) == Token::Pow {
            let degree = Interpreter::eval_number(&token_list.get_token(2));
            return Function::Monomial(Monomial::new(RationalNumber::from(1), degree.numerator));
        }

        for i in (0..token_list.get_length()).rev() {
            let token = token_list.get_token(i);

            // +,-を処理
            if *token == Token::Plus {
                let lhs = self.parse_function(&token_list.get_slice(0, i));
                let rhs =
                    self.parse_function(&token_list.get_slice(i + 1, token_list.get_length()));
                return lhs; // + rhs; //todo: 実装
            } else if *token == Token::Minus {
                let lhs = self.parse_function(&token_list.get_slice(0, i));
                let rhs =
                    self.parse_function(&token_list.get_slice(i + 1, token_list.get_length()));
                return lhs; // - rhs; //todo: 実装
            }
        }
        return Function::Monomial(Monomial::new(RationalNumber::from(1), Integer::from(0)));
    }

    fn assign_variable(&mut self, token_list: &TokenList) {
        let variable_name = SymbolName::VariableName(token_list.get_token(1).get_word());
        let variable_value = Value::Number(self.evaluate(&token_list.get_vec()[3..].to_vec()));

        self.variables
            .insert(variable_name.clone(), variable_value.clone());

        // 出力
        self.output
            .push_str(&String::from(format!("{}=", variable_name)));
        for token in token_list.get_vec()[3..].to_vec() {
            self.output.push_str(&String::from(format!("{}", token)));
        }
    }

    fn evaluate(&self, tokens: &Vec<Token>) -> RationalNumber {
        let n = tokens.len();
        if n == 0 {
            // 何もないとき仮に0を返しておく
            return RationalNumber::from(&Integer { value: 0 });
        }

        if n == 1 {
            let token = &tokens[0];
            if self
                .variables
                .contains_key(&SymbolName::VariableName(token.get_word()))
            {
                // 変数の場合
                return self.eval_variable(&SymbolName::VariableName(token.get_word()));
            } else {
                // 数値の場合
                return Self::eval_number(&token);
            }
        }

        // 括弧に囲まれてる場合
        if tokens[0] == Token::BracketLeft && tokens[tokens.len() - 1] == Token::BracketRight {
            return self.evaluate(&tokens[1..tokens.len() - 1].to_vec());
        }

        // 括弧の数をカウントしておく（右から見ていくため、閉じ括弧で+1,開き括弧で-1）
        let mut bracket_count: i32 = 0;

        // +, -を処理
        for i in (0..n).rev() {
            let token = &tokens[i];

            if *token == Token::BracketRight {
                bracket_count += 1;
            } else if *token == Token::BracketLeft {
                bracket_count -= 1;
            }

            if bracket_count != 0 {
                continue;
            }

            if *token == Token::Plus {
                let lhs = self.evaluate(&tokens[0..i].to_vec());
                let rhs = self.evaluate(&tokens[i + 1..n].to_vec());
                return lhs + rhs;
            } else if *token == Token::Minus {
                let lhs = self.evaluate(&tokens[0..i].to_vec());
                let rhs = self.evaluate(&tokens[i + 1..n].to_vec());
                return lhs - rhs;
            }
        }

        // *, / を処理
        for i in (0..n).rev() {
            let token = &tokens[i];

            if *token == Token::BracketRight {
                bracket_count += 1;
            } else if *token == Token::BracketLeft {
                bracket_count -= 1;
            }

            if bracket_count != 0 {
                continue;
            }

            if *token == Token::Mul {
                let lhs = self.evaluate(&tokens[0..i].to_vec());
                let rhs = self.evaluate(&tokens[i + 1..n].to_vec());
                return lhs * rhs;
            } else if *token == Token::Div {
                let lhs = self.evaluate(&tokens[0..i].to_vec());
                let rhs = self.evaluate(&tokens[i + 1..n].to_vec());
                return lhs / rhs;
            }
        }

        // 最後まで来てしまったら0を返しておく
        return RationalNumber::from(&Integer { value: 0 });
    }

    fn eval_variable(&self, name: &SymbolName) -> RationalNumber {
        let value = self.variables.get(name).unwrap();
        match value {
            Value::Number(num) => {
                return *num;
            }
            Value::Vector(vec) => {
                return RationalNumber::from(0);
            }
            Value::Integer(num) => {
                return RationalNumber::from(num);
            }
        }
    }

    fn eval_number(in_token: &Token) -> RationalNumber {
        return RationalNumber::from(&in_token.get_word());
    }

    fn parse_vector(&mut self, token_list: &TokenList) {
        // ベクトルを宣言する
        let name = SymbolName::VariableName(token_list.get_token(1).get_word());

        // ベクトルをパース
        let mut temp_vec: Vec<RationalNumber> = Vec::new();
        let mut dim = 0;
        let mut start_idx = 4;
        for i in 4..token_list.get_vec().len() - 1 {
            if *token_list.get_token(i) == Token::Comma {
                let value = self.evaluate(token_list.get_slice(start_idx, i).get_vec());

                temp_vec.push(value);
                dim += 1;
                start_idx = i + 1;
            }
        }
        let value = self.evaluate(
            token_list
                .get_slice(start_idx, token_list.get_vec().len() - 1)
                .get_vec(),
        );
        dim += 1;
        temp_vec.push(value);

        // 変数リストに入れる
        let mut vec = NumericalVector::new(dim);
        for i in 0..dim {
            vec.set_value(i, temp_vec[i]);
        }
        self.variables
            .insert(name.clone(), Value::Vector(vec.clone()));

        self.output
            .push_str(&String::from(format!("{}={}", name, vec)));
    }
}

#[cfg(test)]
mod tests {
    use super::super::symbol::SymbolName;
    use super::Interpreter;
    use crate::object::function::function::Function;
    use crate::object::function::function::FunctionTrait;
    use crate::object::function::monomial::Monomial;
    use crate::object::number::integer::Integer;
    use crate::object::number::rational_number::RationalNumber;

    #[test]
    fn test_variable_assign() {
        let mut interpreter = Interpreter::new();
        interpreter.interpret("let x = 1");

        let value = interpreter.eval_variable(&SymbolName::VariableName(String::from("x")));
        let expected_value = RationalNumber::from(1);
        assert!(value == expected_value);
    }

    #[test]
    fn test_function_assign() {
        let mut interpreter = Interpreter::new();
        interpreter.interpret("func f = x");

        let func = interpreter
            .functions
            .get(&SymbolName::FunctionName(String::from("f")))
            .unwrap();
        let expected_func = Monomial::new(RationalNumber::from(1), Integer::from(1));

        let arg = RationalNumber::from(3);
        assert!(func.calc(arg) == expected_func.calc(arg));
    }
}
