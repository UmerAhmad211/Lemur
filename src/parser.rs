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
