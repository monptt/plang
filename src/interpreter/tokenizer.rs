pub struct Token{
    word: String
}

impl Token {
    fn new(word: String) -> Token{
        return Token{word: word};
    }

    fn get_word(&self) -> &String{
        return &self.word;
    }
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