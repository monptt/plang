use super::tokenizer;

use std::collections::HashMap;
use crate::object::number;

pub fn interpret(code: &str) -> String {
    let mut output = String::from("");
    
    // 宣言された変数を格納するハッシュマップ
    let mut variables: HashMap<&str, number::Number> = std::collections::HashMap::new();

    for line in code.lines(){
        // ここで行ごとに処理する
        let tokens = tokenizer::TokenList::new(line).tokens;

        if tokens[0] == "let" && tokens[2] == "="{
            // 変数宣言
            let variable_name = tokens[1];
            let value: number::Number = evaluate(tokens[3..].to_vec(), &variables);
            variables.insert(variable_name, value);


            output.push_str(&String::from(format!("{}=", variable_name)));
            for token in tokens[3..].to_vec() {
                output.push_str(&String::from(format!("{}", token)));
            }
        }
        else if tokens[0] == "eval"{
            // 値を評価する
            let variable_name = tokens[1];
            let value: &number::Number = &evaluate(tokens[1..].to_vec(), &variables);

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

fn evaluate(tokens: Vec<&str>, variables: &HashMap<&str, number::Number>) -> number::Number{
    let mut ret_value: number::Number = number::Number{value: 0};

    let mut i = 0;
    while i < tokens.len(){
        let token = tokens[i];

        if token == "+" {
            i += 1;
            let num = token_to_number(tokens[i], variables);
            ret_value = number::Number::add(ret_value, num);
        }
        else if token == "-" {
            i += 1;
            let num = token_to_number(tokens[i], variables);
            ret_value = number::Number::sub(ret_value, num);
        }
        else if token == "*" {
            i += 1;
            let num = token_to_number(tokens[i], variables);
            ret_value = number::Number::mul(ret_value, num);
        }
        else if token == "/" {
            i += 1;
            let num = token_to_number(tokens[i], variables);
            ret_value = number::Number::div(ret_value, num);
        }
        else{
            ret_value = token_to_number(token, variables);
        }

        i = i + 1;
    }
    return ret_value;
}

fn token_to_number(token: &str, variables: &HashMap<&str, number::Number>) -> number::Number{
    if variables.contains_key(token){
        return *variables.get(token).unwrap();
    }else{
        return number::Number::new(token);
    }
}
