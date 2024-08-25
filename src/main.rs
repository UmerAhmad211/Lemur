use std::io::{stdin, stdout, Write};
mod command;
use command::Comms;

fn main() {
    loop {
        print!(">");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let user_comms = Comms::tokenize(input);
        //  Comms::exits_valid(&user_comms.key_word, &user_comms.args);
        println!("{:?}", user_comms.key_word);
        println!("{:?}", user_comms.args);
    }
}
