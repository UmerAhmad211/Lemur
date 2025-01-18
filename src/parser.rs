use std::env;

pub struct Comms {
    pub key_word: String,
    pub args: Vec<String>,
    pub curr_dir: String,
}

impl Comms {
    pub fn tokenize(input: String, _curr_dir: String) -> Comms {
        let tokenized_in: Vec<String> =
            input.trim().split(' ').map(|val| val.to_string()).collect();
        let user_comms = Comms {
            key_word: tokenized_in[0].clone(),
            args: tokenized_in[1..].to_vec(),
            curr_dir: _curr_dir,
        };

        user_comms
    }

    pub fn init_home_dir() -> Result<String, ()> {
        match env::var("HOME") {
            Ok(path) => Ok(path),
            Err(_) => Err(()),
        }
    }
}
