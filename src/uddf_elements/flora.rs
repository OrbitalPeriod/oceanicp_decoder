use serde::Deserialize;

use super::{fauna::Phylum, notes::Notes};
#[derive(Debug, Deserialize, Clone)]
pub struct Flora{
    pub chlorophyceae : Option<Phylum>,
    pub floravarious : Option<Phylum>,
    pub notes : Option<Notes>,
    pub phaeophyceae : Option<Phylum>,
    pub rhodophyceae : Option<Phylum>,
    pub spermatophyta : Option<Phylum>,
}

crate::test_deserialization!(
    super::Flora,
    r#"
    <flora>
    <rhodophyceae>      <!-- red algae -->
    </rhodophyceae>
    <phaeophyceae>      <!-- brown algae -->
    </phaeophyceae>
    <chlorophyceae>     <!-- green algae -->
    </chlorophyceae>
    <spermatophyta>     <!-- flowering plants -->
    </spermatophyta>
    <floravarious>      <!-- all other plants -->
    </floravarious>
</flora>
    "#
);