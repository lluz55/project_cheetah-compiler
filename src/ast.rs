pub struct Stmt {
  stmt_type:  AstType,
  stmts:      Vec<Box<Stmt>>,
}

impl Stmt {
  pub fn new(stmt_type: AstType) -> Self{
    Self{
      stmt_type, stmts: vec![]}
  }  
}


pub enum AstType {
  NONE,
  VARIABLEDEFINITION ,
  CONSTANTDEFINITION ,
  FUNCTIONDECLARATION,
  FUNCTIONCALL       ,
  VARIABLE           ,
  EXPR               ,
  STRUCTDEFINITION   ,
  METHODDECLARATION  ,
  STRUCTFIELD        ,

  GLOBALSCOPE        ,
  COMPONENT{name: String},
  PROPERTY{name: String, is_pub: bool},
  COMPONENTDEF,

} 