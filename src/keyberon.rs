use keyberon::action::Action;
use keyberon::key_code::KeyCode;

/// Kind of dynamic version of Keyberon layers
pub struct KeyberonLayers<T = core::convert::Infallible, K = KeyCode>(Vec<Vec<Vec<Action<T, K>>>>)
where
    T: 'static,
    K: 'static;

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
