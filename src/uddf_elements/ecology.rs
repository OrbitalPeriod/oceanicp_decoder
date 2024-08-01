use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Ecology{

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