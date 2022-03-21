enum TokenType {
  ILLEGAL, // 未知の場合
  EOF, // ファイルの終端

  IDENT,
  INT,

  ASSIGN, // =
  PLUS, // +

  COMMA, // ,
  SEMICOLON, // ;

  LPAREN, // (
  RPAREN, // )
  LBRAKET, // {
  RBRAKET, // }

  FUNCTION, // FUNCTION
  LET // LET
}


struct Token {
  token_type: TokenType,
  literal: String,
}