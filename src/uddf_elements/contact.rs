use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Contact{
    pub language : Option<Vec<String>>,
    pub phone : Option<Vec<String>>,
    pub mobilephone : Option<Vec<String>>,
    pub fax : Option<Vec<String>>,
    pub email : Option<Vec<String>>,
    pub homepage : Option<Vec<String>>,
}

crate::test_deserialization!(
    super::Contact,
    r#"
    <contact>
    <language>English</language>
    <phone>01234/987654</phone>
    <mobilephone>0123/232323232</mobilephone>
    <fax>0123/232323231</fax>
    <email>walter.walrus@divedeep.info</email>
    <homepage>http://www.divedeep.info/walter/</homepage>
</contact>
    "#
);
