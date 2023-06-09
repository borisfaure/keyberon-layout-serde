use crate::keyberon::{Action, Layers};
use crate::qmk::{QmkAction, QmkKeyMap};
use crate::qmk_keycodes::QmkKeyCode;
use keyberon::key_code::KeyCode;
use log::warn;
use std::convert::TryFrom;

impl TryFrom<&QmkKeyCode> for KeyCode {
    type Error = String;

    fn try_from(qmk: &QmkKeyCode) -> Result<Self, Self::Error> {
        match qmk {
            QmkKeyCode::KcA => Ok(KeyCode::A),
            QmkKeyCode::KcB => Ok(KeyCode::B),
            QmkKeyCode::KcC => Ok(KeyCode::C),
            QmkKeyCode::KcD => Ok(KeyCode::D),
            QmkKeyCode::KcE => Ok(KeyCode::E),
            QmkKeyCode::KcF => Ok(KeyCode::F),
            QmkKeyCode::KcG => Ok(KeyCode::G),
            QmkKeyCode::KcH => Ok(KeyCode::H),
            QmkKeyCode::KcI => Ok(KeyCode::I),
            QmkKeyCode::KcJ => Ok(KeyCode::J),
            QmkKeyCode::KcK => Ok(KeyCode::K),
            QmkKeyCode::KcL => Ok(KeyCode::L),
            QmkKeyCode::KcM => Ok(KeyCode::M),
            QmkKeyCode::KcN => Ok(KeyCode::N),
            QmkKeyCode::KcO => Ok(KeyCode::O),
            QmkKeyCode::KcP => Ok(KeyCode::P),
            QmkKeyCode::KcQ => Ok(KeyCode::Q),
            QmkKeyCode::KcR => Ok(KeyCode::R),
            QmkKeyCode::KcS => Ok(KeyCode::S),
            QmkKeyCode::KcT => Ok(KeyCode::T),
            QmkKeyCode::KcU => Ok(KeyCode::U),
            QmkKeyCode::KcV => Ok(KeyCode::V),
            QmkKeyCode::KcW => Ok(KeyCode::W),
            QmkKeyCode::KcX => Ok(KeyCode::X),
            QmkKeyCode::KcY => Ok(KeyCode::Y),
            QmkKeyCode::KcZ => Ok(KeyCode::Z),
            QmkKeyCode::Kc1 => Ok(KeyCode::Kb1),
            QmkKeyCode::Kc2 => Ok(KeyCode::Kb2),
            QmkKeyCode::Kc3 => Ok(KeyCode::Kb3),
            QmkKeyCode::Kc4 => Ok(KeyCode::Kb4),
            QmkKeyCode::Kc5 => Ok(KeyCode::Kb5),
            QmkKeyCode::Kc6 => Ok(KeyCode::Kb6),
            QmkKeyCode::Kc7 => Ok(KeyCode::Kb7),
            QmkKeyCode::Kc8 => Ok(KeyCode::Kb8),
            QmkKeyCode::Kc9 => Ok(KeyCode::Kb9),
            QmkKeyCode::Kc0 => Ok(KeyCode::Kb0),
            QmkKeyCode::KcEnter => Ok(KeyCode::Enter),
            QmkKeyCode::KcEscape => Ok(KeyCode::Escape),
            QmkKeyCode::KcBackSpace => Ok(KeyCode::BSpace),
            QmkKeyCode::KcTab => Ok(KeyCode::Tab),
            QmkKeyCode::KcSpace => Ok(KeyCode::Space),
            QmkKeyCode::KcMinus => Ok(KeyCode::Minus),
            QmkKeyCode::KcEqual => Ok(KeyCode::Equal),
            QmkKeyCode::KcLeftBracket => Ok(KeyCode::LBracket),
            QmkKeyCode::KcRightBracket => Ok(KeyCode::RBracket),
            QmkKeyCode::KcBackSlash => Ok(KeyCode::Bslash),
            QmkKeyCode::KcNonUsHash => Ok(KeyCode::NonUsHash),
            QmkKeyCode::KcSemiColon => Ok(KeyCode::SColon),
            QmkKeyCode::KcQuote => Ok(KeyCode::Quote),
            QmkKeyCode::KcGrave => Ok(KeyCode::Grave),
            QmkKeyCode::KcComma => Ok(KeyCode::Comma),
            QmkKeyCode::KcDot => Ok(KeyCode::Dot),
            QmkKeyCode::KcSlash => Ok(KeyCode::Slash),
            QmkKeyCode::KcCapsLock => Ok(KeyCode::CapsLock),
            QmkKeyCode::KcF1 => Ok(KeyCode::F1),
            QmkKeyCode::KcF2 => Ok(KeyCode::F2),
            QmkKeyCode::KcF3 => Ok(KeyCode::F3),
            QmkKeyCode::KcF4 => Ok(KeyCode::F4),
            QmkKeyCode::KcF5 => Ok(KeyCode::F5),
            QmkKeyCode::KcF6 => Ok(KeyCode::F6),
            QmkKeyCode::KcF7 => Ok(KeyCode::F7),
            QmkKeyCode::KcF8 => Ok(KeyCode::F8),
            QmkKeyCode::KcF9 => Ok(KeyCode::F9),
            QmkKeyCode::KcF10 => Ok(KeyCode::F10),
            QmkKeyCode::KcF11 => Ok(KeyCode::F11),
            QmkKeyCode::KcF12 => Ok(KeyCode::F12),
            QmkKeyCode::KcPrintScreen => Ok(KeyCode::PScreen),
            QmkKeyCode::KcScrollLock => Ok(KeyCode::ScrollLock),
            QmkKeyCode::KcPause => Ok(KeyCode::Pause),
            QmkKeyCode::KcInsert => Ok(KeyCode::Insert),
            QmkKeyCode::KcHome => Ok(KeyCode::Home),
            QmkKeyCode::KcPageUp => Ok(KeyCode::PgUp),
            QmkKeyCode::KcDel => Ok(KeyCode::Delete),
            QmkKeyCode::KcEnd => Ok(KeyCode::End),
            QmkKeyCode::KcPageDown => Ok(KeyCode::PgDown),
            QmkKeyCode::KcRight => Ok(KeyCode::Right),
            QmkKeyCode::KcLeft => Ok(KeyCode::Left),
            QmkKeyCode::KcDown => Ok(KeyCode::Down),
            QmkKeyCode::KcUp => Ok(KeyCode::Up),
            QmkKeyCode::KcNumLock => Ok(KeyCode::NumLock),
            QmkKeyCode::KcKpSlash => Ok(KeyCode::KpSlash),
            QmkKeyCode::KcKpAsterix => Ok(KeyCode::KpAsterisk),
            QmkKeyCode::KcKpMinus => Ok(KeyCode::KpMinus),
            QmkKeyCode::KcKpPlus => Ok(KeyCode::KpPlus),
            QmkKeyCode::KcKpEnter => Ok(KeyCode::KpEnter),
            QmkKeyCode::KcKp1 => Ok(KeyCode::Kp1),
            QmkKeyCode::KcKp2 => Ok(KeyCode::Kp2),
            QmkKeyCode::KcKp3 => Ok(KeyCode::Kp3),
            QmkKeyCode::KcKp4 => Ok(KeyCode::Kp4),
            QmkKeyCode::KcKp5 => Ok(KeyCode::Kp5),
            QmkKeyCode::KcKp6 => Ok(KeyCode::Kp6),
            QmkKeyCode::KcKp7 => Ok(KeyCode::Kp7),
            QmkKeyCode::KcKp8 => Ok(KeyCode::Kp8),
            QmkKeyCode::KcKp9 => Ok(KeyCode::Kp9),
            QmkKeyCode::KcKp0 => Ok(KeyCode::Kp0),
            QmkKeyCode::KcKpDot => Ok(KeyCode::KpDot),
            QmkKeyCode::KcNonUsBackslash => Ok(KeyCode::NonUsBslash),
            QmkKeyCode::KcApplication => Ok(KeyCode::Application),
            QmkKeyCode::KcKbPower => Ok(KeyCode::Power),
            QmkKeyCode::KcKpEqual => Ok(KeyCode::KpEqual),
            QmkKeyCode::KcF13 => Ok(KeyCode::F13),
            QmkKeyCode::KcF14 => Ok(KeyCode::F14),
            QmkKeyCode::KcF15 => Ok(KeyCode::F15),
            QmkKeyCode::KcF16 => Ok(KeyCode::F16),
            QmkKeyCode::KcF17 => Ok(KeyCode::F17),
            QmkKeyCode::KcF18 => Ok(KeyCode::F18),
            QmkKeyCode::KcF19 => Ok(KeyCode::F19),
            QmkKeyCode::KcF20 => Ok(KeyCode::F20),
            QmkKeyCode::KcF21 => Ok(KeyCode::F21),
            QmkKeyCode::KcF22 => Ok(KeyCode::F22),
            QmkKeyCode::KcF23 => Ok(KeyCode::F23),
            QmkKeyCode::KcF24 => Ok(KeyCode::F24),
            QmkKeyCode::KcExecute => Ok(KeyCode::Execute),
            QmkKeyCode::KcHelp => Ok(KeyCode::Help),
            QmkKeyCode::KcMenu => Ok(KeyCode::Menu),
            QmkKeyCode::KcSelect => Ok(KeyCode::Select),
            QmkKeyCode::KcStop => Ok(KeyCode::Stop),
            QmkKeyCode::KcAgain => Ok(KeyCode::Again),
            QmkKeyCode::KcUndo => Ok(KeyCode::Undo),
            QmkKeyCode::KcCut => Ok(KeyCode::Cut),
            QmkKeyCode::KcCopy => Ok(KeyCode::Copy),
            QmkKeyCode::KcPaste => Ok(KeyCode::Paste),
            QmkKeyCode::KcFind => Ok(KeyCode::Find),
            QmkKeyCode::KcKbMute => Ok(KeyCode::Mute),
            QmkKeyCode::KcKbVolumeUp => Ok(KeyCode::VolUp),
            QmkKeyCode::KcKbVolumeDown => Ok(KeyCode::VolDown),
            QmkKeyCode::KcLockingCapsLock => Ok(KeyCode::LockingCapsLock),
            QmkKeyCode::KcLockingNumLock => Ok(KeyCode::LockingNumLock),
            QmkKeyCode::KcLockingScrollLock => Ok(KeyCode::LockingScrollLock),
            QmkKeyCode::KcKpComma => Ok(KeyCode::KpComma),
            QmkKeyCode::KcKpEqualAs400 => Ok(KeyCode::KpEqualSign),
            QmkKeyCode::KcInternational1 => Ok(KeyCode::Intl1),
            QmkKeyCode::KcInternational2 => Ok(KeyCode::Intl2),
            QmkKeyCode::KcInternational3 => Ok(KeyCode::Intl3),
            QmkKeyCode::KcInternational4 => Ok(KeyCode::Intl4),
            QmkKeyCode::KcInternational5 => Ok(KeyCode::Intl5),
            QmkKeyCode::KcInternational6 => Ok(KeyCode::Intl6),
            QmkKeyCode::KcInternational7 => Ok(KeyCode::Intl7),
            QmkKeyCode::KcInternational8 => Ok(KeyCode::Intl8),
            QmkKeyCode::KcInternational9 => Ok(KeyCode::Intl9),
            QmkKeyCode::KcLanguage1 => Ok(KeyCode::Lang1),
            QmkKeyCode::KcLanguage2 => Ok(KeyCode::Lang2),
            QmkKeyCode::KcLanguage3 => Ok(KeyCode::Lang3),
            QmkKeyCode::KcLanguage4 => Ok(KeyCode::Lang4),
            QmkKeyCode::KcLanguage5 => Ok(KeyCode::Lang5),
            QmkKeyCode::KcLanguage6 => Ok(KeyCode::Lang6),
            QmkKeyCode::KcLanguage7 => Ok(KeyCode::Lang7),
            QmkKeyCode::KcLanguage8 => Ok(KeyCode::Lang8),
            QmkKeyCode::KcLanguage9 => Ok(KeyCode::Lang9),
            QmkKeyCode::KcAlternateErase => Ok(KeyCode::AltErase),
            QmkKeyCode::KcSystemRequest => Ok(KeyCode::SysReq),
            QmkKeyCode::KcCancel => Ok(KeyCode::Cancel),
            QmkKeyCode::KcClear => Ok(KeyCode::Clear),
            QmkKeyCode::KcPrior => Ok(KeyCode::Prior),
            QmkKeyCode::KcReturn => Ok(KeyCode::Return),
            QmkKeyCode::KcSeparator => Ok(KeyCode::Separator),
            QmkKeyCode::KcOut => Ok(KeyCode::Out),
            QmkKeyCode::KcOper => Ok(KeyCode::Oper),
            QmkKeyCode::KcClearAgain => Ok(KeyCode::ClearAgain),
            QmkKeyCode::KcCrSel => Ok(KeyCode::CrSel),
            QmkKeyCode::KcExSel => Ok(KeyCode::ExSel),
            QmkKeyCode::KcLeftCtrl => Ok(KeyCode::LCtrl),
            QmkKeyCode::KcLeftShift => Ok(KeyCode::LShift),
            QmkKeyCode::KcLeftAlt => Ok(KeyCode::LAlt),
            QmkKeyCode::KcLeftGui => Ok(KeyCode::LGui),
            QmkKeyCode::KcRightCtrl => Ok(KeyCode::RCtrl),
            QmkKeyCode::KcRightShift => Ok(KeyCode::RShift),
            QmkKeyCode::KcRightAlt => Ok(KeyCode::RAlt),
            QmkKeyCode::KcRightGui => Ok(KeyCode::RGui),
            QmkKeyCode::KcMediaPlayPause => Ok(KeyCode::MediaPlayPause),
            QmkKeyCode::KcMediaStop => Ok(KeyCode::MediaStopCD),
            QmkKeyCode::KcMediaPrevTrack => Ok(KeyCode::MediaPreviousSong),
            QmkKeyCode::KcMediaNextTrack => Ok(KeyCode::MediaNextSong),
            QmkKeyCode::KcMediaEject => Ok(KeyCode::MediaEjectCD),
            QmkKeyCode::KcAudioVolUp => Ok(KeyCode::MediaVolUp),
            QmkKeyCode::KcAudioVolDown => Ok(KeyCode::MediaVolDown),
            QmkKeyCode::KcAudioMute => Ok(KeyCode::MediaMute),
            QmkKeyCode::KcWwwHome => Ok(KeyCode::MediaWWW),
            QmkKeyCode::KcWwwBack => Ok(KeyCode::MediaBack),
            QmkKeyCode::KcWwwForward => Ok(KeyCode::MediaForward),
            QmkKeyCode::KcWwwStop => Ok(KeyCode::MediaStop),
            QmkKeyCode::KcWwwSearch => Ok(KeyCode::MediaFind),
            QmkKeyCode::KcMediaRewind => Ok(KeyCode::MediaScrollUp),
            QmkKeyCode::KcMediaFastForward => Ok(KeyCode::MediaScrollDown),
            QmkKeyCode::KcSystemSleep => Ok(KeyCode::MediaSleep),
            QmkKeyCode::KcWwwRefresh => Ok(KeyCode::MediaRefresh),
            QmkKeyCode::KcCalculator => Ok(KeyCode::MediaCalc),
            _ => Err(format!("unsupported qmk keycode {:?}", qmk)),
        }
    }
}

