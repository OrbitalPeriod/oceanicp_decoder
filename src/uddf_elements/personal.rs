use serde::Deserialize;

use super::sex::Sex;
#[derive(Debug, Deserialize, Clone)]
pub struct Personal{
    pub birthdate : Option<Birthdate>,
    pub birthname : Option<String>,
    pub bloodgroup : Option<String>,
    pub firstname : Option<String>,
    pub height : Option<f32>,
    pub honorific : Option<String>,
    pub lastname : Option<String>,
    pub membership : Option<Membership>,
    pub middlename : Option<String>,
    pub passport : Option<Passport>,
    pub sex : Option<Sex>,
    pub smoking : Option<String>,
    pub weight : Option<f32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Birthdate{
    pub datetime : super::datetime::Datetime,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Membership();
//TODO: Read organization and memberID

#[derive(Debug, Deserialize, Clone)]
pub struct Passport(u32);

crate::test_deserialization!(
    super::Personal,
    r#"
    <personal>
    <firstname>Arno</firstname>
    <middlename>Albert</middlename>
    <lastname>Alzheimer</lastname>
    <honorific>Dr.</honorific>
    <sex>male</sex>
    <birthdate>
        <datetime>2010-08-16</datetime>
    </birthdate>
    <passport>123456789</passport>
    <bloodgroup>A</bloodgroup>
</personal>
    "#
);