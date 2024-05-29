#[derive(Debug)]
pub struct CompUnit {
  pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
  pub func_type: FuncType,
  pub ident: String,
  pub block: Block,
}

#[derive(Debug)]
pub struct Block {
  pub stmt: Stmt,
}

#[derive(Debug)]
pub enum FuncType {
  Int,
}

#[derive(Debug)]
pub struct Stmt {
  pub num: i32,
}

impl CompUnit {
  pub fn dump(&self) -> String {
    self.func_def.dump()
  }
}

impl FuncDef {
  pub fn dump(&self) -> String {
    format!("fun @{}(): {} {{\n{}\n}}",
      self.ident,  
      self.func_type.dump(),
      self.block.dump())
  }
}

impl Block {
  pub fn dump(&self) -> String {
    format!("%entry:\n{}", self.stmt.dump())
  }
}

impl FuncType {
  pub fn dump(&self) -> String {
    match self {
      FuncType::Int => "i32".to_string(),
    }
  }
}

impl Stmt {
  pub fn dump(&self) -> String {
    format!("  ret {}", self.num)
  }
}