/// Try to convert into Keyberon::MultipleKeyCodes
fn try_multiple_keycodes(action: &QmkAction, mut v: Vec<KeyCode>) -> Result<Action, String> {
    match action {
        QmkAction::KeyCode(QmkKeyCode::KcNo) | QmkAction::KeyCode(QmkKeyCode::KcTransparent) => {
            Err(format!(
                "unexpected {:?} when handling multiple keycodes",
                action
            ))
        }
        QmkAction::KeyCode(kc) => {
            v.push(KeyCode::try_from(kc)?);
            Ok(Action::MultipleKeyCodes(v))
        }
        QmkAction::Any(a) => try_multiple_keycodes(a, v),
        QmkAction::LeftShift(a) => {
            v.push(KeyCode::LShift);
            try_multiple_keycodes(a, v)
        }
        QmkAction::LeftControl(a) => {
            v.push(KeyCode::LCtrl);
            try_multiple_keycodes(a, v)
        }
        QmkAction::LeftAlt(a) => {
            v.push(KeyCode::LAlt);
            try_multiple_keycodes(a, v)
        }
        QmkAction::LeftGui(a) => {
            v.push(KeyCode::LGui);
            try_multiple_keycodes(a, v)
        }
        QmkAction::RightShift(a) => {
            v.push(KeyCode::RShift);
            try_multiple_keycodes(a, v)
        }
        QmkAction::RightControl(a) => {
            v.push(KeyCode::RCtrl);
            try_multiple_keycodes(a, v)
        }
        QmkAction::RightAlt(a) => {
            v.push(KeyCode::RAlt);
            try_multiple_keycodes(a, v)
        }
        QmkAction::RightGui(a) => {
            v.push(KeyCode::RGui);
            try_multiple_keycodes(a, v)
        }
        QmkAction::LeftControlAlt(a) => {
            v.push(KeyCode::LCtrl);
            v.push(KeyCode::LAlt);
            try_multiple_keycodes(a, v)
        }
        QmkAction::LeftShiftAlt(a) => {
            v.push(KeyCode::LShift);
            v.push(KeyCode::LAlt);
            try_multiple_keycodes(a, v)
        }
        QmkAction::LeftShiftGui(a) => {
            v.push(KeyCode::LShift);
            v.push(KeyCode::LGui);
            try_multiple_keycodes(a, v)
        }
        QmkAction::LeftAltGui(a) => {
            v.push(KeyCode::LAlt);
            v.push(KeyCode::LGui);
            try_multiple_keycodes(a, v)
        }
        QmkAction::RightControlAlt(a) => {
            v.push(KeyCode::RCtrl);
            v.push(KeyCode::RAlt);
            try_multiple_keycodes(a, v)
        }
        QmkAction::RightShiftAlt(a) => {
            v.push(KeyCode::RShift);
            v.push(KeyCode::RAlt);
            try_multiple_keycodes(a, v)
        }
        QmkAction::RightShiftGui(a) => {
            v.push(KeyCode::RShift);
            v.push(KeyCode::RGui);
            try_multiple_keycodes(a, v)
        }
        QmkAction::RightAltGui(a) => {
            v.push(KeyCode::RAlt);
            v.push(KeyCode::RGui);
            try_multiple_keycodes(a, v)
        }
        QmkAction::Meh(a) => {
            v.push(KeyCode::LCtrl);
            v.push(KeyCode::LShift);
            v.push(KeyCode::LAlt);
            try_multiple_keycodes(a, v)
        }
        QmkAction::Hyper(a) => {
            v.push(KeyCode::LCtrl);
            v.push(KeyCode::LShift);
            v.push(KeyCode::LAlt);
            v.push(KeyCode::LGui);
            try_multiple_keycodes(a, v)
        }
        _ => Err(format!("can not convert {:?} to keyberon action", action)),
    }
}

