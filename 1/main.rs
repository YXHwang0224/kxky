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

        if args.is_empty() {
            eprintln!("错误：未提供命令");
            continue;
        }

        if args[0] == "exit" {
            println!("退出程序");
            break;
        }

        if args[0] == "cd" {
            let target_dir = if args.len() < 2 {
                dirs::home_dir().unwrap_or(env::current_dir().unwrap())
            } else {
                std::path::PathBuf::from(&args[1])
            };

            if let Err(err) = env::set_current_dir(&target_dir) {
                eprintln!("错误：{}", err);
            }
            continue;
        }

        // Check for mkdir command
        if args[0] == "mkdir" {
            if args.len() < 2 {
                eprintln!("错误：未提供目录名");
                continue;
            }
            let result = Command::new("mkdir")
                                .arg(&args[1])
                                .spawn();
            match result {
                Ok(_) => println!("目录创建成功"),
                Err(e) => eprintln!("无法创建目录：{}", e),
            }
            continue;
        }

        // Check for vim command
        if args[0] == "vim" {
            let result = Command::new("vim")
                                .spawn();
            match result {
                Ok(_) => (),
                Err(e) => eprintln!("无法启动vim：{}", e),
            }
            continue;
        }

        print!("收到的命令和参数为：");
        io::stdout().flush().unwrap();
        for arg in args.iter() {
            println!("{}", arg);
        }

        match args[0].as_str() {
            "greet" => {
                if args.len() < 2 {
                    eprintln!("错误：未提供名字参数");
                    continue;
                }
                println!("Hello, {}!", args[1]);
            },
            _ => {
                eprintln!("错误：未知命令 '{}'", args[0]);
            }
        }

        print!("\r");
        io::stdout().flush().unwrap();
    }
}
