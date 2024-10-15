mod tokenizer;

pub fn interpret(code: &str) -> String {
    let mut output = String::from("");
    
    for line in code.lines(){
        // ここで行ごとに処理する
        let _token_list = tokenizer::TokenList::new(line);

        for token in _token_list.tokens{
            output.push_str(token);
            output.push_str("\n");
        }
    }
    return output;
}
