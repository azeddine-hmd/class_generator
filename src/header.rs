use crate::utils;
use crate::{Class, Variable};
use std::fs::File;
use std::io::{Result, Write};

pub fn generate_header_code(hfile: File, class: &Class) -> Result<()> {
    // top guard macro
    writeln!(&hfile, "#ifndef {}_HPP", class.name.to_uppercase())?;
    writeln!(&hfile, "#define {}_HPP\n", class.name.to_uppercase())?;

    // begin class
    writeln!(&hfile, "class {} {{", class.name)?;

    // obtain all member variables
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

    // member variable
    if !vars.is_empty() {
        for var in &vars {
            let prefixed = utils::prefix_withm(&var.name);
            writeln!(&hfile, "\t{}\t{};", var.data_type, prefixed)?;
        }
        writeln!(&hfile)?;
    }

    // public
    writeln!(&hfile, "public:")?;

    // constructor
    writeln!(&hfile, "\t{}( void );", class.name)?;

    // copy constructor
    writeln!(&hfile, "\t{0}( {0} const& copy );", class.name)?;

    // deconstructor
    writeln!(&hfile, "\t~{}( void );", class.name)?;

    // assignamnet overloading
    writeln!(&hfile, "\t{0}&\toperator=( {} const& other );", class.name)?;

    // getters and setters
    for var in &vars {
        let name_cl = utils::to_cap(&var.name);
        writeln!(
            &hfile,
            "\t{} const&\tget{}( void );",
            var.data_type, name_cl
        )?;
        writeln!(
            &hfile,
            "\tvoid\tset{}( {} const& {} );",
            name_cl, var.data_type, var.name
        )?;
    }

    // end class
    writeln!(&hfile, "}};")?;

    // Bottom guard macro
    writeln!(&hfile, "\n#endif")?;

    Ok(())
}
