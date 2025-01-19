use crate::Comms;
use std::{
    env,
    error::Error,
    ffi::OsStr,
    fs::{self, File, OpenOptions},
    io::{stdout, Write},
    path::Path,
    process::exit,
};
use sysinfo::System;

enum BuiltinCommands {
    Cd,
    Pwd,
    Exit,
    History,
    Mkdir,
    Touch,
    Clear,
    Cat,
    Ls,
    LemurFetch,
    Help,
}

impl BuiltinCommands {
    fn check_builtin_comms(key_word: &str) -> Result<Self, &'static str> {
        match key_word {
            "cd" => Ok(BuiltinCommands::Cd),
            "pwd" => Ok(BuiltinCommands::Pwd),
            "exit" => Ok(BuiltinCommands::Exit),
            "history" => Ok(BuiltinCommands::History),
            "mkdir" => Ok(BuiltinCommands::Mkdir),
            "touch" => Ok(BuiltinCommands::Touch),
            "clear" => Ok(BuiltinCommands::Clear),
            "cat" => Ok(BuiltinCommands::Cat),
            "ls" => Ok(BuiltinCommands::Ls),
            "lemf" => Ok(BuiltinCommands::LemurFetch),
            "help" => Ok(BuiltinCommands::Help),
            _ => Err("Command not found."),
        }
    }
}

pub fn shell_prompt(curr_dir: &String) {
    print!("\x1b[34m{} > \x1b[0m", curr_dir);
    stdout().flush().unwrap();
}

pub fn comms_process(comms: &Comms, curr_dir_path: &mut String) {
    match BuiltinCommands::check_builtin_comms(&comms.key_word) {
        Ok(BuiltinCommands::Cd) => cd_builtin(&comms, curr_dir_path),
        Ok(BuiltinCommands::Pwd) => pwd_builtin(&comms),
        Ok(BuiltinCommands::Exit) => exit_builtin(&comms),
        Ok(BuiltinCommands::History) => history_builtin(&comms),
        Ok(BuiltinCommands::Mkdir) => mkdir_builtin(&comms),
        Ok(BuiltinCommands::Touch) => touch_builtin(&comms),
        Ok(BuiltinCommands::Clear) => clear_builtin(&comms),
        Ok(BuiltinCommands::Cat) => cat_builtin(&comms),
        Ok(BuiltinCommands::Ls) => ls_builtin(&comms),
        Ok(BuiltinCommands::LemurFetch) => lemf_builtin(&comms),
        Ok(BuiltinCommands::Help) => help_builtin(&comms),
        Err(e) => eprintln!("\x1b[31mLemur: Error: {}\x1b[0m", e),
    }
}

fn touch_builtin(comms: &Comms) {
    if comms.args.len() == 1 {
        let mut touch_file_path = comms.curr_dir.clone();
        touch_file_path.push('/');
        touch_file_path.push_str(&comms.args[0]);
        match File::create(&touch_file_path) {
            Ok(_) => {}
            Err(_) => eprintln!("\x1b[31mLemur: Failed to create file.\x1b[0m"),
        }
        _ = store_history(&comms);
        return;
    }
    eprintln!("\x1b[31mLemur: touch failed.\x1b[0m");
}

fn clear_builtin(comms: &Comms) {
    if comms.args.is_empty() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        stdout().flush().unwrap();
        _ = store_history(&comms);
        return;
    }
}

fn mkdir_builtin(comms: &Comms) {
    if comms.args.len() == 1 {
        let mut mkdir_dir = comms.curr_dir.clone();
        mkdir_dir.push('/');
        mkdir_dir.push_str(&comms.args[0]);
        _ = fs::create_dir(mkdir_dir);
        _ = store_history(&comms);
        return;
    }
    eprintln!("\x1b[31mLemur: Failed to create DIR.\x1b[0m");
}

fn cd_builtin(comms: &Comms, curr_dir_path: &mut String) {
    if comms.args.len() == 1 {
        _ = store_history(&comms);
        if comms.args[0] == ".." {
            let dir_path = Path::new(curr_dir_path);
            if let Some(parent_dir) = dir_path.parent() {
                match env::set_current_dir(parent_dir) {
                    Ok(_) => *curr_dir_path = parent_dir.to_string_lossy().to_string(),
                    Err(_) => eprintln!("\x1b[31mLemur: Not a DIR.\x1b[0m"),
                }
            }
        } else {
            let mut cd_dir = comms.curr_dir.clone();
            cd_dir.push('/');
            cd_dir.push_str(&comms.args[0]);
            match env::set_current_dir(&cd_dir) {
                Ok(_) => *curr_dir_path = cd_dir.clone(),
                Err(_) => eprintln!("\x1b[31mLemur: Not a DIR.\x1b[0m"),
            }
        }
        return;
    } else {
        _ = store_history(&comms);
        let home_dir: String = match Comms::init_home_dir() {
            Ok(dir) => dir,
            Err(_) => exit(1),
        };
        match env::set_current_dir(&home_dir) {
            Ok(_) => *curr_dir_path = home_dir.clone(),
            Err(_) => eprintln!("\x1b[31mLemur: cd Error.\x1b[0m"),
        }
    }
}

fn pwd_builtin(comms: &Comms) {
    if comms.args.is_empty() {
        println!("{}", comms.curr_dir);
        _ = store_history(&comms);
        return;
    }
    eprintln!("\x1b[31mLemur: pwd failed.\x1b[0m");
}

fn exit_builtin(comms: &Comms) {
    if comms.args.len() == 0 {
        _ = store_history(&comms);
        exit(0);
    } else if comms.args[0] == "0" {
        _ = store_history(&comms);
        exit(0);
    }
    exit(1);
}

