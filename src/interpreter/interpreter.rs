use super::tokenizer::{self, Token};

use std::collections::btree_map::Range;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::object::number::operation::Add;
use crate::object::number::operation::Div;
use crate::object::number::operation::Mul;
use crate::object::number::operation::Sub;
use crate::object::number::rational_number::RationalNumber;
use crate::object::number::value::Value;

use crate::object::number::integer::Integer;

use crate::object::vector::vector::NumericalVector;

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        return Interpreter {
            variables: HashMap::new(),
        };
    }

    pub fn interpret(&mut self, code: &str) -> String {
        let mut output = String::from("");

        for line in code.lines() {
            // ここで行ごとに処理する
            let token_list = Box::new(tokenizer::TokenList::new(&line.to_string()));

            if token_list.get_token(0).get_word() == "let"
                && token_list.get_token(2).get_word() == "="
            {
                // 変数宣言
                let variable_name = &*token_list.get_token(1).get_word().clone();
                let value: RationalNumber = self.evaluate(&token_list.get_vec()[3..].to_vec());

                let variable_value = Value::Number(value);

                self.variables
                    .insert(variable_name.to_string(), variable_value);

                output.push_str(&String::from(format!("{}=", variable_name)));
                for token in token_list.get_vec()[3..].to_vec() {
                    output.push_str(&String::from(format!("{}", token)));
                }
            } else if token_list.get_token(0).get_word() == "eval" {
                // 値を評価する
                let value: &RationalNumber = &self.evaluate(&token_list.get_vec()[1..].to_vec());

                for token in token_list.get_vec()[1..].to_vec() {
                    output.push_str(&String::from(format!("{}", token)));
                }
                output.push_str(&String::from(format!("={}", value)));
            } else if token_list.get_token(0).get_word() == "vec" {
                // ベクトルを宣言する
                let name = token_list.get_token(1).get_word();

                // ベクトルをパース
                let mut temp_vec: Vec<RationalNumber> = Vec::new();
                let mut dim = 0;
                let mut start_idx = 4;
                for i in 4..token_list.get_vec().len()-1 {
                    if token_list.get_token(i).get_word() == "," {
                        let value = self.evaluate(token_list.get_slice(start_idx, i).get_vec());
                        
                        temp_vec.push(value);
                        dim += 1;
                        start_idx = i + 1;
                    }
                }
                let value = self.evaluate(token_list.get_slice(start_idx, token_list.get_vec().len()-1).get_vec());
                dim += 1;
                temp_vec.push(value);

                // 変数リストに入れる
                let mut vec = NumericalVector::new(dim);
                for i in 0..dim {
                    vec.set_value(i, temp_vec[i]);
                }
                self.variables.insert(name.clone(), Value::Vector(vec.clone()));

                output.push_str(&String::from(format!("{}={}", name, vec)));

            } else {
                // 何も当てはまらない場合はとりあえずそのまま出す
                output.push_str(line);
            }

            output.push_str("\n");
        }
        return output;
    }

    fn evaluate(&self, tokens: &Vec<Token>) -> RationalNumber {
        let n = tokens.len();
        if n == 0 {
            // 何もないとき仮に0を返しておく
            return RationalNumber::from(&Integer { value: 0 });
        }

        if n == 1 {
            let token = &tokens[0];

            if self.variables.contains_key(token.get_word()) {
                // 変数の場合
                return self.eval_variable(token.get_word());
            } else {
                // 数値の場合
                return RationalNumber::from(&tokens[0].to_integer());
            }
        }

        // 括弧に囲まれてる場合
        if tokens[0].get_word() == "(" && tokens[tokens.len() - 1].get_word() == ")" {
            return self.evaluate(&tokens[1..tokens.len() - 1].to_vec());
        }

        // 括弧の数をカウントしておく（右から見ていくため、閉じ括弧で+1,開き括弧で-1）
        let mut bracket_count: i32 = 0;

        // +, -を処理
        for i in (0..n).rev() {
            let token = &tokens[i];

            if token.get_word() == ")" {
                bracket_count += 1;
            } else if token.get_word() == "(" {
                bracket_count -= 1;
            }

            if bracket_count != 0 {
                continue;
            }

            if token.get_word() == "+" {
                let lhs = self.evaluate(&tokens[0..i].to_vec());
                let rhs = self.evaluate(&tokens[i + 1..n].to_vec());
                return lhs + rhs;
            } else if token.get_word() == "-" {
                let lhs = self.evaluate(&tokens[0..i].to_vec());
                let rhs = self.evaluate(&tokens[i + 1..n].to_vec());
                return lhs - rhs;
            }
        }

        // *, / を処理
        for i in (0..n).rev() {
            let token = &tokens[i];

            if token.get_word() == ")" {
                bracket_count += 1;
            } else if token.get_word() == "(" {
                bracket_count -= 1;
            }

            if bracket_count != 0 {
                continue;
            }

            if token.get_word() == "*" {
                let lhs = self.evaluate(&tokens[0..i].to_vec());
                let rhs = self.evaluate(&tokens[i + 1..n].to_vec());
                return lhs * rhs;
            } else if token.get_word() == "/" {
                let lhs = self.evaluate(&tokens[0..i].to_vec());
                let rhs = self.evaluate(&tokens[i + 1..n].to_vec());
                return lhs / rhs;
            }
        }

        // 最後まで来てしまったら0を返しておく
        return RationalNumber::from(&Integer { value: 0 });
    }

    fn eval_variable(&self, name: &String) -> RationalNumber {
        let value = self.variables.get(name).unwrap();
        match value {
            Value::Number(num) => {
                return *num;
            }
            Value::Vector(vec) => {
                return RationalNumber::from(0);
            }
            Value::Integer(num) => {
                return RationalNumber::from(0);
            }
        }
    }
}
