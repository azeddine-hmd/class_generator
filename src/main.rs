extern crate serde_json;
use serde::Deserialize;
use std::fs::File;
use std::io::{stderr, stdin, stdout, Write};

#[derive(Deserialize, Debug)]
struct Variable {
    name: String,
    #[serde(rename = "access_modifier")]
    acc_mod: String,
    #[serde(rename = "type")]
    data_type: String,
}

#[derive(Deserialize, Debug)]
struct Class {
    name: String,
    #[serde(rename = "variables")]
    vars: Vec<Variable>,
}

enum FileExt {
    Header,
    Source,
}

fn main() {
    let jpath: String = get_jpath();
    dbg!(&jpath);

    let jcontent: String = std::fs::read_to_string(jpath).unwrap();

    // mapping json content into class struct
    let class: Class = serde_json::from_str(&jcontent).unwrap();
    drop(jcontent);
    dbg!(&class);

    // generating header file code
    let hfile = create_file(FileExt::Header, &class.name);
    generate_header_code(hfile, &class);
    println!("Code generated successfully for header file!");

    // generating source file code
    let cfile = create_file(FileExt::Source, &class.name);
    generate_source_code(cfile, &class);
    println!("Code generated successfully for source file!");
}

fn ask_for_overwrite() {
    writeln!(&mut stdout(), "File Already exist!").unwrap();
    write!(&mut stdout(), "Overwrite file? [Y/n]: ").unwrap();
    stdout().flush().expect("Failed to flush file buffer");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let answer = input.parse::<String>().unwrap().to_lowercase();
    let answer = answer.trim();
    if answer.ne("y") && answer.ne("yes") {
        std::process::exit(0);
    }
}

fn to_capital_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn create_file(file_ext: FileExt, class_name: &str) -> File {
    match file_ext {
        FileExt::Header => {
            let header_name = format!("{}.hpp", class_name);
            if std::path::Path::new(&header_name).exists() {
                ask_for_overwrite();
            }
            File::create(header_name).expect("Failed to create header file")
        }
        FileExt::Source => {
            let source_name = format!("{}.cpp", class_name);
            if std::path::Path::new(&source_name).exists() {
                ask_for_overwrite();
            }
            File::create(source_name).expect("Failed to create source file")
        }
    }
}

fn get_jpath() -> String{
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        writeln!(stderr(), "Usage: ../class_generator json_path").unwrap();
        std::process::exit(1);
    }

    args.get(1).unwrap().clone()
}

fn prefix_name(name: &str) -> String {
    let mut name_prefixed = String::from("m");
    let name_cl: String = to_capital_letter(name);
    name_prefixed.push_str(&name_cl);
    name_prefixed
}

fn generate_header_code(hfile: File, class: &Class) {
    // top guard macro
    writeln!(
        &hfile,
        "#ifndef {}_HPP",
        class.name.to_uppercase()
    )
    .unwrap();
    writeln!(
        &hfile,
        "#define {}_HPP\n",
        class.name.to_uppercase()
    )
    .unwrap();

    // begin class
    writeln!(&hfile, "class {} {{", class.name).unwrap();

    // write member varaibles
    let vars: Vec<&Variable> = class
        .vars
        .iter()
        .filter_map(|var| {
            if var.acc_mod.eq("private") {
                return Some(var);
            } else {
                return None;
            }
        })
        .collect();
    if !vars.is_empty() {
        for var in &vars {
            let prefixed_name = prefix_name(&var.name);
            writeln!(&hfile, "\t{}\t{};", var.data_type, prefixed_name).unwrap();
        }
        writeln!(&hfile).unwrap();
    }

    // public
    writeln!(&hfile, "public:").unwrap();

    // constructor
    writeln!(&hfile, "\t{}( void );", class.name).unwrap();

    // copy constructor
    writeln!(
        &hfile,
        "\t{}( {} const& copy );",
        class.name, class.name
    )
    .unwrap();

    // deconstructor
    writeln!(&hfile, "\t~{}( void );", class.name).unwrap();

    // assignamnet overloading
    writeln!(
        &hfile,
        "\t{}&\toperator=( {} const& other );",
        class.name, class.name
    )
    .unwrap();

    // getters and setters
    for var in &vars {
        let name_cl = to_capital_letter(&var.name);
        writeln!(
            &hfile,
            "\t{} const&\tget{}( void );",
            var.data_type, name_cl
        )
        .unwrap();
        writeln!(
            &hfile,
            "\tvoid\tset{}( {} const& {} );",
            name_cl, var.data_type, var.name
        )
        .unwrap();
    }

    // end class
    writeln!(&hfile, "}};").unwrap();

    // Bottom guard macro
    writeln!(&hfile, "\n#endif").unwrap();
}

fn generate_source_code(cfile: File, class: &Class) {
    // include
    writeln!(&cfile, "#include \"{}.hpp\"\n", class.name).unwrap();

    // default constructor
    writeln!(&cfile, "{}::{}( void ) {{}}\n", class.name, class.name).unwrap();

    let mut initializer = String::new();
    {
        for (i, var) in class.vars.iter().enumerate() {
            let prefixed_name = prefix_name(&var.name);
            let name_cl = to_capital_letter(&var.name);

            if i > 0 {
                initializer.push_str(", ");
            }
            initializer.push_str(&format!("{}(copy.get{}())", &prefixed_name, &name_cl));
        }
    }
    dbg!(&initializer);

    // copy construct
    write!(&cfile, "{}::{}( {} const& copy ): {} {{}}\n\n", class.name, class.name, class.name, initializer).unwrap();
}
