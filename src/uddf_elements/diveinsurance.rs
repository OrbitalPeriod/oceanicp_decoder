use serde::Deserialize;

use super::{issuedate::Issuedate, notes::Notes, validdate::Validdate};
#[derive(Debug, Deserialize, Clone)]
pub struct DiveInsurance{
    pub insurance : Vec<Insurance>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Insurance{
    pub aliasname : Option<Vec<String>>,
    pub issuedate : Option<Issuedate>,
    pub name : Option<String>,
    pub notes : Option<Notes>,
    pub validdate : Option<Validdate>,
}


crate::test_deserialization!(
    super::DiveInsurance,
    r#"
                <diveinsurances>
                <insurance>
                    <name>For Ever Diving</name>
                    <issuedate>
                        <datetime>2004-02-30</datetime>
                    </issuedate>
                    <validdate>
                        <datetime>2005-02-29</datetime>
                    </validdate>
                </insurance>
            </diveinsurances>
    "#
);