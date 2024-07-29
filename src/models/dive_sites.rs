use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DiveSite{
    #[serde(rename = "site", default)]
    pub sites : Vec<Site>,
}

#[derive(Debug, Deserialize)]
pub struct Site{
    #[serde(rename = "@id", default)]
    pub id : [u8; 29],
    pub name : String,
    pub geography : Geography,
}

#[derive(Debug, Deserialize)]
pub struct Geography{
    pub location : String,
    pub latitude : f32,
    pub longitude : f32,
    pub altitude : f32,
}