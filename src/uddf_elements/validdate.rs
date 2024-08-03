use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Validdate{
    pub datetime : super::datetime::Datetime,
}

crate::test_deserialization!(
    super::Validdate,
    r#"
    <validdate>
    <datetime>2007-02-28</datetime>
</validdate>
    "#
);