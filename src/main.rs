extern crate serde_json;

pub mod data;
pub mod header;
pub mod source;
pub mod utils;

use data::{Class, FileType};

fn main() {
    let jpath: String = utils::json_path();

    let jcontent: String = std::fs::read_to_string(jpath).unwrap();

    // mapping json content into class struct
    let class: Class = serde_json::from_str(&jcontent).unwrap();

    // generating header file code
    let hfile = utils::create_file(FileType::Hpp, &class.name);
    header::generate_header_code(hfile, &class).unwrap();
    println!("Code generated successfully for header file!");

    // generating source file code
    let cfile = utils::create_file(FileType::Cpp, &class.name);
    source::generate_source_code(cfile, &class).unwrap();
    println!("Code generated successfully for source file!");
}
