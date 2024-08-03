use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Instructor{
    pub address : Option<super::address::Address>,
    pub contact : Option<super::contact::Contact>,
    pub notes : Option<super::notes::Notes>,
    pub personal : Option<super::personal::Personal>,
}

crate::test_deserialization!(
    super::Instructor,
    r#"
     <instructor id="di2_ijk>
        <!-- description of dive instructor -->
        <personal>
            <firstname>Ingo</firstname>
            <middlename>Jürgen</middlename>
            <lastname>Knattermann</lastname>
            <sex>male</sex>
            <birthdate>
                <!-- date of birthdate not known -> statements omitted -->
            </birthdate>
            <address>
                <!-- address not known -> statements omitted -->
            </address>
        </personal>
        <contact>
            <language>deutsch</language>
            <!-- phone number unknown -> <phone> element omitted -->
            <email>ijk@knattermanns_tauchschule.de</email>
            <homepage>http://www.knattermanns_tauchschule.de</homepage>
        </contact>
        <notes>
            <!-- here more text-information, pictures etc. -->
        </notes>
    </instructor>
    "#
);