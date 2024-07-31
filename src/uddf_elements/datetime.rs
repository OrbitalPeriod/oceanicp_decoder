use chrono::Utc;
use serde::Deserialize;
use serde::de::Deserializer;

#[derive(Debug, Deserialize, Clone)]
pub struct Datetime{
    //pub datetime : chrono::DateTime<Utc>,
}

fn deserialize_category<'de, D>(deserializer: D) -> Result<chrono::DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let _: String = Deserialize::deserialize(deserializer)?;
    //iso8601::datetime(&s).map_err(serde::de::Error::custom("Datetime not in correct format"))?;

    todo!("Implement datetime deserialization");
}

crate::test_deserialization!(
    super::Datetime,
    r#"
    <datetime>2010-08-16</datetime>
    "#
);