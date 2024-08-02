use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct SerialNumber(String);

crate::test_deserialization!(
    super::SerialNumber,
    r#"
    <serialnumber>192837465</serialnumber>
    "#
);