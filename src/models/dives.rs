use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::de::Deserializer;

use super::link::Link;


#[derive(Debug, Deserialize)]
pub struct Dives{
    #[serde(rename = "repetitiongroup")]
    pub repitition_group : Vec<RepetitionGroup>,
}

#[derive(Debug, Deserialize)]
pub struct RepetitionGroup{
    #[serde(rename = "@id", default)]
    pub id : String,
    #[serde(rename = "dive")]
    pub dives : Vec<Dive>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dive{
    #[serde(rename = "@id", default)]
    pub id : [u8; 29],
    #[serde(rename = "informationbeforedive")]
    pub predive_information : PreDiveInformation,
    pub samples : Samples,
    #[serde(rename = "tankdata")]
    pub tank_data : TankData,
    #[serde(rename = "informationafterdive")]
    pub post_dive_information : PostDiveInformation,
}

impl Dive{
    pub fn calculate_sac_rate(&self) -> Option<f32>{
        let start_pressure = self.tank_data.start_pressure? / 100000.0;
        let end_pressure = self.tank_data.end_pressure? / 100000.0;
        let air_consumed = 12.0 * (start_pressure - end_pressure);

        let waypoints = &self.samples.waypoints;
        let average_depth = waypoints.iter().map(|wp| wp.depth).sum::<f32>() / waypoints.len() as f32;

        const SURFACE_PRESSURE : f32 = 1.0 ;
        const PRESSURE_PER_METER : f32 = 0.1;

        let average_pressure = SURFACE_PRESSURE + (average_depth * PRESSURE_PER_METER);

        let surface_equivalent = air_consumed / average_pressure;

        let sac_rate_per_second = surface_equivalent / self.post_dive_information.dive_duration;
        let sac_rate = sac_rate_per_second * 60.0;
        Some(sac_rate)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PreDiveInformation{
    #[serde(rename = "link")]
    pub site : Link,
    pub datetime : DateTime<Utc>,
    pub altitude : f32,
}

#[derive(Debug, Deserialize, Clone )]
pub struct Samples{
    #[serde(rename = "waypoint")]
    pub waypoints : Vec<Waypoint>
}

#[derive(Debug, Deserialize, Clone )]
pub struct Waypoint{
    pub depth : f32,
    pub divetime : f32,
    #[serde(rename = "switchmix")]
    pub mix_switch : Option<MixSwitch>,
    pub temperature : Option<f32>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct MixSwitch{
    #[serde(rename = "@ref", default)]
    pub name : String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TankData{
    #[serde(rename = "@id", default)]
    pub id : [u8; 29],
    #[serde(rename = "link")]
    pub mix : Link,
    #[serde(rename = "tankpressurebegin")]
    pub start_pressure : Option<f32>,
    #[serde(rename = "tankpressureend")]
    pub end_pressure : Option<f32>
}

#[derive(Debug, Deserialize, Clone)]
pub struct PostDiveInformation{
    #[serde(rename = "lowesttemperature")]
    pub lowest_temperature : f32,
    #[serde(rename = "greatestdepth")]
    pub greatest_depth : f32,
    #[serde(rename = "diveduration")]
    pub dive_duration : f32,
    #[serde(rename = "equipmentused")]
    pub equipment_used : EquipmentUsed,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EquipmentUsed{
    #[serde(rename = "leadquantity")]
    pub weight : f32,
}