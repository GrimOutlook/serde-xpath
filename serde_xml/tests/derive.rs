#[derive(serde_xml::Deserialize, PartialEq, Debug)]
// All attributes of this struct will have thier XPATHs made relative to this
// path
#[xpath("/magicdraw/signal")]
struct TestStruct {
    // `serde_xml::Text` placed after the XPATH indicates that the text
    // contained in the given XPATH should be deserialized instead of the
    // entire element iteself.
    #[xpath("/name", serde_xml::Text)]
    name: String,

    // Gets the `id="..."` value from the parent XPATH.
    #[xpath("/@id")]
    id: String,

    // Returns `Option::default()` if no `<general>` tag is found in the parent
    // XPATH. Also returns `Option::default()` if no `<signal>` tag is
    // found within the `<general>` tag.
    #[serde(default)]
    // Returs the given XPATH deserialize as a `Generalization` struct instance.
    #[xpath("/general/signal")]
    generalization: Option<Generalization>,

    // Return `Vec::default()` if no `<member>` tag is found in the parent
    // XPATH.
    #[serde(default)]
    // Returns each `<property>` element deserialized into a `Property` struct
    // instance.
    #[xpath("/member/property")]
    properties: Vec<Property>,
}

#[derive(serde_xml::Deserialize, PartialEq, Debug)]
struct Generalization {
    #[xpath("/@refid")]
    refid: String,
    #[xpath("/@name")]
    name: String,
}

#[derive(serde_xml::Deserialize, PartialEq, Debug)]
struct Property {
    #[xpath("/@refid")]
    refid: String,
    #[xpath("/@name")]
    name: String,
}

#[test]
fn test_derive() {
    let input = r#"
<magicdraw>
  <signal id="SIGNAL_ID">
    <name>SIGNAL_NAME</name>
    <member>
      <property refid="PROP1_REFID" name="PROP1_NAME" />
      <property refid="PROP2_REFID" name="PROP2_NAME" />
    </member>
    <general>
      <signal refid="GENERAL_ID" name="GENERAL_NAME" />
    </general>
  </signal>
</magicdraw>
"#;
    assert_eq!(
        serde_xml::from_str::<TestStruct>(input).unwrap(),
        TestStruct {
            name: "SIGNAL_NAME".to_string(),
            id: "SIGNAL_ID".to_string(),
            size: 20usize,
            properties: vec![
                Property {
                    refid: "PROP1_REFID".to_string(),
                    name: "PROP1_NAME".to_string(),
                },
                Property {
                    refid: "PROP2_REFID".to_string(),
                    name: "PROP2_NAME".to_string(),
                }
            ],
            generalization: Some(Generalization {
                refid: "GENERAL_ID".to_string(),
                name: "GENERAL_NAME".to_string(),
            })
        }
    )
}
