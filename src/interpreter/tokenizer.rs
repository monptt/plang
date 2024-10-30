use std::fmt;

use crate::object::number::integer::Integer;

#[derive(Clone, PartialEq)]
pub enum Token {
    // キーワード
    Let,
    Eval,
    Vec,
    // 記号
    Eq,
    Plus,
    Minus,
    Mul,
    Div,
    BracketLeft,
    BracketRight,
    Comma,
    // それ以外
    Word(String),
}

impl Token {
    pub fn new(word: &String) -> Token {
        // キーワード
        if word == "let" {
            return Token::Let;
        }
        if word == "eval" {
            return Token::Eval;
        }
        if word == "Vec" {
            return Token::Vec;
        }

        // 記号
        if word == "=" {
            return Token::Eq;
        }
        if word == "+" {
            return Token::Plus;
        }
        if word == "-" {
            return Token::Minus;
        }
        if word == "*" {
            return Token::Mul;
        }
        if word == "/" {
            return Token::Div;
        }
        if word == "(" {
            return Token::BracketLeft;
        }
        if word == ")" {
            return Token::BracketRight;
        }
        if word == "," {
            return Token::Comma;
        }

        // それ以外
        return Token::Word(word.clone());
    }

    pub fn get_word(&self) -> String {
        match self {
            // キーワード
            Token::Let => {
                return String::from("let");
            }
            Token::Eval => {
                return String::from("eval");
            }
            Token::Vec => {
                return String::from("vec");
            }
            // 記号
            Token::Eq => {
                return String::from("=");
            }
            Token::Plus => {
                return String::from("+");
            }
            Token::Minus => {
                return String::from("-");
            }
            Token::Mul => {
                return String::from("*");
            }
            Token::Div => {
                return String::from("/");
            }
            Token::BracketLeft => {
                return String::from("(");
            }
            Token::BracketRight => {
                return String::from(")");
            }
            Token::Comma => {
                return String::from(",");
            }
            // それ以外
            Token::Word(word) => {
                return word.clone();
            }
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.get_word());
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

    pub fn get_slice(&self, begin: usize, end: usize) -> TokenList {
        return TokenList {
            tokens: self.tokens[begin..end].to_vec(),
        };
    }

    pub fn get_vec(&self) -> &Vec<Token> {
        return &self.tokens;
    }
}
