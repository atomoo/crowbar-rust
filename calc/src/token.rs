#[allow(dead_code)]

#[derive(Debug)]
pub enum TokenKind {
    BadToken,
    NumberToken,
    AddOperatorToken,
    SubOperatorToken,
    MulOperatorToken,
    DivOperatorToken,
    LeftParenToken,
    RightParenToken,
    EndOfLineToken
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: f64,
    pub str: String
}

pub const MAX_TOKEN_SIZE: i32 = 100;

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::BadToken,
            value: 0.0,
            str: "".to_string()
        }
    }
}
