use std::env;
use std::process::{Command, exit};
use std::io::{self, Write}; 

fn main() {
    loop {
        let current_dir = env::current_dir().unwrap_or_else(|_| "/".into());

        print!("{} $ ", current_dir.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取输入");

        let args: Vec<String> = input.trim().split_whitespace().map(String::from).collect();

        
        print!("\r");
        io::stdout().flush().unwrap();
    }
}
