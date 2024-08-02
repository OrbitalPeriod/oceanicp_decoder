use serde::Deserialize;

use super::{address::Address, contact::Contact, notes::Notes, rating::Rating};
#[derive(Debug, Deserialize, Clone)]
pub struct Operator{
    pub aliasname : Option<Vec<String>>,
    pub address : Option<Address>,
    pub contact : Option<Contact>,
    pub name : String,
    pub notes : Option<Notes>,
    pub rating : Option<Rating>,
}

crate::test_deserialization!(
    super::Operator,
    r#"
    <operator>
    <name>Sunrise Tours</name>
    <address>
        <country>Germany</country>
    </address>
    <contact>
        <!-- email address, and homepage of operator -->
    </contact>
    <rating>
        <ratingvalue>1</ratingvalue> <!-- lowest possible rating :-) -->
    </rating>
    <notes>
        <para>Never again!!!</para>
    </notes>
</operator>
    "#
);