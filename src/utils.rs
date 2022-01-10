use crate::data::{FileType, Variable};
use std::fs::File;
use std::io::{self, Write};

pub fn usage() {
    writeln!(io::stderr(), "Usage: ../class_generator JSON_PATH").unwrap();
}

pub fn to_cap(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn prefix_withm(s: &str) -> String {
    let mut prefixed = String::from("m");
    let capitalized: String = to_cap(s);
    prefixed.push_str(&capitalized);
    return prefixed;
}

pub fn json_path() -> String {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        usage();
        std::process::exit(1);
    }

    args.get(1).unwrap().clone()
}

pub fn ask_for_overwrite() {
    writeln!(&mut io::stdout(), "File Already exist!").unwrap();
    write!(&mut io::stdout(), "Overwrite file? [Y/n]: ").unwrap();
    io::stdout().flush().expect("Failed to flush file buffer");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let answer = input.parse::<String>().unwrap().to_lowercase();
    let answer = answer.trim();
    if answer.ne("y") && answer.ne("yes") {
        std::process::exit(0);
    }
}

pub fn create_file(file_ext: FileType, class_name: &str) -> File {
    match file_ext {
        FileType::Hpp => {
            let header_name = format!("{}.hpp", class_name);
            if std::path::Path::new(&header_name).exists() {
                ask_for_overwrite();
            }
            File::create(header_name).expect("Failed to create header file")
        }
        FileType::Cpp => {
            let source_name = format!("{}.cpp", class_name);
            if std::path::Path::new(&source_name).exists() {
                ask_for_overwrite();
            }
            File::create(source_name).expect("Failed to create source file")
        }
    }
}

pub fn member_variables(vars: &Vec<Variable>) -> Vec<&Variable> {
    vars
        .iter()
        .filter_map(|var| {
            if var.acc_mod.eq("private") {
                return Some(var);
            } else {
                return None;
            }
        })
    .collect()
}
