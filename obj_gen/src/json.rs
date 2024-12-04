#[derive(Debug, serde::Deserialize)]
pub struct SpaTypeInfo {
    pub r#type: u32,
    pub parent: u32,
    pub name: String,
    #[allow(unused)]
    #[serde(default)]
    pub values: Vec<SpaTypeInfo>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Entry {
    pub name: String,
    pub r#type: u32,
    pub properties: Vec<SpaTypeInfo>,
}
