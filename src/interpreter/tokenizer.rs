use std::fmt;

use crate::object::number::integer::Integer;

#[derive(Clone)]
pub struct Token {
    word: String,
}

impl Token {
    pub fn new(word: &String) -> Token {
        return Token { word: word.clone() };
    }

    pub fn get_word(&self) -> &String {
        return &self.word;
    }

    pub fn to_integer(&self) -> Integer {
        return Integer::new(&self.get_word());
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.word);
    }
}

pub struct TokenList {
    tokens: Vec<Token>,
}

impl TokenList {
    pub fn new(line: &String) -> TokenList {
        let mut token_list = TokenList { tokens: Vec::new() };

        // とりあえずスペースで区切る
        let words: Vec<_> = line.split(" ").collect();

        for word in words {
            let mut temp_token = String::from("");
            for c in word.chars() {
                if "=+-*/()[],".contains(c) {
                    // 記号が来たらその時点でトークン化
                    if temp_token.len() > 0 {
                        let token = Token::new(&temp_token);
                        token_list.tokens.push(token);
                    }

                    // 記号もトークン化
                    token_list.tokens.push(Token::new(&c.to_string()));
                    // 文字列をリセット
                    temp_token = String::from("");
                } else {
                    // それ以外は文字列に突っ込む
                    temp_token.push_str(c.to_string().as_str());
                }
            }

            // 残ってたらトークン化
            if temp_token.len() > 0 {
                let token = Token::new(&temp_token);
                token_list.tokens.push(token);
            }
        }

        return token_list;
    }

    pub fn get_token(&self, index: usize) -> &Token {
        return &self.tokens.get(index).unwrap();
    }

    pub fn get_vec(&self) -> &Vec<Token> {
        return &self.tokens;
    }
}