impl TryFrom<&QmkAction> for Action {
    type Error = String;

    fn try_from(qmk: &QmkAction) -> Result<Self, Self::Error> {
        match qmk {
            QmkAction::KeyCode(QmkKeyCode::KcNo) => Ok(Action::NoOp),
            QmkAction::KeyCode(QmkKeyCode::KcTransparent) => Ok(Action::Trans),
            QmkAction::KeyCode(kc) => Ok(Action::KeyCode(KeyCode::try_from(kc)?)),
            QmkAction::Any(a) => try_multiple_keycodes(a, vec![]),
            QmkAction::LeftShift(_)
            | QmkAction::LeftControl(_)
            | QmkAction::LeftAlt(_)
            | QmkAction::LeftGui(_)
            | QmkAction::RightShift(_)
            | QmkAction::RightControl(_)
            | QmkAction::RightAlt(_)
            | QmkAction::RightGui(_)
            | QmkAction::LeftControlAlt(_)
            | QmkAction::LeftShiftAlt(_)
            | QmkAction::LeftShiftGui(_)
            | QmkAction::LeftAltGui(_)
            | QmkAction::RightControlAlt(_)
            | QmkAction::RightShiftAlt(_)
            | QmkAction::RightShiftGui(_)
            | QmkAction::RightAltGui(_)
            | QmkAction::Meh(_)
            | QmkAction::Hyper(_) => try_multiple_keycodes(qmk, vec![]),
            QmkAction::DefaultLayer(num) => Ok(Action::DefaultLayer(*num as usize)),
            QmkAction::TurnOnLayerWhenPressed(num) => Ok(Action::Layer(*num as usize)),
            QmkAction::LayerWhenHeldOr(num, a) => Ok(Action::HoldTap(
                Box::new(Action::Layer(*num as usize)),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::LeftShiftWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::KeyCode(KeyCode::LShift)),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::LeftControlWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::KeyCode(KeyCode::LCtrl)),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::LeftAltWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::KeyCode(KeyCode::LAlt)),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::LeftGuiWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::KeyCode(KeyCode::LGui)),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::RightShiftWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::KeyCode(KeyCode::RShift)),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::RightControlWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::KeyCode(KeyCode::RCtrl)),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::RightAltWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::KeyCode(KeyCode::RAlt)),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::RightGuiWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::KeyCode(KeyCode::RGui)),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::LeftControlShiftWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::MultipleKeyCodes(vec![
                    KeyCode::LCtrl,
                    KeyCode::LShift,
                ])),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::LeftControlAltWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::MultipleKeyCodes(vec![
                    KeyCode::LCtrl,
                    KeyCode::LAlt,
                ])),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::LeftShiftGuiWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::MultipleKeyCodes(vec![
                    KeyCode::LShift,
                    KeyCode::LGui,
                ])),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::LeftControlAltGuiWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::MultipleKeyCodes(vec![
                    KeyCode::LCtrl,
                    KeyCode::LAlt,
                    KeyCode::LGui,
                ])),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::RightControlAltGuiWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::MultipleKeyCodes(vec![
                    KeyCode::RCtrl,
                    KeyCode::RAlt,
                    KeyCode::RGui,
                ])),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::LeftControlShiftAltWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::MultipleKeyCodes(vec![
                    KeyCode::LCtrl,
                    KeyCode::LShift,
                    KeyCode::LAlt,
                ])),
                Box::new(Action::try_from(&**a)?),
            )),
            QmkAction::LeftControlShiftAltGuiWhenHeldOr(a) => Ok(Action::HoldTap(
                Box::new(Action::MultipleKeyCodes(vec![
                    KeyCode::LCtrl,
                    KeyCode::LShift,
                    KeyCode::LAlt,
                    KeyCode::LGui,
                ])),
                Box::new(Action::try_from(&**a)?),
            )),
            _ => Err(format!("can not convert {:?} to keyberon action", qmk)),
        }
    }
}

