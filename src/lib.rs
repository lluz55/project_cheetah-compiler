#![allow(dead_code)]
mod lexer;
mod tokens;

#[cfg(test)]
mod tests {

  use lexer::Lexer;
  use super::*;

  #[test]
  fn parse_lexer() {
    static CONTENT: &'static str = r#"$@Name { 
      *height: $parent.width
      !value:   "empty"
      onClick: (ev) {}
    }"#;


    let mut lexer = Lexer::new(CONTENT);
    let tokens = lexer.cellectt_tokens();

    assert_eq!(tokens[0].value, "$");
    assert_eq!(tokens[1].value, "@");
    assert_eq!(tokens[2].value, "Name");
    assert_eq!(tokens[3].value, "{");
    assert_eq!(tokens[4].value, "|");
    assert_eq!(tokens[5].value, "*");
    assert_eq!(tokens[6].value, "height");
    assert_eq!(tokens[7].value, ":");
    assert_eq!(tokens[8].value, "$");
    assert_eq!(tokens[9].value, "parent");
    assert_eq!(tokens[10].value, ".");
    assert_eq!(tokens[11].value, "width");
    assert_eq!(tokens[12].value, "|");
    assert_eq!(tokens[13].value, "!");
    assert_eq!(tokens[14].value, "value");
    assert_eq!(tokens[15].value, ":");
    assert_eq!(tokens[16].value, "empty");
    assert_eq!(tokens[17].value, "|");
    assert_eq!(tokens[18].value, "onClick");
    assert_eq!(tokens[19].value, ":");
    assert_eq!(tokens[20].value, "(");
    assert_eq!(tokens[21].value, "ev");
    assert_eq!(tokens[22].value, ")");
    assert_eq!(tokens[23].value, "{");
    assert_eq!(tokens[24].value, "}");
    assert_eq!(tokens[25].value, "|");
    assert_eq!(tokens[26].value, "}");
        
  }
}
