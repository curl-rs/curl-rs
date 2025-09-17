mod parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in &args[1..] {
        if arg.starts_with("--") {
            let parts: Vec<&str> = arg[2..].splitn(2, '=').collect();
            match parts.as_slice() {
                [key, value] => {
                    println!("Option: {} Value: {}", key, value);
                }
                [key] => {
                    println!("Flag: {}", key);
                }
                _ => {}
            }
        } else {
            println!("Positional argument: {}", arg);
        }
    }
}
