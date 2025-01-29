use crate::Comms;
use std::{env, error::Error, ffi::OsStr, fs::OpenOptions, io::Write, process::exit};

pub fn store_history(comms: &Comms) -> Result<(), Box<dyn Error>> {
    let mut home_dir: String = match Comms::init_home_dir() {
        Ok(dir) => dir,
        Err(_) => exit(1),
    };

    let history_file_name = String::from(".lemur_history");
    home_dir.push_str("/");
    home_dir.push_str(&history_file_name);

    let mut history_add = OpenOptions::new()
        .append(true)
        .create(true)
        .open(home_dir)?;

    let mut origin_input = comms.key_word.clone();
    origin_input.push_str(" ");

    for i in &comms.args {
        origin_input.push_str(i);
        origin_input.push_str(" ");
    }
    writeln!(history_add, "{\n}", origin_input)?;

    Ok(())
}

pub fn os_str_to_string(op_string: &Option<&OsStr>) -> String {
    match op_string {
        Some(file_or_dir) => file_or_dir
            .to_str()
            .map_or_else(|| String::new(), |s| s.to_string()),
        None => String::new(),
    }
}

pub fn option_string_to_string(op_string: &Option<String>) -> String {
    match op_string {
        Some(ret_string) => ret_string.to_string(),
        None => String::from("Lemur grunts."),
    }
}

pub fn get_term_name() -> String {
    match env::var("TERM") {
        Ok(term) => term.to_string(),
        Err(_) => String::from("Lemur grunts."),
    }
}

pub fn get_desktop_name() -> String {
    match env::var("DESKTOP_SESSION") {
        Ok(desk_name) => desk_name.to_string(),
        Err(_) => String::from("Lemur grunts."),
    }
}
