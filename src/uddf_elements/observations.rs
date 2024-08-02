use serde::Deserialize;

use super::{fauna::Fauna, flora::Flora, notes::Notes};
#[derive(Debug, Deserialize, Clone)]
pub struct Observations{
    pub fauna : Option<Fauna>,
    pub flora : Option<Flora>,
    pub notes : Option<Notes>,
}

crate::test_deserialization!(
    super::Observations,
    r#"
    <observations>
    <fauna>
        <!-- here listed all animals seen during the dive -->
    </fauna>
    <flora>
        <!-- here listed all plants recognized during the dive -->
    </flora>
    <notes>
        <!-- here additional general information (text, images etc.) -->
    </notes>
</observations>
    "#
);