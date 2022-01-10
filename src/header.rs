use crate::utils;
use crate::Class;
use std::fs::File;
use std::io::{Result, Write};

pub fn generate_header_code(mut hfile: File, class: &Class) -> Result<()> {
    writeln!(&hfile, "#ifndef {}_HPP", class.name.to_uppercase())?;
    writeln!(&hfile, "#define {}_HPP\n", class.name.to_uppercase())?;
    writeln!(&hfile, "class {} {{", class.name)?;
    member_variables(&mut hfile, class)?;
    writeln!(&hfile, "public:")?;
    constructor(&mut hfile, class)?;
    copy_constructor(&mut hfile, class)?;
    destructor(&mut hfile, class)?;
    assignament(&mut hfile, class)?;
    getters_setters(&mut hfile, class)?;
    writeln!(&hfile, "}};")?;
    writeln!(&hfile, "\n#endif")?;

    Ok(())
}

fn member_variables(hfile: &mut File, class: &Class) -> Result<()> {
    let vars = utils::member_variables(&class.vars);

    if !vars.is_empty() {
        for var in &vars {
            let prefixed = utils::prefix_withm(&var.name);
            writeln!(hfile, "\t{}\t{};", var.data_type, prefixed)?;
        }
        writeln!(hfile)?;
    }

    Ok(())
}

fn constructor(hfile: &mut File, class: &Class) -> Result<()> {
    writeln!(hfile, "\t{}( void );", class.name)?;

    Ok(())
}

fn copy_constructor(hfile: &mut File, class: &Class) -> Result<()> {
    writeln!(hfile, "\t{0}( {0} const& copy );", class.name)?;

    Ok(())
}

fn destructor(hfile: &mut File, class: &Class) -> Result<()> {
    writeln!(hfile, "\t~{}( void );", class.name)?;

    Ok(())
}

fn assignament(hfile: &mut File, class: &Class) -> Result<()> {
    writeln!(hfile, "\t{0}&\toperator=( {} const& other );", class.name)?;

    Ok(())
}

fn getters_setters(hfile: &mut File, class: &Class) -> Result<()> {
    for var in &class.vars {
        let name_cl = utils::to_cap(&var.name);
        writeln!(hfile, "\t{} const&\tget{}( void ) const;", var.data_type, name_cl)?;
        writeln!(
            hfile,
            "\tvoid\tset{}( {} const& {} );",
            name_cl, var.data_type, var.name
        )?;
    }

    Ok(())
}
