#[allow(unused)]
pub enum TokenType {
  ID,        // 0
  STRING,    // 1
  COLON,     // 2
  SEMI,      // 3
  LPAREN,    // 4
  RPAREN,    // 5
  DOT,       // 6
  LESST,     // 7
  GREAT,     // 8
  NEWLINE,   // 9
  EQUALS,    // 10
  LCBRA,     // 11
  RCBRA,     // 12
  LSBRA,     // 13
  RSBRA,     // 14
  MINUS,     // 15
  PLUS,      // 16
  TIMES,     // 17
  NUMBER,    // 18
  EOF,       // 19
  COMMA,     // 20
  CONSTDEF,  // 21
  VARDEF,    // 22
  AMPERSAND, // 23
  STAR,      // 24
  COMMENT,   // 25
  SLASH,     // 26
  AT,        // 27
  HASHTAG,   // 28
  DOLLAR,    // 29
  EXCLAM,    // 30
}

pub struct LexerToken {
  pub tk_type: TokenType,
  pub line: u32,
  pub col: u32,
  pub value: String,
}

impl LexerToken {
  pub fn new(tk_type: TokenType, line: u32, col: u32 , value: String) -> LexerToken {
    Self{
      line,
      col,
      tk_type,
      value,
    }
  }
}