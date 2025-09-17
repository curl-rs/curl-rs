use std::env;

pub struct Argument {
    args: Vec<String>,

}
impl Argument {
    pub fn new(aurgument: String, short_arg: Option<String>) {}
}

impl Arguments {
    fn parse_sysargs() {
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
}
