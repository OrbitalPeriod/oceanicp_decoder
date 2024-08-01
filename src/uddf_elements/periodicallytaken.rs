#[derive(Debug, Clone)]
pub struct PeriodicallyTaken(bool);

impl <'de> serde::Deserialize<'de> for PeriodicallyTaken {
    fn deserialize<D>(deserializer: D) -> Result<PeriodicallyTaken, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "yes" => Ok(PeriodicallyTaken(true)),
            "no" => Ok(PeriodicallyTaken(false)),
            _ => Err(serde::de::Error::custom("Invalid value for PeriodicallyTaken, expecting yes or no")),
        }
    }
    
}

crate::test_deserialization!(
    super::PeriodicallyTaken,
    r#"
    <periodicallytaken>no</periodicallytaken>
    "#
);