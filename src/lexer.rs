#![allow(dead_code)]
#![allow(unused_variables)]
use crate::tokens::*;

use std::process::exit;

#[derive(Default)]
pub struct Lexer {
  line: u32,
  col: u32,
  content: &'static str,
  index: usize,
  tokens: Vec<LexerToken>,
}

impl Lexer {

  pub fn new(content: &'static str) -> Self {
    let mut lexer = Lexer::default();
    lexer.content = content;
    lexer
  }

  pub fn cellectt_tokens(&mut self) -> &mut Vec<LexerToken> {
    self.line = 1;
    self.col = 1;

    while self.index < self.content.len() {
      let i = self.index;

      match self.content.chars().nth(i).unwrap() {  
        '"' => self.collect_string(),
        ' ' => {
          self.skip_white_space();
          continue;
        },
        '\n' => {
          self.add_token(TokenType::NEWLINE, "|".to_string());
          self.line+=1;
          self.col = 1;
          self.advance();
        },
        c @ '.' => self.add_token_and_advance(TokenType::DOT, c.to_string()),
        c @ ',' => self.add_token_and_advance(TokenType::COMMA, c.to_string()),
        c @ ':' => self.add_token_and_advance(TokenType::COLON, c.to_string()),
        c @ ';' => self.add_token_and_advance(TokenType::SEMI, c.to_string()),
        c @ '=' => self.add_token_and_advance(TokenType::EQUALS, c.to_string()),
        c @ '(' => self.add_token_and_advance(TokenType::LPAREN, c.to_string()),
        c @ ')' => self.add_token_and_advance(TokenType::RPAREN, c.to_string()),
        c @ '{' => self.add_token_and_advance(TokenType::LCBRA, c.to_string()),
        c @ '}' => self.add_token_and_advance(TokenType::RCBRA, c.to_string()),
        c @ '[' => self.add_token_and_advance(TokenType::LSBRA, c.to_string()),
        c @ ']' => self.add_token_and_advance(TokenType::RSBRA, c.to_string()),
        c @ '<' => self.add_token_and_advance(TokenType::LESST, c.to_string()),
        c @ '>' => self.add_token_and_advance(TokenType::GREAT, c.to_string()),
        c @ '-' => self.add_token_and_advance(TokenType::MINUS, c.to_string()),
        c @ '+' => self.add_token_and_advance(TokenType::PLUS, c.to_string()),
        c @ '&' => self.add_token_and_advance(TokenType::AMPERSAND, c.to_string()),
        c @ '*' => self.add_token_and_advance(TokenType::STAR, c.to_string()),
        c @ '@' => self.add_token_and_advance(TokenType::AT, c.to_string()),
        c @ '#' => self.add_token_and_advance(TokenType::HASHTAG, c.to_string()),
        c @ '$' => self.add_token_and_advance(TokenType::DOLLAR, c.to_string()),
        c @ '!' => self.add_token_and_advance(TokenType::EXCLAM, c.to_string()),
        
        c => {
          if c.is_numeric() {
            self.collect_number();
          }

          if c.is_alphabetic() || c == '_' {
            self.collect_id();
          }

          else {
            println!("not handled char: {}. {}:{}", c, self.line, self.col);
            exit(1);
          }
        }        
      }
    }

    &mut self.tokens 
  }


  fn add_token(&mut self, tk_type: TokenType, value: String) {
    if self.col == 1 {
      self.col+=1;
    }
    self.tokens.push(LexerToken::new(tk_type, self.line, self.col+1, value));
  }

  fn add_token_and_advance(&mut self, tk_type: TokenType, value: String) { 
    self.add_token(tk_type, value);
    self.advance();
  }

  fn advance(&mut self) {
    self.index += 1;
  }

  fn skip_white_space(&mut self) {
    while self.index < self.content.len() {
      if self.get_current_char() == ' ' {
        self.col += 1;
        self.advance();
      } else {
        break;
      }
    }
  }

  fn collect_line_comment(&mut self) {
    self.advance();
    while self.index < self.content.len() && self.get_current_char() != '\n' {
      self.advance();
    }
    self.add_token(TokenType::COMMENT, "#".to_string())
  }

  fn has_next_char(&mut self) -> bool {
    todo!()
  }

  fn collect_string(&mut self) {
    self.advance();
    let mut value = String::from("");
    self.col +=1;

    while self.index < self.content.len() && self.get_current_char() != '"' {
      let c = self.get_current_char();
      if c == '\n' {
        println!("Cannot use new line in single line string. {}:{}", self.line, self.col);
        // TODO: improve error handling
        exit(1);
      }
      value.push(c);
      self.advance();
    }

    self.col += 1;
    self.add_token(TokenType::STRING, value);
    self.advance();
  }

  fn get_current_char(&mut self) -> char {
    let tmp = match self.content.chars().nth(self.index) {
      Some(c) => c,
      None => {
        println!("'Lexer.get_current_char': Unexpected end of file. {}:{}", self.line, self.col); 
        '§'
      }        
    };
    tmp
  }

  fn collect_id(&mut self) {
    let mut value = String::from("");

    while self.index < self.content.len() &&
      self.get_current_char().is_alphanumeric() ||
      self.get_current_char() == '_' {
        value.push(self.get_current_char());
        self.col +=1;
        self.advance();
    }

    self.add_token(TokenType::ID, value);
  }

  fn collect_number(&mut self) {
    let mut has_dot = false;
    let mut value = String::from("");

    while self.index < self.content.len() &&
      self.get_current_char().is_numeric() ||
      self.get_current_char() == '_' ||
      self.get_current_char() == '.' {
        let c = self.get_current_char();
        if c == '_' {
          self.advance();
          self.col += 1;
          continue;
        }

        if c == '.' {
          if has_dot {
            println!("Connot have more then one 'dot' in number representation. {}:{}", self.line, self.col);
            // TODO: improve error handling
            exit(1);
          }
          has_dot = true;
        }

        self.col += 1;
        value.push(c);
        self.advance();
    }

    self.add_token(TokenType::NUMBER, value);
  }

  fn retreat(&mut self) {

  }

  fn clean(&mut self) {

  }   
}