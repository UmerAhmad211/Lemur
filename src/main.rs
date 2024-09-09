use std::io::stdin;
mod builtins;
mod parser;
use parser::Comms;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    loop {
        builtins::shell_prompt();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let user_comms = Comms::tokenize(input);
        builtins::comms_process(&user_comms);
    }
}
