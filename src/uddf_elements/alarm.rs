use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum Alarm {
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

impl<'de> Deserialize<'de> for Alarm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "ascent" => Ok(Alarm::Ascent),
            "breath" => Ok(Alarm::Breath),
            "deco" => Ok(Alarm::Deco),
            "error" => Ok(Alarm::Error),
            "link" => Ok(Alarm::Link),
            "microbubbles" => Ok(Alarm::Microbubbles),
            "rbt" => Ok(Alarm::Rbt),
            "skincooling" => Ok(Alarm::Skincooling),
            "surface" => Ok(Alarm::Surface),
            _ => Err(serde::de::Error::custom("Invalid alarm type")),
        }
    }
}

crate::test_deserialization!(
    super::Alarm,
    r#"
    <alarm>ascent</alarm>
    "#
);
