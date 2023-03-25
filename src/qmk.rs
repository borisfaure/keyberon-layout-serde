use crate::qmk_keycodes::QmkKeyCode;
use serde::de::{self, Deserializer, IntoDeserializer};
use serde::Deserialize;
use serde_json::Error as JsonError;
use std::str::FromStr;

#[derive(Debug)]
pub enum QmkAction {
    /// A key code
    KeyCode(QmkKeyCode),
    /// MO()
    MomentaryTurnLayerOn(u8),
    /// TG()
    ToggleLayer(u8),
    /// TO()
    TurnOnLayerWhenPressed(u8),
    /// TT()
    TapToggleLayer(u8),
    /// DF()
    DefaultLayer(u8),
    /// OSL()
    OneShotLayer(u8),
}

/// Lexer item
#[derive(Debug, Clone)]
enum LexItem<'a> {
    /// A token
    Token(&'a str),
    /// A parenthesis opening or closing
    Parenthesis(char),
    /// A number
    Number(usize),
}

/// Transform @input into a vector of tokens
fn lex(input: &'_ str) -> Result<Vec<LexItem<'_>>, String> {
    let mut result = Vec::new();

    let chars = input.chars();
    let mut it = chars.enumerate();
    let mut pos = it.next();
    while let Some((i, c)) = pos {
        match c {
            '0'..='9' => {
                let mut num: usize = 0;
                while let Some((_i, c)) = pos {
                    match c {
                        '0'..='9' => {
                            num = (c as usize - '0' as usize) + num * 10;
                            pos = it.next();
                        }
                        _ => break,
                    }
                }
                result.push(LexItem::Number(num));
            }
            'A'..='Z' | '_' => {
                let start = i;
                let mut end = i;
                pos = it.next();
                while let Some((i, c)) = pos {
                    match c {
                        'A'..='Z' | '_' => {
                            end = i;
                            pos = it.next();
                        }
                        _ => break,
                    }
                }
                result.push(LexItem::Token(&input[start..=end]));
            }
            '(' | ')' => {
                result.push(LexItem::Parenthesis(c));
                pos = it.next();
            }
            ' ' | ',' => {}
            _ => {
                return Err(format!("caracter '{}'@{} while parsing {}", c, i, input));
            }
        }
    }
    Ok(result)
}

/// Parse a function that only accepts a u8 (usually a layer)
fn parse_func_u8(func: &str, tokens: &[LexItem]) -> Result<u8, String> {
    if tokens.len() != 3 {
        return Err(format!(
            "{}: must have 3 tokens, got {}",
            func,
            tokens.len()
        ));
    }
    match (&tokens[0], &tokens[1], &tokens[2]) {
        (LexItem::Parenthesis('('), LexItem::Number(num), LexItem::Parenthesis(')')) => {
            Ok(*num as u8)
        }
        _ => Err(String::from("func: invalid tokens")),
    }
}

/// Parse a vector of tokens
fn parse(tokens: &[LexItem]) -> Result<QmkAction, String> {
    if tokens.is_empty() {
        return Err(String::from("no tokens"));
    }
    if tokens.len() == 1 {
        if let LexItem::Token(tok) = tokens[0] {
            let d: de::value::StrDeserializer<de::value::Error> = tok.into_deserializer();
            if let Ok(kc) = QmkKeyCode::deserialize(d) {
                return Ok(QmkAction::KeyCode(kc));
            } else {
                return Err(format!("invalid keycode '{}'", tok));
            }
        } else {
            return Err(String::from("Expecting token"));
        }
    }
    if let LexItem::Token(tok) = tokens[0] {
        match tok {
            "MO" => Ok(QmkAction::MomentaryTurnLayerOn(parse_func_u8(
                tok,
                &tokens[1..],
            )?)),
            "TG" => Ok(QmkAction::ToggleLayer(parse_func_u8(tok, &tokens[1..])?)),
            "TO" => Ok(QmkAction::TurnOnLayerWhenPressed(parse_func_u8(
                tok,
                &tokens[1..],
            )?)),
            "TT" => Ok(QmkAction::TapToggleLayer(parse_func_u8(tok, &tokens[1..])?)),
            "DF" => Ok(QmkAction::DefaultLayer(parse_func_u8(tok, &tokens[1..])?)),
            "OSL" => Ok(QmkAction::OneShotLayer(parse_func_u8(tok, &tokens[1..])?)),
            _ => Err(format!("unsupported command {}", tok)),
        }
    } else {
        Err(String::from("Expecting token"))
    }
}

impl<'de> Deserialize<'de> for QmkAction {
    fn deserialize<D>(deserializer: D) -> Result<QmkAction, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let tokens = lex(&s);
        if let Err(s) = tokens {
            return Err(de::Error::custom(s));
        }
        let tokens = tokens.unwrap();
        match parse(tokens.as_slice()) {
            Ok(qa) => Ok(qa),
            Err(s) => Err(de::Error::custom(s)),
        }
    }
}

/// The basic structure for the layout JSON files used in
/// [qmk configurator](https://config.qmk.fm/)
#[derive(Deserialize, Debug)]
pub struct QmkKeymap {
    pub version: u32,
    pub notes: String,
    pub documentation: String,
    pub keyboard: String,
    pub keymap: String,
    pub layout: String,
    pub layers: Vec<Vec<QmkAction>>,
    pub author: String,
}

impl FromStr for QmkKeymap {
    type Err = JsonError;
    /// Deserialize a JSON string @json into a QmkKeymap
    fn from_str(json: &str) -> Result<Self, JsonError> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod qmk_tests {
    use crate::qmk::QmkKeymap;
    #[test]
    fn test_basic_one_layer_only_keycodes() {
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
        let qmk_res = QmkKeymap::from_str(&json);
        assert!(qmk_res.is_ok());
    }

    #[test]
    fn test_layers() {
        let json = r#"
{
  "version": 1,
  "notes": "",
  "documentation": "",
  "keyboard": "pad",
  "keymap": "map",
  "layout": "LAYOUT_ortho_1x3",
  "layers": [
    [
      "KC_SPACE",
      "MO(1)",
      "TG(1)"
    ],
    [
      "KC_ENTER",
      "MO(1)",
      "TG(0)"
    ]
  ],
  "author": ""
}"#;
        let qmk_res = QmkKeymap::from_str(&json);
        println!("{:?}", qmk_res);
        assert!(qmk_res.is_ok());
    }
}