/// Compute the Row/Col index in the keyberon matrix based on the index on the
/// Qmk layer
fn get_row_col_idx(
    idx: usize,
    cols: usize,
    rows: usize,
    is_split: bool,
    n_keys: usize,
) -> (usize, usize) {
    let row = idx / cols;
    if row == rows - 1 && is_split {
        let keys_last_row = n_keys - row * cols;
        let idx = idx - row * cols;
        let pad = (cols - keys_last_row) / 2;
        (row, (idx + pad) % cols) // TODO
    } else {
        (row, idx % cols)
    }
}

impl Layers {
    pub fn try_from(
        qmk: QmkKeyMap,
        cols: usize,
        rows: usize,
        is_split: bool,
        ignore_errors: bool,
    ) -> Result<Self, String> {
        let n_layers = qmk.layers.len();
        let mut layers = Layers::with_capacity(cols, rows, n_layers);

        for l in 0..n_layers {
            let n_keys = qmk.layers[l].len();
            for (i, a) in qmk.layers[l].iter().enumerate() {
                let (r, c) = get_row_col_idx(i, cols, rows, is_split, n_keys);
                let res_a = Action::try_from(a);
                let a = if ignore_errors {
                    match res_a {
                        Ok(a) => a,
                        Err(err) => {
                            warn!("Ignoring: {}", err);
                            Action::NoOp
                        }
                    }
                } else {
                    res_a?
                };
                layers.layers[l][r][c] = a;
            }
        }
        layers.is_split = is_split;

        Ok(layers)
    }
}

