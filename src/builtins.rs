use crate::Comms;
use dirs;
use std::{
    error::Error,
    fs,
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
    Mkdir,
    Touch,
    Clear,
    //Cat
    //Lemurfetch //like fastfetch or catnap
    ////more to be added
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
            "mkdir" => Ok(Builtin_comms::Mkdir),
            "touch" => Ok(Builtin_comms::Touch),
            "clear" => Ok(Builtin_comms::Clear),
            _ => Err("Command not found."),
        }
    }
}

pub fn shell_prompt() {
    match dirs::home_dir() {
        Some(path) => println!("{}", path.display()),
        None => println!("Home directory could not be determined"),
    }
    print!("--> ");
    stdout().flush().unwrap();
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
        Ok(Builtin_comms::Mkdir) => mkdir_builtin(&comms),
        Ok(Builtin_comms::Touch) => touch_builtin(&comms),
        Ok(Builtin_comms::Clear) => clear_builtin(&comms),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn touch_builtin(comms: &Comms) {
    println!("Command not implemented.");
}

fn clear_builtin(comms: &Comms) {
    println!("Command not implemented.");
}

fn mkdir_builtin(comms: &Comms) {
    if comms.args.len() == 0 {
        eprintln!("mkdir should have a path, i.e.: mkdir example will create a directory in home/user.
            \nmkdir folder_name/example will create a directory home/user_name/folder_name/example.");
        return;
    }
    let hme_dir = match ::dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Directory does not exist.");
            return;
        }
    };
    let _ = store_history(&comms);
    let args = comms.args.join("/");
    let dir_path = hme_dir.join(args);
    match fs::create_dir_all(&dir_path) {
        Ok(_) => println!("Directory created at: {}", dir_path.display()),
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
