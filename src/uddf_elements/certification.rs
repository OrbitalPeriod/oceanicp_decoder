use serde::Deserialize;

use super::{instructor::Instructor, issuedate::Issuedate};
#[derive(Debug, Deserialize, Clone)]
pub struct Certification{
    pub instructor : Option<Instructor>,
    pub issuedate : Option<Issuedate>,
    pub level : Option<String>,
    pub organization : Option<String>,
    pub specialty : Option<Vec<Specialty>>,
    pub validdate : Option<super::validdate::Validdate>,
    
}

#[derive(Debug, Deserialize, Clone)]
pub struct Specialty(String);

crate::test_deserialization!(
    super::Certification,
    r#"
    "#
);