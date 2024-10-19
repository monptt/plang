use super::tokenizer::{self, Token};

use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::object::number::value::Value;

use crate::object::number::integer::Integer;

pub struct Interpreter{
    variables: HashMap<String, Rc<Integer>>
}

impl Interpreter{
    pub fn new() -> Interpreter{
        return Interpreter{variables: HashMap::new()};
    }

    pub fn interpret(&mut self, code: &str) -> String {
        let mut output = String::from("");
            
        for line in code.lines(){
            // ここで行ごとに処理する
            let token_list = Box::new(tokenizer::TokenList::new(line));
    
            if token_list.get_token(0).get_word() == "let" && token_list.get_token(2).get_word() == "="{
                // 変数宣言
                let variable_name = &*token_list.get_token(1).get_word().clone();
                let value: Integer = self.evaluate(token_list.get_vec()[3..].to_vec(), &self.variables);
                self.variables.insert(variable_name.to_string(), Rc::new(value));
    
    
                output.push_str(&String::from(format!("{}=", variable_name)));
                for token in token_list.get_vec()[3..].to_vec() {
                    output.push_str(&String::from(format!("{}", token)));
                }
            }
            else if token_list.get_token(0).get_word() == "eval"{
                // 値を評価する
                let variable_name = token_list.get_token(1).get_word();
                let value: &Integer = &self.evaluate(token_list.get_vec()[1..].to_vec(), &self.variables);
    
                output.push_str(&String::from(format!("{}={}", variable_name, value)));
            }
            else{
                // 何も当てはまらない場合はとりあえずそのまま出す
                output.push_str(line);
            }
    
            output.push_str("\n");
        }
        return output;
    }
    
    fn evaluate(&self, tokens: Vec<Token>, variables: &HashMap<String, Rc<Integer>>) -> Integer{
        let mut ret_value: Integer = Integer{value: 0};
    
        let mut i = 0;
        while i < tokens.len(){
            let token = &tokens[i];
    
            if variables.contains_key(token.get_word()){
                return **variables.get(token.get_word()).unwrap();
            }
            else if token.get_word() == "+" {
                i += 1;
                let num = self.token_to_integer(tokens[i].get_word(), variables);
                ret_value = Integer::add(ret_value, num);
            }
            else if token.get_word() == "-" {
                i += 1;
                let num = self.token_to_integer(tokens[i].get_word(), variables);
                ret_value = Integer::sub(ret_value, num);
            }
            else if token.get_word() == "*" {
                i += 1;
                let num = self.token_to_integer(tokens[i].get_word(), variables);
                ret_value = Integer::mul(ret_value, num);
            }
            else if token.get_word() == "/" {
                i += 1;
                let num = self.token_to_integer(tokens[i].get_word(), variables);
                ret_value = Integer::div(ret_value, num);
            }
            else{
                return Integer::new(&token.get_word());
            }
            
            i = i + 1;
        }
        return ret_value;
    }
    
    fn token_to_integer(&self, token: &str, variables: &HashMap<String, Rc<Integer>>) -> Integer{
        if variables.contains_key(token){
            return **variables.get(token).unwrap();
        }else{
            return Integer::new(token);
        }
    }
}

