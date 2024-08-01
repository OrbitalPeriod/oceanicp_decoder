use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct TimeSpanBeforeDive(f32);

crate::test_deserialization!(
    super::TimeSpanBeforeDive,
    r#"
    <timespanbeforedive>12600.0</timespanbeforedive>
    "#
);