use serde::Deserialize;

use super::{address::Address, contact::Contact};
#[derive(Debug, Deserialize, Clone)]
pub struct Manufacturer{
    pub name : Option<String>,
    pub aliasname : Option<Vec<String>>,
    pub address : Option<Address>,
    pub contact : Option<Contact>,
}

crate::test_deserialization!(
    super::Manufacturer,
    r#"
    <manufacturer id="manufacturer_lame_ducks">
    <name>Lame Ducks Inc.</name>
    <address>
        <street>Lake Road 12</street>
        <city>Laketown</city>
        <postcode>91827</postcode>
        <country>New Zealand</country>
        <province>Wellington District</province>
    </address>
    <contact>
    <phone>01234/987654</phone>
        <mobilephone>0123/232323232</mobilephone>
        <email>info@lame-ducks.com</email>
        <homepage>http://www.lame-ducks.com</homepage>
    </contact>
</manufacturer>
    "#
);