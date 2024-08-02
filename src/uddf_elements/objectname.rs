use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Objectname(String);

crate::test_deserialization!(
    super::Objectname,
    r#"
    <objectname>../abc/diving.jpg</objectname>
    "#
);