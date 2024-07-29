use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Link{
    #[serde(rename = "@ref", default)]
    pub reference : String,
}