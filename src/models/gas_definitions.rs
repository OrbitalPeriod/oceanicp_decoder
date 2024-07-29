use serde::Deserialize;
use serde::de::Deserializer;

#[derive(Debug, Deserialize)]
pub struct GasDefintions{
    #[serde(rename = "mix")]
    pub mixes : Vec<Mix>,
}

#[derive(Debug, Deserialize)]
pub struct Mix{
    #[serde(rename = "@id", default)]
    pub id : String,
    pub name : String,
    #[serde(rename = "o2", deserialize_with = "deserialize_percentage")]
    pub oxygen : u32,
    #[serde(rename = "n2", deserialize_with = "deserialize_percentage")]
    pub nitrogen : u32,
}

fn deserialize_percentage<'de, D>(deserializer: D) -> Result<u32, D::Error>
where D : Deserializer<'de>{
    let f : f32 = Deserialize::deserialize(deserializer)?;
    Ok((f * 100.0) as u32)
}