use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Sex{
    Undetermined,
    Male,
    Female,
    Hermaphrodite,
}