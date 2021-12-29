use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "access_modifier")]
    pub acc_mod: String,
    #[serde(rename = "type")]
    pub data_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Class {
    pub name: String,
    #[serde(rename = "variables")]
    pub vars: Vec<Variable>,
}

pub enum FileExt {
    Hpp,
    Cpp,
}
