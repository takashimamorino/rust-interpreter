#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL, // 未知の場合
    EOF,     // ファイルの終端

    IDENT,
    INT,

    ASSIGN, // =
    PLUS,   // +

    COMMA,     // ,
    SEMICOLON, // ;

    LPAREN,  // (
    RPAREN,  // )
    LBRAKET, // {
    RBRAKET, // }

    FUNCTION, // FUNCTION
    LET,      // LET
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
