use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Output{
    pub lingo : Option<String>,
    pub fileformat : Option<String>,
    pub filename : Option<String>,
    pub headline : Option<String>,
    pub remark : Option<String>,
}

crate::test_deserialization!(
    super::Output,
    r#"
    <output>
    <!-- English is the language to be used in the output -->
    <lingo>en</lingo>
    <fileformat>tex</fileformat>
    <!-- The file extension must not be given in the file name, because -->
    <!-- it is specified through the output format in the line above -->
    <filename>kais_tab1</filename>
    <headline>Kai_s Special Decompression Table</headline>
    <remark>
        The best decompression table ever generated!!! ;-)
    </remark>
</output>
    "#
);