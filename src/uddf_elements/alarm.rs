use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum AlarmType {
    Ascent,
    Breath,
    Deco,
    Error,
    Link,
    Microbubbles,
    Rbt,
    Skincooling,
    Surface,
}

use serde::de::Deserializer;

impl<'de> Deserialize<'de> for AlarmType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "ascent" => Ok(AlarmType::Ascent),
            "breath" => Ok(AlarmType::Breath),
            "deco" => Ok(AlarmType::Deco),
            "error" => Ok(AlarmType::Error),
            "link" => Ok(AlarmType::Link),
            "microbubbles" => Ok(AlarmType::Microbubbles),
            "rbt" => Ok(AlarmType::Rbt),
            "skincooling" => Ok(AlarmType::Skincooling),
            "surface" => Ok(AlarmType::Surface),
            _ => Err(serde::de::Error::custom("Invalid alarm type")),
        }
    }
}



crate::test_deserialization!(
    super::AlarmType,
    r#"
    <alarm>ascent</alarm>
    "#
);
