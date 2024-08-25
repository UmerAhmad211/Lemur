use std::process::exit;

enum Builtin_comms {
    Cd,
    Echo,
    Pwd,
    Exit,
    History, //shows history
    Export,  //sets env vars
    Unset,   //opposite of export
    Alias,   //sets an alias for a process e.g:alias vim=nvim
    Unalias, //oppposite of alias
             //more to be added
}

pub struct Comms {
    pub key_word: String,
    pub args: Vec<String>,
}

impl Comms {
    pub fn tokenize(input: String) -> Comms {
        let tokenized_in: Vec<String> =
            input.trim().split(' ').map(|val| val.to_string()).collect();
        let user_comms = Comms {
            key_word: tokenized_in[0].clone(),
            args: tokenized_in[1..].to_vec(),
        };

        user_comms
    }
}
// impl Comms {
//     pub fn exits_valid(key_word: &String, args: &Vec<String>) {
//         if key_word == "exit" && args[0] == "0" {
//             println!("Exiting");
//             exit(0);
//         } else if key_word == "exit" && args[0] == "1" {
//             eprint!("Err Exit");
//             exit(1);
//         }
//     }
// }
