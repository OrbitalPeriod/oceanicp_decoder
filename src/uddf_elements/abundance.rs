use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Abundance(u32);

crate::test_deserialization!(
    super::Abundance,
    r#"
    <abundance>0</abundance>
    "#
);