#![allow(unused)]
use crate::{ast::*, tokens::*};

pub struct Parser<'a>{
  current_index: usize,
  tokens: &'a mut Vec<LexerToken>,
}

impl<'a> Parser<'a> {
  pub fn new(tokens: &'a mut Vec<LexerToken>) -> Self {
    Self{
      current_index: 0,
      tokens
    }
  }

  pub fn parse_tokens(&mut self, mut scope: &mut Stmt){
    while self.current_index < self.tokens.len() {
      self.parse_token(&mut scope, false);
    }
    
  }

  fn parse_token(&mut self, mut scope: &mut Stmt, inside_struct: bool){
    match self.tokens[self.current_index].tk_type {
      TokenType::DOLLAR => {
       match self.tokens[self.current_index+1].tk_type {
        TokenType::AT => {
          if let TokenType::ID = self.tokens[self.current_index+2].tk_type {
            self.current_index+= 2;
            self.component_definition(&mut scope);
          } 
        },
        TokenType::ID => {
          self.built_in_method(&mut scope);
        },
        _ => {}
      }
      },

      _ => {}
    }
    self.current_index += 1;
  }

  fn parse_inheritance(&mut self, mut scope: &mut Stmt) {

  }

  fn component_definition(&mut self, mut scope: &mut Stmt) {
    if let TokenType::COLON = self.tokens[self.current_index+1].tk_type {
      self.advance();
      self.parse_inheritance(&mut scope);
    }


  }

  fn built_in_method(&mut self, scope: &mut Stmt) {

  }

  fn add_stmt_to_scope(scope: &mut Stmt, stmt: &mut Stmt){

  }


  fn get_last_token(&mut self) -> LexerToken{
    todo!()
  }

  fn get_current_token(&mut self) -> &mut LexerToken{
    &mut self.tokens[self.current_index]
  }

  fn advance(&mut self)-> bool{
    if self.current_index < self.tokens.len() {
      self.current_index+=1;
      return true
    }
    false
  }

  fn retreat(&mut self)-> bool{
    false
  }

  fn must_advance(&mut self){

  }

  fn must_advance_count(&mut self, count: usize){

  }

  fn var_const_field_decl(&mut self, scope: &mut Stmt,is_struct_field: bool,is_const: bool){

  }

  fn error_expecting(&mut self, expected: &str){

  }

  fn error_msg(&mut self, msg: &str){

  }

  fn function_call(&mut self, scope: Stmt){

  }

  fn get_definition_token(&mut self) -> LexerToken{
    todo!()
  }

  fn function_declaration(&mut self, scope: &mut Stmt,is_method: bool){

  }

  fn struct_declaration(&mut self, scope: &mut Stmt){

  }

  fn is_public(name: String)-> bool{
    false
  }

  fn is_infered_float(name: String)-> bool{
    false
  }
}