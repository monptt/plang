mod tokenizer;
use std::collections::HashMap;

pub fn interpret(code: &str) -> String {
    let mut output = String::from("");
    
    // 宣言された変数を格納するハッシュマップ
    let mut variables: HashMap<&str, i32> = std::collections::HashMap::new();

    for line in code.lines(){
        // ここで行ごとに処理する
        let tokens = tokenizer::TokenList::new(line).tokens;

        if tokens[0] == "let"{

        }
        else if tokens[0] == "eval"{
            
        }


        for token in tokens{
            output.push_str(token);
            output.push_str("\n");
        }
    }
    return output;
}
