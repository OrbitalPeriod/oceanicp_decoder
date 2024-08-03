use serde::Deserialize;

use super::{address::Address, contact::Contact, diveinsurance::DiveInsurance, education::Education};
#[derive(Debug, Deserialize, Clone)]
pub struct Owner {
    pub address: Option<Address>,
    pub contact: Option<Contact>,
    pub diveinsurances: Option<DiveInsurance>,
    pub education : Option<Education>,
}

crate::test_deserialization!(
    super::Owner,
    r#"
    <owner id="robertpeterroth">
            <personal>
                <firstname>Robert</firstname>
                <middlename>Peter</middlename>
                <lastname>Roth</lastname>
                <honorific>Dr.</honorific>
                <sex>male</sex>
                <birthdate>
                    <datetime>1954-07-22</datetime>
                </birthdate>
            </personal>
            <address>
                <street>Newtonstr. 123</street>
                <city>Turingtown</city>
                <postcode>54345</postcode>
                <country>Texas, USA</country>
            </address>
            <contact>
                <language>English</language>
                <phone>0123/456789</phone>
                <mobilephone>0321/987654</mobilephone>
                <fax>0123/456780</fax>
                <email>robert-peter.roth@robert-peter-roth.org</email>
                <homepage>http://www.robert-peter-roth.org</homepage>
            </contact>
            <education>
                <!-- all levels of diving education -> several -->
                <!-- <certification> elements one after the other -->
                <certification>
                    <level>OWD</level>
                    <organisation>PADI</organisation>
                    <!-- because the data of the then diving instructor were not -->
                    <!-- written into the UDDF file a cross refence via -->
                    <!-- <link ref="..."/> is omitted -->
                    <issuedate>
                        <datetime>1994-03-15</datetime>
                    </issuedate>
                </certification>
                <certification>
                    <level>AOWD</level>
                    <organisation>PADI</organisation>
                    <!-- because the data of the then diving instructor were not -->
                    <!-- written into the UDDF file a cross refence via -->
                    <!-- <link ref="..."/> is omitted -->
                    <issuedate>
                        <datetime>1997-11-26</datetime>
                    </issuedate>
                </certification>
                <certification>
                    <level>Divemaster</level>
                    <organisation>PADI</organisation>
                    <!-- because the data of the then diving instructor were not -->
                    <!-- written into the UDDF file a cross refence via -->
                    <!-- <link ref="..."/> is omitted -->
                    <issuedate>
                        <datetime>2000-05-10</datetime>
                    </issuedate>
                </certification>
            </education>
            <divepermissions>
                <!-- as many dive permissions as needed can be listed here -->
                <permit>
                    <name>DiveCard</name>
                    <region>Austria</region>
                    <issuedate>
                        <datetime>2004-08-24</datetime>
                    </issuedate>
                    <validdate>
                        <datetime>2005-08-23</datetime>
                    </validdate>
                </permit>
                <permit>
                    <name>Zeeland</name>
                    <region>Zeeland (The Netherlands)</region>
                    <issuedate>
                        <datetime>1996-09-03</datetime>
                    </issuedate>
                    <validdate>
                        <datetime>2001-08-31</datetime>
                    </validdate>
                </permit>
                <!-- here more permissions if necessary -->
            </divepermissions>
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
            <notes>
                <link ref="img_my_equipment_and_me_1978"/>
                <link ref="img_my_first_divecomputer_1992"/>
                <link ref="img_dive_1111-our_group"/>
                <link ref="audio_talk_for_celebration of dive_888"/>
                <link ref="video_party_dive_1000"/>
            </notes>
        </owner>
    "#
);
