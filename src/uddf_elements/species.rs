use serde::Deserialize;

use super::{abundance::Abundance, notes::Notes, sex::Sex};
#[derive(Debug, Deserialize, Clone)]
pub struct Species{
    pub abundance : Option<Abundance>,
    pub age : Option<f32>,
    pub dominance : Option<String>,
    pub lifestage : Option<Lifestage>,
    pub notes : Option<Notes>,
    pub scientificname : Option<String>,
    pub sex : Option<Sex>,
    pub size : Option<f32>,
    pub trivialname : Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Lifestage{
    Larva,
    Juvenile,
    Adult,
}

crate::test_deserialization!(
    super::Species,
    r#"
    <species id="rainbow-wrasse-2">
        <trivialname>Rainbow wrasse</trivialname>
        <lifestage>adult</lifestage>
        <scientificname>Coris julis</scientificname>
        <abundance occurrence="2">8</abundance>         <!-- here: group of Coris julis - 8 fishes -->
        <sex>female</sex>                               <!-- here: female -->
                                                      <!-- no size information about the fishes is given -->
        <note>
            <!-- 4 pictures taken of female fishes -->
            <link ref="img_coris_julis_002"/>
            <link ref="img_coris_julis_003"/>
            <link ref="img_coris_julis_004"/>
            <link ref="img_coris_julis_005"/>
        </note>
    </species>
    "#
);