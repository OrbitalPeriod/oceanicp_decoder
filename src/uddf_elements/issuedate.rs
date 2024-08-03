use serde::Deserialize;

use super::datetime::Datetime;
#[derive(Debug, Deserialize, Clone)]
pub struct Issuedate{
    pub datetime : Datetime,
}

crate::test_deserialization!(
    super::Issuedate,
    r#"
    <issuedate>
    <datetime>2003-03-02</datetime>
    </issuedate>
    "#
);