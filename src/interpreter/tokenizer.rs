enum Token{

}

pub struct TokenList{
    tokens : Vec<Token>
}

impl TokenList{
    pub fn new(input : Vec<char>) -> TokenList{
        let tokens = Vec::<Token>::new();
        for _c in &input{

        }
        return TokenList{tokens : tokens};
    }
}