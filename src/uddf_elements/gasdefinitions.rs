use serde::Deserialize;

use super::notes::Notes;
#[derive(Debug, Deserialize, Clone)]
pub struct GasDefinitions{
    pub mix : Vec<Mix>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mix{
    pub aliasname : Option<Vec<String>>,
    pub ar : Option<f32>,
    pub equivalentdepth : Option<f32>,
    pub h2 : Option<f32>,
    pub he : Option<f32>,
    pub maximumoperatingdepth : Option<f32>,
    pub maximumpo2 : Option<f32>,
    pub n2 : Option<f32>,
    pub name : String,
    pub notes : Option<Notes>,
    pub o2 : Option<f32>,
    pub priceperlitre : Option<f32>,
}

//TODO: Make some way to read if in tag

crate::test_deserialization!(
    super::GasDefinitions,
    r#"
    <gasdefinitions>
    <!-- all breathing gases used -->
    <mix id="oxygen_pure">
        <name>pure oxygen</name>
        <o2>1.000</o2>
        <n2>0.000</n2>
        <he>0.000</he>
        <ar>0.000</ar>
        <h2>0.000</h2>
    </mix>
    <mix id="air">
        <name>air</name>
        <o2>0.210</o2>
        <n2>0.790</n2>
        <he>0.000</he>
        <ar>0.000</ar>
        <h2>0.000</h2>
    </mix>
    <mix id="nitrox6040">
        <name>Nitrox60/40</name>  <!-- Nitrox 60 % N2, 40 % O2 -->
        <o2>0.400</o2>
        <n2>0.600</n2>
        <he>0.000</he>
        <ar>0.000</ar>
        <h2>0.000</h2>
    </mix>
    <mix id="trimix">
        <name>Trimix</name>  <!-- Trimix 15 % O2, 40 % N2, 45 % He -->
        <o2>0.150</o2>
        <n2>0.400</n2>
        <he>0.450</he>
        <ar>0.000</ar>
        <h2>0.000</h2>
    </mix>
</gasdefinitions>
    "#
);