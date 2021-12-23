extern crate serde_json;
use serde::Deserialize;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::process::exit;

#[derive(Deserialize, Debug)]
struct Variable {
    name: String,

    access_modifier: String,

    #[serde(rename = "type")]
    variable_type: String,
}

#[derive(Deserialize, Debug)]
struct Class {
    name: String,

    #[serde(rename = "variables")]
    variables: Vec<Variable>,
}

fn main() {
    let json_content = std::fs::read_to_string("example/weapon.json")
        .expect("Something went wrong reading from the file");

    let class: Class = serde_json::from_str(&json_content)
        .expect("Failed to deseerialize json content into 'Class' struct");

    //dbg!(&class);

    // check header file if exists and overwrite if permited
    let header_name = format!("{}.hpp", class.name);
    if std::path::Path::new(&header_name).exists() {
        ask_for_overwrite();
    }

    // create header file
    let mut header_file = File::create(header_name).expect("failed to create target file");

    // write begining of guard macro
    writeln!(
        &mut header_file,
        "#ifndef {}_HPP",
        class.name.to_uppercase()
    )
    .unwrap();
    writeln!(
        &mut header_file,
        "#define {}_HPP\n",
        class.name.to_uppercase()
    )
    .unwrap();

    // write class
    writeln!(&mut header_file, "class {} {{", class.name);

    // write member varaibles
    let variables: Vec<&Variable> = class
        .variables
        .iter()
        .filter_map(|variable| {
            if variable.access_modifier.eq("private") {
                return Some(variable);
            } else {
                return None;
            }
        })
        .collect();
    if !variables.is_empty() {
        for variable in variables {
            writeln!(&mut header_file, "\t{}\t{};", variable.variable_type, variable.name);
        }
        writeln!(&mut header_file);
    }

    // public
    writeln!(&mut header_file, "public:");

    // constructor
    //TODO: continue
    //writeln!(&mut head_file)

    // end of class
    writeln!(&mut header_file, "}};");

    // end of guard macro
    writeln!(&mut header_file, "\n#endif");

    println!("Code generated successfully!");
}

fn ask_for_overwrite() {
    writeln!(&mut stdout(), "File Already exist!");
    write!(&mut stdout(), "Overwrite file? [Y/n]: ");
    stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    let answer: String = input.parse().expect("failed to parse input from stdin");
    let answer_lowercase = answer.to_lowercase();
    if answer_lowercase.ne("y") || answer_lowercase.ne("yes") {
        exit(0);
    }
}
