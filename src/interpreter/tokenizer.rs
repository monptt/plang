enum Token{

}

pub struct TokenList<'a>{
    pub tokens : Vec<&'a str>
}

impl TokenList<'_>{
    pub fn new(line : &str) -> TokenList{
        let words: Vec<_> = line.split(" ").collect();
        return TokenList{tokens : words};
    }
}