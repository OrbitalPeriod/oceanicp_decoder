use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Notes{
    #[serde(rename = "para", default)]
    pub para : Vec<String>,
}

crate::test_deserialization!(
    super::Notes,
    r#"
    <notes>
    <para>
        This is text in-between the text element. Using this, it's not possible
        to further format the text. The executing program formats the text.
    </para>
    <link ref="image_1"/>
    <link ref="image_2"/>
    <para>
        Now an audio file and a video sequence do follow...
    </para>
    <link ref="audio_1"/>
    <link ref="video_1"/>
</notes>
    "#
);

