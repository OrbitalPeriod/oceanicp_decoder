use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct Generator{
    pub manufacturer: Manufacturer,
    pub version : String,
    pub datetime : DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Manufacturer{
    pub contact : Contact,
}

#[derive(Debug, Deserialize)]
pub struct Contact{
    pub homepage : String,
}