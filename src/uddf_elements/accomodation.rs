use serde::de::Deserializer;
use serde::Deserialize;

use super::address::Address;
use super::contact::Contact;
use super::rating::Rating;
use super::notes::Notes;

#[derive(Debug, Deserialize, Clone)]
pub struct Accomodation {
    pub name: String,
    #[serde(deserialize_with = "deserialize_category")]
    pub category: Option<Category>,
    pub address: Option<Address>,
    pub contact: Option<Contact>,
    pub rating: Option<Vec<Rating>>,
    pub notes : Option<Notes>
}

fn deserialize_category<'de, D>(deserializer: D) -> Result<Option<Category>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "" => Ok(None),
        "hotel" => Ok(Some(Category::Hotel)),
        "camping" => Ok(Some(Category::Camping)),
        _ => Err(serde::de::Error::custom("Invalid category")),
    }
}

#[derive(Debug, Clone)]
pub enum Category {
    Hotel,
    Camping,
}

crate::test_deserialization!(super::Accomodation, r#" <accommodation>
    <name>Hotel of the 1000 Stars</name>
    <category>camping</category>
<address>
    <street>Kornstr. 24</street>
    <city>Unna</city>
    <postcode>59625</postcode>
    <country>Germany</country>
</address>
<contact>
    <language>English</language>
    <phone>01234/987654</phone>
    <mobilephone>0123/232323232</mobilephone>
    <fax>0123/232323231</fax>
    <email>walter.walrus@divedeep.info</email>
    <homepage>http://www.divedeep.info/walter/</homepage>
</contact>
    <rating>
        <datetime>2008-10-23</datetime>
        <ratingvalue>7</ratingvalue>
    </rating>
    <notes>
        <para>Nice "home" for this week!</para>
    </notes>
</accommodation>"#);

