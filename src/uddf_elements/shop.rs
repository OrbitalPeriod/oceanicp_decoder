use serde::Deserialize;

use super::{address::Address, contact::Contact, notes::Notes};
#[derive(Debug, Deserialize, Clone)]
pub struct Shop{
    pub aliasname : Option<Vec<String>>,
    pub address : Option<Address>,
    pub contact : Option<Contact>,
    pub name : String,
    pub notes : Option<Notes>,
}

crate::test_deserialization!(
    super::Shop,
    r#"
    <shop id="shop_the-deepdivers">
    <name>The DeepDivers</name>
    <contact>
        <!-- additional contact data (phone number, email address etc.) -->
    </contact>
    <notes>
        <!-- additional remarks, pictures etc. -->
    </notes>
</shop>
    "#
);