use serde::Deserialize;

use super::{dive_sites::DiveSite, dives::Dives, gas_definitions::GasDefintions, generator::Generator};

#[derive(Debug, Deserialize)]
pub struct InputFile{
    pub generator : Generator,
    #[serde(rename = "divesite")]
    pub divesites : DiveSite,
    #[serde(rename = "gasdefinitions")]
    pub gas_defintions : GasDefintions,
    #[serde(rename = "profiledata")]
    pub dives : Dives,
}

