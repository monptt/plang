mod tokenizer;
use std::collections::HashMap;

pub fn interpret(code: &str) -> String {
    let mut output = String::from("");
    
    // 宣言された変数を格納するハッシュマップ
    let mut variables: HashMap<&str, i32> = std::collections::HashMap::new();

    for line in code.lines(){
        // ここで行ごとに処理する
        let tokens = tokenizer::TokenList::new(line).tokens;

        if tokens[0] == "let" && tokens[2] == "="{
            // 変数宣言
            let variable_name = tokens[1];
            let value: i32 = tokens[3].parse().unwrap();
            variables.insert(variable_name, value);

            output.push_str(&String::from(format!("{}={}", variable_name, value)));
        }
        else if tokens[0] == "eval"{
            // 値を評価する
            let variable_name = tokens[1];
            let value: i32 = *variables.get(variable_name).unwrap();

            output.push_str(&String::from(format!("{}={}", variable_name, value)));
        }

        output.push_str("\n");
    }
    return output;
}
