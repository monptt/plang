use super::tokenizer;

use std::collections::HashMap;
use crate::object::number::value;
use crate::object::number::integer::Integer;

pub fn interpret(code: &str) -> String {
    let mut output = String::from("");
    
    // 宣言された変数を格納するハッシュマップ
    let mut variables: HashMap<&str, Box<Integer>> = std::collections::HashMap::new();

    for line in code.lines(){
        // ここで行ごとに処理する
        let tokens = tokenizer::TokenList::new(line).tokens;

        if tokens[0] == "let" && tokens[2] == "="{
            // 変数宣言
            let variable_name = tokens[1];
            let value: Integer = evaluate(tokens[3..].to_vec(), &variables);
            variables.insert(variable_name, Box::new(value));


            output.push_str(&String::from(format!("{}=", variable_name)));
            for token in tokens[3..].to_vec() {
                output.push_str(&String::from(format!("{}", token)));
            }
        }
        else if tokens[0] == "eval"{
            // 値を評価する
            let variable_name = tokens[1];
            let value: &Integer = &evaluate(tokens[1..].to_vec(), &variables);

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

fn evaluate(tokens: Vec<&str>, variables: &HashMap<&str, Box<Integer>>) -> Integer{
    let mut ret_value: Integer = Integer{value: 0};

    let mut i = 0;
    while i < tokens.len(){
        let token = tokens[i];

        if token == "+" {
            i += 1;
            let num = token_to_integer(tokens[i], variables);
            ret_value = Integer::add(ret_value, num);
        }
        else if token == "-" {
            i += 1;
            let num = token_to_integer(tokens[i], variables);
            ret_value = Integer::sub(ret_value, num);
        }
        else if token == "*" {
            i += 1;
            let num = token_to_integer(tokens[i], variables);
            ret_value = Integer::mul(ret_value, num);
        }
        else if token == "/" {
            i += 1;
            let num = token_to_integer(tokens[i], variables);
            ret_value = Integer::div(ret_value, num);
        }
        else{
            ret_value = token_to_integer(token, variables);
        }

        i = i + 1;
    }
    return ret_value;
}

fn token_to_integer(token: &str, variables: &HashMap<&str, Box<Integer>>) -> Integer{
    if variables.contains_key(token){
        return **variables.get(token).unwrap();
    }else{
        return Integer::new(token);
    }
}
