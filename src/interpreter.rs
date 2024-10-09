pub fn interpret(code: &str) -> String {
    let n = code.parse::<i32>().unwrap();

    let mut output = String::from("");
    for _i in 0..n{
        output.push_str("にゃっはろー\n");
    }

    return output;
}
