use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Contact{
    language : Option<Vec<String>>,
    phone : Option<Vec<String>>,
    mobilephone : Option<Vec<String>>,
    fax : Option<Vec<String>>,
    email : Option<Vec<String>>,
    homepage : Option<Vec<String>>,
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
