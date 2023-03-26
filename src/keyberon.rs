use keyberon::action::Action;
use keyberon::key_code::KeyCode;

/// Kind of dynamic version of Keyberon layers
pub struct KeyberonLayers<T = core::convert::Infallible, K = KeyCode>(Vec<Vec<Vec<Action<T, K>>>>)
where
    T: 'static,
    K: 'static;
