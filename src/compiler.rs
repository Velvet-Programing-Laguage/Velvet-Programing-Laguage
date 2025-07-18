use crate::ast::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn compile(statements: Vec<Statement>) -> Result<(), String> {
    let mut output = File::create("velvet_out.rs").map_err(|e| e.to_string())?;
    writeln!(output, "use std::collections::HashMap;")?;
    writeln!(output, "fn main() {{")?;
    writeln!(output, "    let mut env: HashMap<String, f64> = HashMap::new();")?;
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
        Statement::Say(expr) => {
            writeln!(output, "{}println!(\"{{}}\", {});", indent_str, compile_expr(expr)?)
                .map_err(|e| e.to_string())?;
        }
        Statement::Val(ident, expr, type_anno) => {
            let type_str = type_anno.as_deref().unwrap_or("f64");
            if let Some(e) = expr {
                writeln!(output, "{}let mut {}: {} = {};", indent_str, ident, type_str, compile_expr(e)?)
                    .map_err(|e| e.to_string())?;
            } else {
                writeln!(output, "{}let mut {}: {};", indent_str, ident, type_str)
                    .map_err(|e| e.to_string())?;
            }
        }
        Statement::Const(ident, expr, type_anno) => {
            let type_str = type_anno.as_deref().unwrap_or("f64");
            writeln!(output, "{}const {}: {} = {};", indent_str, ident, type_str, compile_expr(expr)?)
                .map_err(|e| e.to_string())?;
        }
        Statement::Fun(name, params, body) => {
            writeln!(output, "{}fn {}(", indent_str, name)?;
            for (i, (param, type_anno)) in params.iter().enumerate() {
                write!(output, "{}{}: {}", indent_str, param, type_anno)?;
                if i < params.len() - 1 {
                    write!(output, ", ")?;
                }
            }
            writeln!(output, ") {{")?;
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
        Statement::Break => {
            writeln!(output, "{}break;", indent_str)?;
        }
        Statement::Continue => {
            writeln!(output, "{}continue;", indent_str)?;
        }
        Statement::Try(_, try_block, catch_block) => {
            writeln!(output, "{}/* Try block */", indent_str)?;
            for stmt in try_block {
                compile_stmt(output, stmt, indent + 1)?;
            }
            writeln!(output, "{}/* Catch block */", indent_str)?;
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
        Statement::Expr(expr) => {
            writeln!(output, "{}{};", indent_str, compile_expr(expr)?)?;
        }
        Statement::Return(expr) => {
            writeln!(output, "{}return {};", indent_str, compile_expr(expr)?)?;
        }
        Statement::Import(module) => {
            let module_path = format!("{}.velvet", module);
            if Path::new(&module_path).exists() {
                writeln!(output, "{}// Including module: {}", indent_str, module)?;
            } else {
                return Err(format!("Module '{}' not found", module));
            }
        }
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
        Expr::Binary(left, op, right) => Ok(format!(
            "({} {} {})",
            compile_expr(left)?,
            op,
            compile_expr(right)?
        )),
        Expr::Unary(op, expr) => Ok(format!("{}{}", op, compile_expr(expr)?)),
        Expr::Call(name, args) => {
            let mut arg_str = String::new();
            for (i, arg) in args.iter().enumerate() {
                arg_str.push_str(&compile_expr(arg)?);
                if i < args.len() - 1 {
                    arg_str.push_str(", ");
                }
            }
            Ok(format!("{}({})", name, arg_str))
        }
        Expr::List(elements) => {
            let mut list_str = String::new();
            list_str.push_str("vec![");
            for (i, elem) in elements.iter().enumerate() {
                list_str.push_str(&compile_expr(elem)?);
                if i < elements.len() - 1 {
                    list_str.push_str(", ");
                }
            }
            list_str.push(']');
            Ok(list_str)
        }
    }
}
