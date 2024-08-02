use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct ServiceInterval(i32);

crate::test_deserialization!(
    super::ServiceInterval,
    r#"
    <serviceinterval>365</serviceinterval>
    "#
);