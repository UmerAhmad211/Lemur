use crate::Comms;
use std::{
    env,
    error::Error,
    fs::OpenOptions,
    io::{stdout, Write},
    process::exit,
};

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

pub fn shell_prompt() -> Result<(), Box<dyn Error>> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    println!("|");
    print!("--> ");
    stdout().flush().unwrap();
    Ok(())
}

pub fn comms_process(comms: &Comms) {
    match Builtin_comms::check_builtin_comms(&comms.key_word) {
        Ok(Builtin_comms::Cd) => cd_builtin(&comms),
        Ok(Builtin_comms::Echo) => echo_builtin(&comms),
        Ok(Builtin_comms::Pwd) => pwd_builtin(&comms),
        Ok(Builtin_comms::Exit) => exit_builtin(&comms),
        Ok(Builtin_comms::History) => history_builtin(&comms),
        Ok(Builtin_comms::Export) => export_builtin(&comms),
        Ok(Builtin_comms::Unset) => unset_builtin(&comms),
        Ok(Builtin_comms::Alias) => alias_builtin(&comms),
        Ok(Builtin_comms::Unalias) => unalias_builtin(&comms),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cd_builtin(comms: &Comms) {
    println!("Command not implemented.");
}

fn echo_builtin(comms: &Comms) {
    let _rtrn = store_history(&comms);
    println!("{}", comms.args.join(" "));
}

fn pwd_builtin(comms: &Comms) {
    println!("Command not implemented.");
}

fn exit_builtin(comms: &Comms) {
    let _rtrn = store_history(&comms);
    if comms.args.len() == 0 {
        exit(0);
    } else if comms.args[0] == "0" {
        exit(0);
    }
    exit(1);
}

fn history_builtin(comms: &Comms) {
    println!("Command not implemented.");
}

fn export_builtin(comms: &Comms) {
    println!("Command not implemented.");
}

fn unset_builtin(comms: &Comms) {
    println!("Command not implemented.");
}

fn alias_builtin(comms: &Comms) {
    println!("Command not implemented.");
}

fn unalias_builtin(comms: &Comms) {
    println!("Command not implemented.");
}

fn store_history(comms: &Comms) -> Result<(), Box<dyn Error>> {
    let mut history_add = OpenOptions::new()
        .append(true)
        .create(true)
        .open("history.txt")?;

    let mut origin_input = comms.key_word.clone();
    origin_input.push_str(" ");

    for i in &comms.args {
        origin_input.push_str(i);
        origin_input.push_str(" ");
    }
    writeln!(history_add, "{\n}", origin_input)?;

    Ok(())
}
