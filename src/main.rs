#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::process::Command;
use std::path::Path;

fn main() {
    // let mut commands: HashMap<&str, Regex> = HashMap::new();
    // commands.insert("echo", generate_cmd_regex("echo"));
    // commands.insert("type", generate_cmd_regex("type"));
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input == "exit 0" {
            break;
        }
        
        if let Some((cmd, args)) = parse_command(input) {
            match cmd.as_str() {
                "echo" => println!("{}", args.join(" ")),
                "type" => println!("{}", print_cmd_type_info(&args[0])),
                _ => execute_external_command(&cmd, &args),
            }
        } else {
            println!("{}: command not found", input);
        }
    }
}

fn parse_command(input: &str) -> Option<(String, Vec<String>)> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }
    Some((parts[0].to_string(), parts[1..].iter().map(|&s| s.to_string()).collect()))
}

fn print_cmd_type_info(cmd: &str) -> String {
    match cmd {
        "echo" | "type" | "exit" => format!("{} is a shell builtin", cmd),
        _ => match find_executable_in_path(cmd) {
            Some(path) => format!("{} is {}", cmd, path),
            None => format!("{}: not found", cmd),
        },
    }
}

fn find_executable_in_path(cmd: &str) -> Option<String> {
    if let Ok(paths) = env::var("PATH") {
        for dir in paths.split(':') {
            let full_path = format!("{}/{}", dir, cmd);
            if fs::metadata(&full_path).is_ok() {
                return Some(full_path);
            }
        }
    }
    None
}

fn execute_external_command(cmd: &str, args: &[String]) {
    if let Some(path) = find_executable_in_path(cmd) {
        let cmd_name = Path::new(&path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(); // Extract only the filename

        let mut child = Command::new(path)
            .args(args)
            .env("ARGV0", cmd_name) 
            .spawn()
            .expect("Failed to execute command");
        
        let _ = child.wait(); 
    } else {
        println!("{}: command not found", cmd);
    }
}