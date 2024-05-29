#[derive(Debug)]
pub struct Info {
  pub context: String,
  pub expr: String,
  pub id_begin: i32,
  pub id_end: i32,
}

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
  pub exp: Box<Exp>,
}

#[derive(Debug)]
pub enum Exp {
  Number { num: i32 },
  UnaryExp { op: UnaryOp, exp: Box<Exp> },
  BinaryExp { lhs: Box<Exp>, op: BinaryOp, rhs: Box<Exp> },
}

#[derive(Debug)]
pub enum UnaryOp {
  Pos,
  Neg,
  Not,
}

#[derive(Debug)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
}

impl CompUnit {
  pub fn dump(&self) -> String {
    self.func_def.dump()
  }
}

impl FuncDef {
  pub fn dump(&self) -> String {
    format!("fun @{}(): {} {{\n{}}}\n",
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
    let info = self.exp.dump(0);
    format!("{}  ret {}\n", info.context, info.expr)
  }
}

impl Exp {
  pub fn dump(&self, id: i32) -> Info {
    match self {
      Exp::Number { num } => 
        Info {
          context: "".to_string(),
          expr: num.to_string(),
          id_begin: id,
          id_end: id,
        },
      Exp::UnaryExp { op, ref exp } => {
        let info = exp.dump(id);
        match op {
          UnaryOp::Pos => info,
          UnaryOp::Not => Info {
            context: format!("{}  %{} = eq {}, 0\n", info.context, info.id_end, info.expr),
            expr: format!("%{}", info.id_end),
            id_begin: info.id_begin,
            id_end: info.id_end + 1,
          },
          UnaryOp::Neg => Info {
            context: format!("{}  %{} = sub 0, {}\n", info.context, info.id_end, info.expr),
            expr: format!("%{}", info.id_end),
            id_begin: info.id_begin,
            id_end: info.id_end + 1,
          },
        }
      }
      Exp::BinaryExp { ref lhs, op, ref rhs } => {
        // Info {
        //   context: "hello world".to_string(),
        //   expr: "123".to_string(),
        //   id_begin: 0,
        //   id_end: 0,
        // }
        let lhs_info = lhs.dump(id);
        let rhs_info = rhs.dump(lhs_info.id_end);
        Info {
          context: format!("{}  %{} = {} {}, {}\n",
            format!("{}{}", lhs_info.context, rhs_info.context),
            rhs_info.id_end,
            op.dump(), lhs_info.expr, rhs_info.expr),
          expr: format!("%{}", rhs_info.id_end),
          id_begin: lhs_info.id_begin,
          id_end: rhs_info.id_end + 1,
        }
      }
    }
  }
}

impl BinaryOp {
  pub fn dump(&self) -> String {
    match self {
      BinaryOp::Add => "add".to_string(),
      BinaryOp::Sub => "sub".to_string(),
      BinaryOp::Mul => "mul".to_string(),
      BinaryOp::Div => "sdiv".to_string(),
      BinaryOp::Mod => "srem".to_string(),
    }
  }
}