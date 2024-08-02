use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct NextServiceDate{
    pub datetime : super::datetime::Datetime,
}

crate::test_deserialization!(
    super::NextServiceDate,
    r#"
    <nextservicedate>
    <!-- device has to be revisioned until May 31st, 2007 (a Thursday) -->
    <datetime>2007-05-31</datetime>
</nextservicedate>
    "#
);