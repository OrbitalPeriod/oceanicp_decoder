use serde::Deserialize;

use super::{notes::Notes, species::Species};
#[derive(Debug, Deserialize, Clone)]
pub struct Fauna{
   pub invertebrata :  Option<Invertebrata>,
   pub notes : Option<Notes>,
   pub vertebrata : Option<Vertebrata>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Invertebrata{
    pub ascidiacea : Option<Phylum>,
    pub bryozoa : Option<Phylum>,
    pub coelenterata : Option<Phylum>,
    pub crustacea : Option<Phylum>,
    pub ctenophora : Option<Phylum>,
    pub echinodermata : Option<Phylum>,
    pub invertebratavarious : Option<Phylum>,
    pub mollusca : Option<Phylum>,
    pub phoronidea : Option<Phylum>,
    pub plathelminthes : Option<Phylum>,
    pub porifera : Option<Phylum>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Vertebrata{
    pub amphibia : Option<Phylum>,
    pub chondrichthyes : Option<Phylum>,
    pub mammalia : Option<Phylum>,
    pub osteichthyes : Option<Phylum>,
    pub reptilia : Option<Phylum>,
    pub vertebratavarious : Option<Phylum>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Phylum{
    pub species : Option<Vec<Species>>,
}

crate::test_deserialization!(
    super::Fauna,
    r#"
    <fauna>
    <invertebrata>                <!-- invertebrates -->
        <porifera>                <!-- sponges -->
        </porifera>
        <coelenterata>            <!-- coelenteratas -->
        </coelenterata>
        <cnidaria>                <!-- cnidarians -->
        </cnidaria>
        <ctenophora>              <!-- ctenophoranes -->
        </ctenophora>
        <plathelminthes>          <!-- flatworms -->
        </plathelminthes>
        <bryozoa>                 <!-- bryozoans -->
        </bryozoa>
        <phoronidea>              <!-- phoronids -->
        </phoronidea>
        <ascidiacea>              <!-- seasquirts -->
        </ascidiacea>
        <echinodermata>           <!-- echinoderms -->
        </echinodermata>
        <mollusca>                <!-- molluscs -->
        </mollusca>
        <crustacea>               <!-- crustaceans -->
        </crustacea>
        <invertebratavarious>     <!-- all other invertebrates -->
        </invertebratavarious>
    </invertebrata>
    <vertebrata>                  <!-- vertebrates -->
        <chondrichthyes>          <!-- cartilaginous fishes - sharks, rays -->
        </chondrichthyes>
        <osteichthyes>            <!-- Knochenfische -->
        </osteichthyes>
        <mammalia>                <!-- mammalians -->
        </mammalia>
        <amphibia>                <!-- amphibians -->
        </amphibia>
        <reptilia>                <!-- reptiles -->
        </reptilia>
        <vertebratavarious>       <!-- all other vertebrates-->
        </vertebratavarious>
    </vertebrata>
</fauna>
    "#
);