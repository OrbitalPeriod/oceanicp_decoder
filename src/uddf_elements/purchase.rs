use serde::Deserialize;

use super::{price::Price, shop::Shop};
#[derive(Debug, Deserialize, Clone)]
pub struct Purchase{
    pub datetime : Option<super::datetime::Datetime>,
    pub link : Option<super::link::Link>,
    pub price : Option<Price>,
    pub shop : Option<Shop>,
}

crate::test_deserialization!(
    super::Purchase,
    r#"
    <purchase>
    <datetime>2002-05-04</datetime>
    <price currency="EUR">16.95</price>
    <shop>
        <name>Wassersport-Freizeit Mader</name>
        <address>
            <street>Kornstr. 24</street>
            <city>Unna</city>
            <postcode>59625</postcode>
            <country>Germany</country>
        </address>
        <contact>
            <language>German</language>
            <phone>02303/16514</phone>
            <email>tauchsport-zentrale.mader@t-online.de</email>
            <homepage>http://www.wassersport-freizeit-mader.de</homepage>
        </contact>
    </shop>
</purchase>
            
    "#
);