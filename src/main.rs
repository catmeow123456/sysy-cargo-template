use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{Write, Result};

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy

lalrpop_mod!(sysy);
mod ast;

fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  let mode = args.next().unwrap();

  if mode == "-koopa" {
    let input = args.next().unwrap();
    let mode = args.next().unwrap();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

    // 输出解析得到的 AST
    println!("{:#?}", ast);

    let result = ast.dump();
    // let result = "unknown";
    println!("{}", result);
    if mode == "-o" {
      let mut output = File::create(output)?;
      writeln!(output, "{}", result)?;
    }
  }
  Ok(())
}
