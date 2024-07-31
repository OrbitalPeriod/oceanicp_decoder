use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Address {
    pub street: Option<String>,
    pub city: Option<String>,
    pub postcode: Option<String>,
    pub country: String,
    pub province: Option<String>,
}

crate::test_deserialization!(
    super::Address,
    r#"
    <address>
    <street>Kornstr. 24</street>
    <city>Unna</city>
    <postcode>59625</postcode>
    <country>Germany</country>
</address>
    "#
);

