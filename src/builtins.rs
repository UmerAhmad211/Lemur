use crate::Comms;
use std::{error::Error, process::exit};

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

impl Builtin_comms {
    fn check_builtin_comms(key_word: &str) -> Result<Self, &'static str> {
        match key_word {
            "cd" => Ok(Builtin_comms::Cd),
            "echo" => Ok(Builtin_comms::Echo),
            "pwd" => Ok(Builtin_comms::Pwd),
            "exit" => Ok(Builtin_comms::Exit),
            "history" => Ok(Builtin_comms::History),
            "export" => Ok(Builtin_comms::Export),
            "unset" => Ok(Builtin_comms::Unset),
            "alias" => Ok(Builtin_comms::Alias),
            "unalias" => Ok(Builtin_comms::Unalias),
            _ => Err("Command not found."),
        }
    }
}

pub fn comms_process(comms: &Comms) {
    match Builtin_comms::check_builtin_comms(&comms.key_word) {
        Ok(Builtin_comms::Cd) => cd_builtin(&comms.args),
        Ok(Builtin_comms::Echo) => echo_builtin(&comms.args),
        Ok(Builtin_comms::Pwd) => pwd_builtin(&comms.args),
        Ok(Builtin_comms::Exit) => exit_builtin(&comms.args),
        Ok(Builtin_comms::History) => history_builtin(&comms.args),
        Ok(Builtin_comms::Export) => export_builtin(&comms.args),
        Ok(Builtin_comms::Unset) => unset_builtin(&comms.args),
        Ok(Builtin_comms::Alias) => alias_builtin(&comms.args),
        Ok(Builtin_comms::Unalias) => unalias_builtin(&comms.args),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cd_builtin(args: &Vec<String>) {
    println!("Command not implemented.");
}

fn echo_builtin(args: &Vec<String>) {
    println!("{}", args.join(" "));
}

fn pwd_builtin(args: &Vec<String>) {
    println!("Command not implemented.");
}

fn exit_builtin(args: &Vec<String>) {
    if args.len() == 0 {
        exit(0);
    } else if args[0] == "0" {
        exit(0);
    }
    exit(1);
}

fn history_builtin(args: &Vec<String>) {
    println!("Command not implemented.");
}

fn export_builtin(args: &Vec<String>) {
    println!("Command not implemented.");
}

fn unset_builtin(args: &Vec<String>) {
    println!("Command not implemented.");
}

fn alias_builtin(args: &Vec<String>) {
    println!("Command not implemented.");
}

fn unalias_builtin(args: &Vec<String>) {
    println!("Command not implemented.");
}
