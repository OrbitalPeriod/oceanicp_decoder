use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Price(f32);

//TODO: Think of some way to make currency work

crate::test_deserialization!(
    super::Price,
    r#"
    <price currency="EUR">123.45</price>
    "#
);