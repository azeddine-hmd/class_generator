use crate::utils;
use crate::Class;
use std::fs::File;
use std::io::{Result, Write};

pub fn generate_source_code(mut cfile: File, class: &Class) -> Result<()> {
    writeln!(&cfile, "#include \"{}.hpp\"", class.name)?; // include
    default_constructor(&mut cfile, class)?;
    copy_constructor(&mut cfile, class)?;
    destructor(&mut cfile, class)?;
    getters_setters(&mut cfile, class)?;
    assignment_operator(&mut cfile, class)?;

    Ok(())
}

fn default_constructor(cfile: &mut File, class: &Class) -> Result<()> {
    writeln!(cfile, "\n{0}::{0}( void ) {{}}", class.name)?;

    Ok(())
}

fn copy_constructor(cfile: &mut File, class: &Class) -> Result<()> {
    let mut initializer = String::new();
    for (i, var) in class.vars.iter().enumerate() {
        let name_pref = utils::prefix_withm(&var.name);
        let name_cap = utils::to_cap(&var.name);

        if i > 0 {
            initializer.push_str(", ");
        }
        initializer.push_str(&format!("{}(copy.get{}())", &name_pref, &name_cap));
    }
    write!(
        cfile,
        "\n{0}::{0}( {0} const& copy ): {1} {{}}\n",
        class.name, initializer
    )?;

    Ok(())
}

fn destructor(cfile: &mut File, class: &Class) -> Result<()> {
    writeln!(cfile, "\n{0}::~{0}( void ) {{}}", class.name)?;

    Ok(())
}

fn getters_setters(cfile: &mut File, class: &Class) -> Result<()> {
    for var in &class.vars {
        let name_pref = utils::prefix_withm(&var.name);
        let name_cap = utils::to_cap(&var.name);

        // getter
        writeln!(
            cfile,
            "\n{} const&\t{}::get{}( void ) const {{",
            var.data_type, class.name, name_cap
        )?;
        writeln!(cfile, "\treturn {};", name_pref)?;
        writeln!(cfile, "}}")?;

        // setter
        writeln!(
            cfile,
            "\nvoid\t{}::set{}( {} const& {} ) {{",
            class.name, name_cap, var.data_type, var.name
        )?;
        writeln!(cfile, "\t{} = {};", name_pref, var.name)?;
        writeln!(cfile, "}}")?;
    }

    Ok(())
}

fn assignment_operator(cfile: &mut File, class: &Class) -> Result<()> {
    writeln!(
        cfile,
        "\n{0}&\t{0}::operator=( {0} const& other ) {{",
        class.name
    )?;
    for var in &class.vars {
        let name_pref = utils::prefix_withm(&var.name);
        let name_cap = utils::to_cap(&var.name);

        writeln!(cfile, "\t{} = other.get{}();", name_pref, name_cap)?;
    }
    writeln!(cfile, "\n\treturn *this;")?;
    writeln!(cfile, "}}")?;

    Ok(())
}
