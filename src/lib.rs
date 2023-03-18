use keyberon::layout::Layout;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Error as JsonError;

#[derive(Serialize, Deserialize)]
struct QmkKeymap {
    version: u32,
    notes: String,
    documentation: String,
    keyboard: String,
    keymap: String,
    layout: String,
    layers: Vec<Vec<String>>,
    author: String,
}

pub fn to() {}

#[cfg(test)]
mod tests {
    use crate::JsonError;
    use crate::QmkKeymap;
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
        assert!(qmk_res.is_ok());
    }
}
