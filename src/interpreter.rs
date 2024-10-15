mod tokenizer;

pub fn interpret(code: &str) -> String {
    let mut output = String::from("");
    
    for line in code.lines(){
        output.push_str(line);

        // ここで行ごとに処理する
        let _token_list = tokenizer::TokenList::new(line.chars().collect());

        output.push_str("\n");
    }
    return output;
}
