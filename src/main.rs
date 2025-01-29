use std::{io::stdin, process::exit};
mod builtins;
mod parser;
mod utils;
use parser::Comms;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut home_dir: String = match Comms::init_home_dir() {
        Ok(dir) => dir,
        Err(_) => exit(1),
    };

    loop {
        builtins::shell_prompt(&home_dir);
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let user_comms = Comms::tokenize(input, home_dir.clone());
        builtins::comms_process(&user_comms, &mut home_dir);
    }
}
