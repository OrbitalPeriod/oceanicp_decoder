use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Model(String);

crate::test_deserialization!(
    super::Model,
    r#"
    <model>model</model>
    "#
);