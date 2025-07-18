use crate::ast::*;
use std::fs::File;
use std::io::Write;

pub fn compile(statements: Vec<Statement>) -> Result<(), String> {
    let mut output = File::create("velvet_out.rs")?;
    writeln!(output, "use std::collections::HashMap;\nfn main() {{ let mut env: HashMap<String, f64> = HashMap::new();")?;
    for stmt in statements {
        compile_stmt(&mut output, &stmt, 1)?;
    }
    writeln!(output, "}}")?;
    std::process::Command::new("rustc")
        .arg("velvet_out.rs")
        .arg("-o")
        .arg("velvet_out")
        .status()
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn compile_stmt(output: &mut File, stmt: &Statement, indent: usize) -> Result<(), String> {
    let indent_str = "    ".repeat(indent);
    match stmt {
        Statement::Say(expr) => writeln!(output, "{}println!(\"{{}}\", {});", indent_str, compile_expr(expr)?)?,
        Statement::Val(ident, expr, type_anno) => {
            let type_str = type_anno.as_deref().unwrap_or("f64");
            if let Some(e) = expr {
                writeln!(output, "{}let mut {}: {} = {};", indent_str, ident, type_str, compile_expr(e)?)?;
            } else {
                writeln!(output, "{}let mut {}: {};", indent_str, ident, type_str)?;
            }
        }
        Statement::Const(ident, expr, type_anno) => {
            let type_str = type_anno.as_deref().unwrap_or("f64");
            writeln!(output, "{}const {}: {} = {};", indent_str, ident, type_str, compile_expr(expr)?)?;
        }
        Statement::Fun(name, params, ret_type, body) => {
            writeln!(output, "{}fn {}(", indent_str, name)?;
            for (i, (param, type_anno)) in params.iter().enumerate() {
                write!(output, "{}{}: {}", indent_str, param, type_anno)?;
                if i < params.len() - 1 { write!(output, ", ")?; }
            }
            writeln!(output, ") {} {{", ret_type.as_deref().unwrap_or(""))?;
            for stmt in body {
                compile_stmt(output, stmt, indent + 1)?;
            }
            writeln!(output, "{}}}", indent_str)?;
        }
        Statement::If(condition, then_block, else_block) => {
            writeln!(output, "{}if {} {{", indent_str, compile_expr(condition)?)?;
            for stmt in then_block {
                compile_stmt(output, stmt, indent + 1)?;
            }
            if let Some(else_block) = else_block {
                writeln!(output, "{}}} else {{", indent_str)?;
                for stmt in else_block {
                    compile_stmt(output, stmt, indent + 1)?;
                }
            }
            writeln!(output, "{}}}", indent_str)?;
        }
        Statement::For(ident, expr, body) => {
            writeln!(output, "{}for {} in {} {{", indent_str, ident, compile_expr(expr)?)?;
            for stmt in body {
                compile_stmt(output, stmt, indent + 1)?;
            }
            writeln!(output, "{}}}", indent_str)?;
        }
        Statement::While(condition, body) => {
            writeln!(output, "{}while {} {{", indent_str, compile_expr(condition)?)?;
            for stmt in body {
                compile_stmt(output, stmt, indent + 1)?;
            }
            writeln!(output, "{}}}", indent_str)?;
        }
        Statement::Break => writeln!(output, "{}break;", indent_str)?,
        Statement::Continue => writeln!(output, "{}continue;", indent_str)?,
        Statement::Try(_, try_block, catch_block) => {
            writeln!(output, "{}// Try block", indent_str)?;
            for stmt in try_block {
                compile_stmt(output, stmt, indent + 1)?;
            }
            writeln!(output, "{}// Catch block", indent_str)?;
            for stmt in catch_block {
                compile_stmt(output, stmt, indent + 1)?;
            }
        }
        Statement::Match(expr, branches) => {
            writeln!(output, "{}match {} {{", indent_str, compile_expr(expr)?)?;
            for (pattern, statements) in branches {
                writeln!(output, "{}{} => {{", indent_str, pattern)?;
                for stmt in statements {
                    compile_stmt(output, stmt, indent + 1)?;
                }
                writeln!(output, "{}}}", indent_str)?;
            }
            writeln!(output, "{}}}", indent_str)?;
        }
        Statement::Expr(expr) => writeln!(output, "{}{};", indent_str, compile_expr(expr)?)?,
        Statement::Return(expr) => writeln!(output, "{}return {};", indent_str, compile_expr(expr)?)?,
        Statement::Import(module, _source) => writeln!(output, "{}mod {};", indent_str, module)?,
        Statement::Test(name, body) => {
            writeln!(output, "{}// Test: {}", indent_str, name)?;
            for stmt in body {
                compile_stmt(output, stmt, indent + 1)?;
            }
        }
    }
    Ok(())
}

fn compile_expr(expr: &Expr) -> Result<String, String> {
    match expr {
        Expr::String(s) => Ok(format!("\"{}\"", s)),
        Expr::Number(n) => Ok(n.to_string()),
        Expr::Bool(b) => Ok(b.to_string()),
        Expr::Ident(id) => Ok(id.clone()),
        Expr::Binary(left, op, right) => Ok(format!("({} {} {})", compile_expr(left)?, op, compile_expr(right)?)),
        Expr::Unary(op, expr) => Ok(format!("{}{}", op, compile_expr(expr)?)),
        Expr::Call(name, args) => {
            let args_str = args.iter().map(compile_expr).collect::<Result<Vec<_>, _>>()?.join(", ");
            Ok(format!("{}({})", name, args_str))
        }
        Expr::List(elements) => {
            let list_str = elements.iter().map(compile_expr).collect::<Result<Vec<_>, _>>()?.join(", ");
            Ok(format!("vec![{}]", list_str))
        }
        Expr::Index(ident, index) => Ok(format!("{}[{}]", ident, compile_expr(index)?))
    }
}
