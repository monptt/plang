use std::fmt;

use crate::object::number::integer::Integer;

#[derive(Clone)]
pub struct Token{
    word: String
}

impl Token {
    fn new(word: String) -> Token{
        return Token{word: word};
    }

    pub fn get_word(&self) -> &String{
        return &self.word;
    }

    pub fn to_integer(&self) -> Integer{
        return Integer::new(&self.get_word());
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.word);
    }
}

pub struct TokenList{
    tokens : Vec<Token>
}

impl TokenList{
    pub fn new(line : &str) -> TokenList{
        let mut token_list = TokenList{tokens: Vec::new()};

        let words: Vec<_> = line.split(" ").collect();

        for word in words{
            let token = Token::new(word.to_string());
            token_list.tokens.push(token);
        }

        return token_list;
    }

    pub fn get_token(&self, index: usize) -> &Token{
        return &self.tokens.get(index).unwrap();
    }

    pub fn get_vec(&self) -> &Vec<Token>{
        return &self.tokens;
    }
}