fn history_builtin(comms: &Comms) {
    if comms.args.is_empty() {
        let mut home_dir: String = match Comms::init_home_dir() {
            Ok(dir) => dir,
            Err(_) => exit(1),
        };

        let history_file_name = String::from(".lemur_history");
        home_dir.push_str("/");
        home_dir.push_str(&history_file_name);
        match fs::read_to_string(history_file_name) {
            Ok(content) => println!("{}", content),
            Err(_) => {
                eprintln!("\x1b[31mLemur: Failed to show history.\x1b[0m");
                return;
            }
        }
        _ = store_history(&comms);
        return;
    }
    eprintln!("\x1b[31mLemur: Failed to show history.\x1b[0m");
}

fn cat_builtin(comms: &Comms) {
    if comms.args.len() == 1 {
        let mut cat_file_name = comms.curr_dir.clone();
        cat_file_name.push('/');
        cat_file_name.push_str(&comms.args[0]);

        match fs::read_to_string(cat_file_name) {
            Ok(content) => println!("{}", content),
            Err(_) => {
                eprintln!("\x1b[31mLemur: Failed to show file content.\x1b[0m");
                return;
            }
        }
        _ = store_history(&comms);
        return;
    }
    eprintln!("\x1b[31mLemur: Failed to cat.\x1b[0m");
}

fn ls_builtin(comms: &Comms) {
    if comms.args.is_empty() {
        _ = store_history(&comms);
        match fs::read_dir(&comms.curr_dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            let file_dir_name = path.file_name();
                            let ret_val = os_str_to_string(&file_dir_name);
                            if path.is_dir() {
                                println!("\x1b[32m{}\x1b[0m", ret_val);
                            } else if path.is_file() {
                                println!("{}", ret_val);
                            } else {
                                println!("{}", ret_val);
                            }
                        }

                        Err(_) => {}
                    }
                }
            }

            Err(_) => eprintln!("\x1b[31mLemur: ls failed.\x1b[0m"),
        }
    }
}

fn lemf_builtin(comms: &Comms) {
    if comms.args.is_empty() {
        let mut sys_info = System::new_all();
        sys_info.refresh_all();
        let lemur = r#"                      ,,
                      ==
                       ==
                         ==
                          ==
                    ==     ==
                  ==  ==  ==
                 ==     ==
          ,  ,    ==
          |\/|   ,-..-,
      ,d__(..)\_/      \
      ;-,_`o/          |
          '-| \_,' /^| /
            ( //  /  \ \
            || \ <    \ )
           _\|  \ )   _\\
            ~`  _\|    ~`
                 ~`"#;
        println!("{}", lemur);
        let memory = sys_info.total_memory() / (1024 * 1024 * 1024);
        println!(
            "\x1b[32mTotal memory: \x1b[0m   {} \x1b[32mGiB\x1b[0m",
            memory
        );
        let memory = sys_info.used_memory() / (1024 * 1024 * 1024);
        println!(
            "\x1b[32mUsed memory: \x1b[0m    {} \x1b[32mGiB\x1b[0m",
            memory
        );
        println!(
            "\x1b[32mSystem name: \x1b[0m    {}",
            option_string_to_string(&System::name())
        );
        println!(
            "\x1b[32mKernel version: \x1b[0m {}",
            option_string_to_string(&System::kernel_version())
        );
        println!(
            "\x1b[32mOS version: \x1b[0m     {}",
            option_string_to_string(&System::os_version())
        );
        println!(
            "\x1b[32mHost name: \x1b[0m      {}",
            option_string_to_string(&System::host_name())
        );
        println!("\x1b[32mNumber of CPUs: \x1b[0m {}", sys_info.cpus().len());
        println!("\x1b[32mTerminal: \x1b[0m       {}", get_term_name());
        println!("\x1b[32mDE/WM: \x1b[0m          {}", get_desktop_name());
        _ = store_history(&comms);
        return;
    }
    eprintln!("\x1b[31mLemur: fetch failed.\x1b[0m");
}

fn help_builtin(comms: &Comms) {
    if comms.args.is_empty() {
        let help = r#"
        cd <dir_name>, cd .. to go to parent directory, cd to go to home directory
        exit
        pwd 
        history (shows history which is stored in a file .lemur_history in the home directory)
        mkdir <dir_name>
        touch <file_name>
        clear 
        cat <file_name>
        ls
        lemf
        help
       "#;
        println!("{}", help);
        _ = store_history(&comms);
        return;
    }
    eprintln!("\x1b[31mLemur: help failed.\x1b[0m");
}

fn store_history(comms: &Comms) -> Result<(), Box<dyn Error>> {
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

fn os_str_to_string(op_string: &Option<&OsStr>) -> String {
    match op_string {
        Some(file_or_dir) => file_or_dir
            .to_str()
            .map_or_else(|| String::new(), |s| s.to_string()),
        None => String::new(),
    }
}

fn option_string_to_string(op_string: &Option<String>) -> String {
    match op_string {
        Some(ret_string) => ret_string.to_string(),
        None => String::from("Lemur grunts."),
    }
}

fn get_term_name() -> String {
    match env::var("TERM") {
        Ok(term) => term.to_string(),
        Err(_) => String::from("Lemur grunts."),
    }
}

fn get_desktop_name() -> String {
    match env::var("DESKTOP_SESSION") {
        Ok(desk_name) => desk_name.to_string(),
        Err(_) => String::from("Lemur grunts."),
    }
}
