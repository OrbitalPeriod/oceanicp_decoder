use super::{notes::Notes, periodicallytaken::PeriodicallyTaken, timespanberforedive::TimeSpanBeforeDive};

use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Drink{
    pub name : Vec<String>,
    pub aliasname : Option<Vec<String>>,
    pub periodicallytaken : Option<PeriodicallyTaken>, 
    pub notes : Option<Notes>,
    pub timespanbeforedive : Option<TimeSpanBeforeDive>,
}

crate::test_deserialization!(
    super::Drink,
    r#"
    <drink>
    <name>Tequila Sunrise</name>
    <!-- not periodically taken :-) -->
    <periodicallytaken>no</periodicallytaken>
    <!-- taken 30 minutes before the dive -->
    <timespanbeforedive>1800.0</timespanbeforedive>
    <notes>
        <para>
            Maybe not very good so short-timed before the dive,
            but I couldn't resist the invitation...
        </para>
    </notes>
</drink>
    "#
);