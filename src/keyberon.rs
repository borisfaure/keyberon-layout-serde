use keyberon::key_code::KeyCode;

/// Kind of dynamic version of Keyberon layers
pub struct KeyberonLayers(Vec<Vec<Vec<Action>>>);

impl KeyberonLayers {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}
impl Default for KeyberonLayers {
    fn default() -> Self {
        Self::new()
    }
}

/// The different actions that can be done.
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    NoOp,
    Trans,
    KeyCode(KeyCode),
    MultipleKeyCodes(Vec<KeyCode>),
    MultipleActions(Vec<Action>),
    Layer(usize),
    DefaultLayer(usize),
    HoldTap(Box<Action>, Box<Action>),
}
