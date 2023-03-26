use crate::qmk_keycodes::QmkKeyCode;
use serde::de::{self, Deserializer, IntoDeserializer};
use serde::Deserialize;
use serde_json::Error as JsonError;

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
    /// LT()
    LayerWhenHeldOr(u8, Box<QmkAction>),
    /// ANY()
    Any(Box<QmkAction>),
    /// LSFT()
    LeftShift(Box<QmkAction>),
    /// LCTL()
    LeftControl(Box<QmkAction>),
    /// LALT()
    LeftAlt(Box<QmkAction>),
    /// LGUI()
    LeftGui(Box<QmkAction>),
    /// RSFT()
    RightShift(Box<QmkAction>),
    /// RCTL()
    RightControl(Box<QmkAction>),
    /// RALT()
    RightAlt(Box<QmkAction>),
    /// RGUI()
    RightGui(Box<QmkAction>),
    /// LCA()
    LeftControlAlt(Box<QmkAction>),
    /// LSA()
    LeftShiftAlt(Box<QmkAction>),
    /// SGUI()
    LeftShiftGui(Box<QmkAction>),
    /// LAG()
    LeftAltGui(Box<QmkAction>),
    /// RCA()
    RightControlAlt(Box<QmkAction>),
    /// RSA()
    RightShiftAlt(Box<QmkAction>),
    /// RSG()
    RightShiftGui(Box<QmkAction>),
    /// RAG()
    RightAltGui(Box<QmkAction>),
    /// LCAG()
    LeftControlAltGui(Box<QmkAction>),
    /// MEH()
    Meh(Box<QmkAction>),
    /// HYPER()
    Hyper(Box<QmkAction>),
    /// LSFT_T()
    LeftShiftWhenHeldOr(Box<QmkAction>),
    /// LCTL_T()
    LeftControlWhenHeldOr(Box<QmkAction>),
    /// LALT_T()
    LeftAltWhenHeldOr(Box<QmkAction>),
    /// LGUI_T()
    LeftGuiWhenHeldOr(Box<QmkAction>),
    /// RSFT_T()
    RightShiftWhenHeldOr(Box<QmkAction>),
    /// RCTL_T()
    RightControlWhenHeldOr(Box<QmkAction>),
    /// RALT_T()
    RightAltWhenHeldOr(Box<QmkAction>),
    /// RGUI_T()
    RightGuiWhenHeldOr(Box<QmkAction>),
    /// C_S_T()
    LeftControlShiftWhenHeldOr(Box<QmkAction>),
    /// LCA_T()
    LeftControlAltWhenHeldOr(Box<QmkAction>),
    /// SGUI_T()
    LeftShiftGuiWhenHeldOr(Box<QmkAction>),
    /// LCAG_T()
    LeftControlAltGuiWhenHeldOr(Box<QmkAction>),
    /// RCAG_T()
    RightControlAltGuiWhenHeldOr(Box<QmkAction>),
    /// MEH_T()
    LeftControlShiftAltWhenHeldOr(Box<QmkAction>),
    /// ALL_T()
    LeftControlShiftAltGuiWhenHeldOr(Box<QmkAction>),
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
                        'A'..='Z' | '0'..='9' | '_' => {
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
            ' ' | ',' => {
                pos = it.next();
            }
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
        _ => Err(format!("{}: invalid tokens", func)),
    }
}

/// Parse a function that only accepts a key paramter (usually a keycode)
fn parse_func_kc(func: &str, tokens: &[LexItem]) -> Result<Box<QmkAction>, String> {
    let len = tokens.len();
    if len < 3 {
        return Err(format!(
            "{}: must have at least 3 tokens, got {}",
            func, len
        ));
    }
    match (&tokens[0], &tokens[len - 1]) {
        (LexItem::Parenthesis('('), LexItem::Parenthesis(')')) => {
            Ok(Box::new(parse(&tokens[1..len - 1])?))
        }
        _ => Err(format!("{}: invalid tokens", func)),
    }
}

