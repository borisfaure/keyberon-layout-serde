use keyberon::key_code::KeyCode;

/// Kind of dynamic version of Keyberon layers
#[derive(Debug, Eq, PartialEq)]
pub struct Layers {
    pub cols: usize,
    pub rows: usize,
    pub is_split: bool,
    pub layers: Vec<Vec<Vec<Action>>>,
}

impl Default for Layers {
    fn default() -> Self {
        Self::new()
    }
}

impl Layers {
    pub fn new() -> Self {
        Self {
            cols: 0,
            rows: 0,
            is_split: false,
            layers: Vec::new(),
        }
    }
    pub fn with_capacity(nb_cols: usize, nb_rows: usize, nb_layers: usize) -> Self {
        let mut layers = Vec::with_capacity(nb_layers);
        for _ in 0..layers.capacity() {
            let mut rows = Vec::with_capacity(nb_rows);
            for _ in 0..rows.capacity() {
                let mut cols = Vec::with_capacity(nb_cols);
                cols.resize(nb_cols, Action::NoOp);
                rows.push(cols);
            }
            layers.push(rows);
        }
        Self {
            cols: nb_cols,
            rows: nb_rows,
            is_split: false,
            layers,
        }
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
