use serde::Deserialize;

use super::{fauna::Fauna, flora::Flora};
#[derive(Debug, Deserialize, Clone)]
pub struct Ecology{
    pub fauna : Option<Fauna>,
    pub flora : Option<Flora>,
}

crate::test_deserialization!(
    super::Ecology,
    r#"
    <ecology>
    <fauna>
        <!-- here typically found animal species at this dive spot -->
    </fauna>
    <flora>
        <!-- here typically found plant species at this dive spot -->
    </flora>
</ecology>
    "#
);