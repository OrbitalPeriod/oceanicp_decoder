use serde::Deserialize;

use super::datetime::Datetime;

#[derive(Debug, Deserialize, Clone)]
pub struct Rating {
    pub datetime: Option<Datetime>,
    pub ratingvalue: f32,
}

crate::test_deserialization!(
    super::Rating,
    r#"
<rating>
    <datetime>2010-08-16</datetime>
    <ratingvalue>9</ratingvalue>
</rating>
        "#
);