/// Parse LT(u8, Action)
fn parse_lt(tokens: &[LexItem]) -> Result<QmkAction, String> {
    let len = tokens.len();
    if len < 4 {
        return Err(format!("LT: must have at least 4 tokens, got {}", len));
    }
    match (&tokens[0], &tokens[1], &tokens[len - 1]) {
        (LexItem::Parenthesis('('), LexItem::Number(num), LexItem::Parenthesis(')')) => Ok(
            QmkAction::LayerWhenHeldOr(*num as u8, Box::new(parse(&tokens[2..len - 1])?)),
        ),
        _ => Err(String::from("LT: invalid tokens")),
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
            "LT" => parse_lt(&tokens[1..]),
            "ANY" => Ok(QmkAction::Any(parse_func_kc(tok, &tokens[1..])?)),
            "LSFT" => Ok(QmkAction::LeftShift(parse_func_kc(tok, &tokens[1..])?)),
            "LCTL" => Ok(QmkAction::LeftControl(parse_func_kc(tok, &tokens[1..])?)),
            "LALT" => Ok(QmkAction::LeftAlt(parse_func_kc(tok, &tokens[1..])?)),
            "LGUI" => Ok(QmkAction::LeftGui(parse_func_kc(tok, &tokens[1..])?)),
            "RSFT" => Ok(QmkAction::RightShift(parse_func_kc(tok, &tokens[1..])?)),
            "RCTL" => Ok(QmkAction::RightControl(parse_func_kc(tok, &tokens[1..])?)),
            "RALT" => Ok(QmkAction::RightAlt(parse_func_kc(tok, &tokens[1..])?)),
            "RGUI" => Ok(QmkAction::RightGui(parse_func_kc(tok, &tokens[1..])?)),
            "LCA" => Ok(QmkAction::LeftControlAlt(parse_func_kc(tok, &tokens[1..])?)),
            "LSA" => Ok(QmkAction::LeftShiftAlt(parse_func_kc(tok, &tokens[1..])?)),
            "SGUI" => Ok(QmkAction::LeftShiftGui(parse_func_kc(tok, &tokens[1..])?)),
            "LAG" => Ok(QmkAction::LeftAltGui(parse_func_kc(tok, &tokens[1..])?)),
            "RCA" => Ok(QmkAction::RightControlAlt(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "RSA" => Ok(QmkAction::RightShiftAlt(parse_func_kc(tok, &tokens[1..])?)),
            "RSG" => Ok(QmkAction::RightShiftGui(parse_func_kc(tok, &tokens[1..])?)),
            "RAG" => Ok(QmkAction::RightAltGui(parse_func_kc(tok, &tokens[1..])?)),
            "LCAG" => Ok(QmkAction::LeftControlAltGui(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "MEH" => Ok(QmkAction::Meh(parse_func_kc(tok, &tokens[1..])?)),
            "HYPER" => Ok(QmkAction::Hyper(parse_func_kc(tok, &tokens[1..])?)),
            "LSFT_T" => Ok(QmkAction::LeftShiftWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "LCTL_T" => Ok(QmkAction::LeftControlWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "LALT_T" => Ok(QmkAction::LeftAltWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "LGUI_T" => Ok(QmkAction::LeftGuiWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "RSFT_T" => Ok(QmkAction::RightShiftWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "RCTL_T" => Ok(QmkAction::RightControlWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "RALT_T" => Ok(QmkAction::RightAltWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "RGUI_T" => Ok(QmkAction::RightGuiWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "C_S_T" => Ok(QmkAction::LeftControlShiftWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "LCA_T" => Ok(QmkAction::LeftControlAltWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "SGUI_T" => Ok(QmkAction::LeftShiftGuiWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "LCAG_T" => Ok(QmkAction::LeftControlAltGuiWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "RCAG_T" => Ok(QmkAction::RightControlAltGuiWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "MEH_T" => Ok(QmkAction::LeftControlShiftAltWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),
            "ALL_T" => Ok(QmkAction::LeftControlShiftAltGuiWhenHeldOr(parse_func_kc(
                tok,
                &tokens[1..],
            )?)),

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

impl QmkKeymap {
    /// Deserialize a JSON string @json into a QmkKeymap
    pub fn from_json_str(json: &str) -> Result<Self, JsonError> {
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
        let qmk_res = QmkKeymap::from_json_str(&json);
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
        let qmk_res = QmkKeymap::from_json_str(&json);
        println!("{:?}", qmk_res);
        assert!(qmk_res.is_ok());
    }

    #[test]
    fn test_kc_recursion() {
        let json = r#"{
  "version": 1, "notes": "", "documentation": "", "keyboard": "", "keymap": "",
  "layout": "", "author": "",
  "layers": [ [
      "LSFT_T(KC_A)",
      "LCTL_T(KC_B)",
      "LCA(KC_LSFT)",
      "ANY(LCTL(LALT(KC_DEL)))",
      "RCTL(KC_LALT)",
      "ANY(KC_ENTER)"
  ]]}"#;
        let qmk_res = QmkKeymap::from_json_str(&json);
        println!("{:?}", qmk_res);
        assert!(qmk_res.is_ok());
    }

    #[test]
    fn test_kc_lt() {
        let json = r#"{
  "version": 1, "notes": "", "documentation": "", "keyboard": "", "keymap": "",
  "layout": "", "author": "",
  "layers": [ [
      "LT(0,KC_A)",
      "LT(1,LSFT(KC_B))"
  ]]}"#;
        let qmk_res = QmkKeymap::from_json_str(&json);
        println!("{:?}", qmk_res);
        assert!(qmk_res.is_ok());
    }
}