#[cfg(test)]
mod convert_tests {
    use crate::keyberon::{Action, Layers};
    use crate::qmk::{QmkAction, QmkKeyMap};
    use crate::qmk_keycodes::QmkKeyCode::*;
    use keyberon::key_code::KeyCode::{self, *};

    #[test]
    fn test_convert_keycodes() {
        // A
        let kc_res = KeyCode::try_from(&KcA);
        assert!(kc_res.is_ok());
        assert_eq!(kc_res.unwrap(), A);
        // Calculator
        let kc_res = KeyCode::try_from(&KcCalculator);
        assert!(kc_res.is_ok());
        assert_eq!(kc_res.unwrap(), MediaCalc);
    }

    #[test]
    fn test_convert_action_keycodes() {
        let res = Action::try_from(&QmkAction::KeyCode(KcA));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::KeyCode(A));
        // Calculator
        let res = Action::try_from(&QmkAction::KeyCode(KcCalculator));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::KeyCode(MediaCalc));
        // No
        let res = Action::try_from(&QmkAction::KeyCode(KcNo));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::NoOp);
        // Trans
        let res = Action::try_from(&QmkAction::KeyCode(KcTransparent));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::Trans);
    }

    #[test]
    fn test_multiple_keycodes() {
        // LSFT(KC_A),
        let res = Action::try_from(&QmkAction::LeftShift(Box::new(QmkAction::KeyCode(KcA))));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::MultipleKeyCodes(vec![LShift, A]));
        // ANY(LCTL(LSFT(KC_B)))
        let res = Action::try_from(&QmkAction::Any(Box::new(QmkAction::LeftControl(Box::new(
            QmkAction::LeftShift(Box::new(QmkAction::KeyCode(KcB))),
        )))));
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Action::MultipleKeyCodes(vec![LCtrl, LShift, B])
        );
        // ANY(RCTL(RALT(KC_DELETE)))
        let res = Action::try_from(&QmkAction::Any(Box::new(QmkAction::RightControl(
            Box::new(QmkAction::RightAlt(Box::new(QmkAction::KeyCode(KcDel)))),
        ))));
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Action::MultipleKeyCodes(vec![RCtrl, RAlt, Delete])
        );
    }

    #[test]
    fn test_hold_tap() {
        // LSFT_T(KC_A),
        let res = Action::try_from(&QmkAction::LeftShiftWhenHeldOr(Box::new(
            QmkAction::KeyCode(KcA),
        )));
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Action::HoldTap(
                Box::new(Action::KeyCode(LShift)),
                Box::new(Action::KeyCode(A))
            )
        );
        // C_S_T(KC_B)))
        let res = Action::try_from(&QmkAction::LeftControlShiftWhenHeldOr(Box::new(
            QmkAction::KeyCode(KcB),
        )));
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Action::HoldTap(
                Box::new(Action::MultipleKeyCodes(vec![LCtrl, LShift])),
                Box::new(Action::KeyCode(B))
            )
        );
        // MEH_T(ANY(RCTL(RALT(KC_DELETE))))
        let res = Action::try_from(&QmkAction::LeftControlShiftAltWhenHeldOr(Box::new(
            QmkAction::Any(Box::new(QmkAction::RightControl(Box::new(
                QmkAction::RightAlt(Box::new(QmkAction::KeyCode(KcDel))),
            )))),
        )));
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Action::HoldTap(
                Box::new(Action::MultipleKeyCodes(vec![LCtrl, LShift, LAlt])),
                Box::new(Action::MultipleKeyCodes(vec![RCtrl, RAlt, Delete]))
            )
        );
    }

    #[test]
    fn test_layers() {
        // TO(0),
        let res = Action::try_from(&QmkAction::TurnOnLayerWhenPressed(0));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::Layer(0));
        // LT(1, KC_B)))
        let res = Action::try_from(&QmkAction::LayerWhenHeldOr(
            1,
            Box::new(QmkAction::KeyCode(KcB)),
        ));
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Action::HoldTap(Box::new(Action::Layer(1)), Box::new(Action::KeyCode(B)))
        );
        // DF(2)
        let res = Action::try_from(&QmkAction::DefaultLayer(2));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::DefaultLayer(2));
    }

    #[test]
    fn test_no_split_keycodes_only() {
        let json = r#"{
  "version": 1, "notes": "", "documentation": "", "keyboard": "", "keymap": "",
  "layout": "", "author": "",
  "layers": [ [
      "KC_A", "KC_B", "KC_C", "KC_D",
      "KC_E", "KC_F", "KC_G", "KC_H"
      ], [
      "KC_I", "KC_J", "KC_K", "KC_L",
      "KC_M", "KC_N", "KC_O", "KC_P"
      ], [
      "KC_Q", "KC_R", "KC_S", "KC_T",
      "KC_U", "KC_V", "KC_W", "KC_X"
  ]]}"#;
        let kb = Layers {
            cols: 4,
            rows: 2,
            is_split: false,
            layers: vec![
                vec![
                    vec![
                        Action::KeyCode(A),
                        Action::KeyCode(B),
                        Action::KeyCode(C),
                        Action::KeyCode(D),
                    ],
                    vec![
                        Action::KeyCode(E),
                        Action::KeyCode(F),
                        Action::KeyCode(G),
                        Action::KeyCode(H),
                    ],
                ],
                vec![
                    vec![
                        Action::KeyCode(I),
                        Action::KeyCode(J),
                        Action::KeyCode(K),
                        Action::KeyCode(L),
                    ],
                    vec![
                        Action::KeyCode(M),
                        Action::KeyCode(N),
                        Action::KeyCode(O),
                        Action::KeyCode(P),
                    ],
                ],
                vec![
                    vec![
                        Action::KeyCode(Q),
                        Action::KeyCode(R),
                        Action::KeyCode(S),
                        Action::KeyCode(T),
                    ],
                    vec![
                        Action::KeyCode(U),
                        Action::KeyCode(V),
                        Action::KeyCode(W),
                        Action::KeyCode(X),
                    ],
                ],
            ],
        };
        let qmk = QmkKeyMap::from_json_str(&json).unwrap();
        let res = Layers::try_from(qmk, 4, 2, false, false);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), kb);
    }

    #[test]
    fn test_split_8_3() {
        let json = r#"{
  "version": 1, "notes": "", "documentation": "", "keyboard": "", "keymap": "",
  "layout": "", "author": "",
  "layers": [ [
      "KC_A", "KC_B", "KC_C", "KC_D",    "KC_E", "KC_F", "KC_G", "KC_H",
      "KC_I", "KC_J", "KC_K", "KC_L",    "KC_M", "KC_N", "KC_O", "KC_P",
                      "KC_Q", "KC_R",    "KC_S", "KC_T"
  ]]}"#;
        let kb = Layers {
            cols: 8,
            rows: 3,
            is_split: true,
            layers: vec![vec![
                vec![
                    Action::KeyCode(A),
                    Action::KeyCode(B),
                    Action::KeyCode(C),
                    Action::KeyCode(D),
                    Action::KeyCode(E),
                    Action::KeyCode(F),
                    Action::KeyCode(G),
                    Action::KeyCode(H),
                ],
                vec![
                    Action::KeyCode(I),
                    Action::KeyCode(J),
                    Action::KeyCode(K),
                    Action::KeyCode(L),
                    Action::KeyCode(M),
                    Action::KeyCode(N),
                    Action::KeyCode(O),
                    Action::KeyCode(P),
                ],
                vec![
                    Action::NoOp,
                    Action::NoOp,
                    Action::KeyCode(Q),
                    Action::KeyCode(R),
                    Action::KeyCode(S),
                    Action::KeyCode(T),
                    Action::NoOp,
                    Action::NoOp,
                ],
            ]],
        };
        let qmk = QmkKeyMap::from_json_str(&json).unwrap();
        let res = Layers::try_from(qmk, 8, 3, true, false);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), kb);
    }

    #[test]
    fn test_split_6_3() {
        let json = r#"{
  "version": 1, "notes": "", "documentation": "", "keyboard": "", "keymap": "",
  "layout": "", "author": "",
  "layers": [ [
      "KC_B", "KC_C", "KC_D",    "KC_E", "KC_F", "KC_G",
      "KC_J", "KC_K", "KC_L",    "KC_M", "KC_N", "KC_O",
              "KC_Q", "KC_R",    "KC_S", "KC_T"
  ]]}"#;
        let kb = Layers {
            cols: 6,
            rows: 3,
            is_split: true,
            layers: vec![vec![
                vec![
                    Action::KeyCode(B),
                    Action::KeyCode(C),
                    Action::KeyCode(D),
                    Action::KeyCode(E),
                    Action::KeyCode(F),
                    Action::KeyCode(G),
                ],
                vec![
                    Action::KeyCode(J),
                    Action::KeyCode(K),
                    Action::KeyCode(L),
                    Action::KeyCode(M),
                    Action::KeyCode(N),
                    Action::KeyCode(O),
                ],
                vec![
                    Action::NoOp,
                    Action::KeyCode(Q),
                    Action::KeyCode(R),
                    Action::KeyCode(S),
                    Action::KeyCode(T),
                    Action::NoOp,
                ],
            ]],
        };
        let qmk = QmkKeyMap::from_json_str(&json).unwrap();
        let res = Layers::try_from(qmk, 6, 3, true, false);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), kb);
    }

    #[test]
    fn test_split_6_3_bis() {
        let json = r#"{
  "version": 1, "notes": "", "documentation": "", "keyboard": "", "keymap": "",
  "layout": "", "author": "",
  "layers": [ [
      "KC_B", "KC_C", "KC_D",    "KC_E", "KC_F", "KC_G",
      "KC_J", "KC_K", "KC_L",    "KC_M", "KC_N", "KC_O",
                      "KC_R",    "KC_S"
  ]]}"#;
        let kb = Layers {
            cols: 6,
            rows: 3,
            is_split: true,
            layers: vec![vec![
                vec![
                    Action::KeyCode(B),
                    Action::KeyCode(C),
                    Action::KeyCode(D),
                    Action::KeyCode(E),
                    Action::KeyCode(F),
                    Action::KeyCode(G),
                ],
                vec![
                    Action::KeyCode(J),
                    Action::KeyCode(K),
                    Action::KeyCode(L),
                    Action::KeyCode(M),
                    Action::KeyCode(N),
                    Action::KeyCode(O),
                ],
                vec![
                    Action::NoOp,
                    Action::NoOp,
                    Action::KeyCode(R),
                    Action::KeyCode(S),
                    Action::NoOp,
                    Action::NoOp,
                ],
            ]],
        };
        let qmk = QmkKeyMap::from_json_str(&json).unwrap();
        let res = Layers::try_from(qmk, 6, 3, true, false);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), kb);
    }
}
