use std::env;

#[derive(Debug)]
enum CrustErr {
    Result(String),
}

fn main() -> Result<(), CrustErr> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(CrustErr::Result("too long or short argument".to_string()));
    }
    let num = args[1].parse::<i64>().unwrap();
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", num);
    println!("  ret");
    Ok(())
}
