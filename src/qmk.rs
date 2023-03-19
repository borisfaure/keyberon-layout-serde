use crate::qmk_keycodes::QmkKeyCode;
use serde::Deserialize;
use serde_json;
use serde_json::Error as JsonError;

#[derive(Deserialize, Debug)]
pub struct QmkKeymap {
    pub version: u32,
    pub notes: String,
    pub documentation: String,
    pub keyboard: String,
    pub keymap: String,
    pub layout: String,
    pub layers: Vec<Vec<QmkKeyCode>>,
    pub author: String,
}

#[cfg(test)]
mod qmk_tests {
    use crate::qmk::JsonError;
    use crate::qmk::QmkKeymap;
    #[test]
    fn test_simple_one_layer() {
        let json = r#"
{
  "version": 1,
  "notes": "",
  "documentation": "",
  "keyboard": "pad",
  "keymap": "map",
  "layout": "LAYOUT_ortho_1x5",
  "layers": [
    [
      "KC_CAPS",
      "KC_PGUP",
      "KC_DEL",
      "KC_END",
      "KC_PGDN"
    ]
  ],
  "author": ""
}"#;
        let qmk_res: Result<QmkKeymap, JsonError> = serde_json::from_str(&json);
        println!("{:?}", qmk_res);
        assert!(qmk_res.is_ok());
    }
}
