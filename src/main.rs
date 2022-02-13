use std::env;

#[derive(Debug)]
enum CrustErr {
    Result(String),
}

#[derive(Clone)]
enum TokenKind<'a> {
    Symbol(&'a str),
    Num(&'a str),
}

fn tokenize(vals: Vec<String>) -> Vec<String> {
    let program_text = vals.join("");
    let tokens: Vec<String> = program_text
        .replace("+", " + ")
        .replace("-", " - ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();
    return tokens;
}
    
fn main() -> Result<(), CrustErr> {

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    let mut tokens = tokenize(args);
    tokens = write_asm(&tokens)?.to_vec();
    loop {
        if tokens.is_empty() {
            break;
        }
        tokens = write_asm(&tokens)?.to_vec();
    }
    println!("  ret");
    Ok(())
}

fn write_asm(tokens: &[String]) -> Result<&[String], CrustErr> {

    let (token, other) = tokens.split_first().ok_or(
        CrustErr::Result("error parsing".to_string())
    )?;
    if !is_numeric(token) {
        let (val, rest) = other.split_first().ok_or(
            CrustErr::Result("error parsing".to_string())
        )?;
        match &token[..] {
            "+" => write_asm_for_unary_op(TokenKind::Symbol("+"), val),
            "-" => write_asm_for_unary_op(TokenKind::Symbol("-"), val),
            _ => Err(CrustErr::Result("failed to fetch value".to_string())),
        };
        Ok(rest)
    }
    else {
        write_asm_for_single_value(token);
        Ok(other)
    }
}

fn write_asm_for_single_value (val: &str) {
   println!("  mov rax, {}", val);
}

fn write_asm_for_unary_op (token_kind: TokenKind, val: &str) -> Result<(), CrustErr> {

    match token_kind {
        TokenKind::Symbol(op) => {
            match op {
                "+" => println!("  add rax, {}", val),
                "-" => println!("  sub rax, {}", val),
                _ => {
                    return Err(CrustErr::Result("consumed operator crust doesnt know".to_string()));
                }
            }
        },
        _ => {
            return Err(CrustErr::Result("consumed value crust doesnt know".to_string()));
        },
    }
    Ok(())
}

fn is_numeric(val: &str) -> bool {
    
    let mut flag: bool = false;
    for s in val.chars() {
        flag = s.is_numeric() || flag;
    }
    return flag;
}

