use super::tokenizer::{self, Token};

use std::collections::HashMap;
use std::rc::Rc;

use crate::object::number::value::Value;

use crate::object::number::integer::Integer;

pub fn interpret(code: &str) -> String {
    let mut output = String::from("");
    
    // 宣言された変数を格納するハッシュマップ
    let mut variables: HashMap<String, Rc<Integer>> = std::collections::HashMap::new();

    for line in code.lines(){
        // ここで行ごとに処理する
        let token_list = Box::new(tokenizer::TokenList::new(line));

        if token_list.get_token(0).get_word() == "let" && token_list.get_token(2).get_word() == "="{
            // 変数宣言
            let variable_name = &*token_list.get_token(1).get_word().clone();
            let value: Integer = evaluate(token_list.get_vec()[3..].to_vec(), &variables);
            variables.insert(variable_name.to_string(), Rc::new(value));


            output.push_str(&String::from(format!("{}=", variable_name)));
            for token in token_list.get_vec()[3..].to_vec() {
                output.push_str(&String::from(format!("{}", token)));
            }
        }
        else if token_list.get_token(0).get_word() == "eval"{
            // 値を評価する
            let variable_name = token_list.get_token(1).get_word();
            let value: &Integer = &evaluate(token_list.get_vec()[1..].to_vec(), &variables);

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

fn evaluate(tokens: Vec<Token>, variables: &HashMap<String, Rc<Integer>>) -> Integer{
    let mut ret_value: Integer = Integer{value: 0};

    let mut i = 0;
    while i < tokens.len(){
        let token = &tokens[i];

        if token.get_word() == "+" {
            i += 1;
            let num = token_to_integer(tokens[i].get_word(), variables);
            ret_value = Integer::add(ret_value, num);
        }
        else if token.get_word() == "-" {
            i += 1;
            let num = token_to_integer(tokens[i].get_word(), variables);
            ret_value = Integer::sub(ret_value, num);
        }
        else if token.get_word() == "*" {
            i += 1;
            let num = token_to_integer(tokens[i].get_word(), variables);
            ret_value = Integer::mul(ret_value, num);
        }
        else if token.get_word() == "/" {
            i += 1;
            let num = token_to_integer(tokens[i].get_word(), variables);
            ret_value = Integer::div(ret_value, num);
        }
        else if variables.contains_key(token.get_word()){
            return **variables.get(token.get_word()).unwrap();
        }else{
            return Integer::new(&token.get_word());
        }
        
        i = i + 1;
    }
    return ret_value;
}

fn token_to_integer(token: &str, variables: &HashMap<String, Rc<Integer>>) -> Integer{
    if variables.contains_key(token){
        return **variables.get(token).unwrap();
    }else{
        return Integer::new(token);
    }
}
