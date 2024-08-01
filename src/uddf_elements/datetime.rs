
use serde::Deserialize;
use serde::de::Deserializer;

//I currently dont have a good ISO8601 parser, i try with one i found, if it works, i use it, else i use the string
#[derive(Debug, Clone)]
pub struct Datetime{
    pub datetime : iso8601_timestamp::Timestamp,
}

impl<'de> Deserialize<'de> for Datetime{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> 
    {
        let s : String = Deserialize::deserialize(deserializer)?;

        match iso8601_timestamp::Timestamp::parse(&s)
        {
            Some(timestamp) => Ok(Datetime{datetime: timestamp}),
            None => Err(serde::de::Error::custom("Invalid ISO8601 format")),
        }
    }
}

crate::test_deserialization!(
    super::Datetime,
    r#"
    <datetime>2010-08-16</datetime>
    "#
);