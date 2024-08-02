use serde::Deserialize;

use super::{link::Link, manufacturer::Manufacturer, model::Model, nextservicedate::NextServiceDate, notes::Notes, purchase::Purchase, serialnumber::SerialNumber, serviceinterval::ServiceInterval};
#[derive(Debug, Deserialize, Clone)]
pub struct EquipmentItem{
    pub aliasname : Option<Vec<String>>,
    pub link : Option<Link>,
    pub manufacturer : Option<Manufacturer>,
    pub model : Option<Model>,
    pub nextservicedate : Option<NextServiceDate>,
    pub notes : Option<Notes>,
    pub purchase : Option<Purchase>,
    pub serialnumber : Option<SerialNumber>,
    pub serviceinterval : Option<ServiceInterval>,
}

crate::test_deserialization!(
    super::EquipmentItem,
    r#"
        <fins id="my_red_fins">
        <name>Quickdiver</name>
        <manufacturer>
            <name>DiveFaster</name>
        </manufacturer>
        <model>Flash-of-Lightning</model>
        <!-- no serial number -->
        <purchase>
            <datetime>1960-10-12</datetime>
            <price currency="DM">80.00</price>
            <shop>
                <name>Diver's Choice</name>
                <address>
                    <country>netherlands</country>
                </address>
                <contact>
                    <!-- phone number, email address etc. -->
                </contact>
                <notes>
                    <!-- additional remarks -->
                </notes>
            </shop>
        </purchase>
        <!-- no service interval -->
    </fins>
    "#
);