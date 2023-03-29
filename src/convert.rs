use crate::keyberon::Action;
use crate::qmk::QmkAction;
use crate::qmk_keycodes::QmkKeyCode;
use keyberon::key_code::KeyCode;
use std::convert::TryFrom;

impl TryFrom<QmkKeyCode> for KeyCode {
    type Error = String;

    fn try_from(qmk: QmkKeyCode) -> Result<Self, Self::Error> {
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
fn try_multiple_keycodes(action: QmkAction, mut v: Vec<KeyCode>) -> Result<Action, String> {
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
        QmkAction::Any(a) => try_multiple_keycodes(*a, v),
        QmkAction::LeftShift(a) => {
            v.push(KeyCode::LShift);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::LeftControl(a) => {
            v.push(KeyCode::LCtrl);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::LeftAlt(a) => {
            v.push(KeyCode::LAlt);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::LeftGui(a) => {
            v.push(KeyCode::LGui);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::RightShift(a) => {
            v.push(KeyCode::RShift);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::RightControl(a) => {
            v.push(KeyCode::RCtrl);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::RightAlt(a) => {
            v.push(KeyCode::RAlt);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::RightGui(a) => {
            v.push(KeyCode::RGui);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::LeftControlAlt(a) => {
            v.push(KeyCode::LCtrl);
            v.push(KeyCode::LAlt);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::LeftShiftAlt(a) => {
            v.push(KeyCode::LShift);
            v.push(KeyCode::LAlt);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::LeftShiftGui(a) => {
            v.push(KeyCode::LShift);
            v.push(KeyCode::LGui);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::LeftAltGui(a) => {
            v.push(KeyCode::LAlt);
            v.push(KeyCode::LGui);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::RightControlAlt(a) => {
            v.push(KeyCode::RCtrl);
            v.push(KeyCode::RAlt);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::RightShiftAlt(a) => {
            v.push(KeyCode::RShift);
            v.push(KeyCode::RAlt);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::RightShiftGui(a) => {
            v.push(KeyCode::RShift);
            v.push(KeyCode::RGui);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::RightAltGui(a) => {
            v.push(KeyCode::RAlt);
            v.push(KeyCode::RGui);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::Meh(a) => {
            v.push(KeyCode::LCtrl);
            v.push(KeyCode::LShift);
            v.push(KeyCode::LAlt);
            try_multiple_keycodes(*a, v)
        }
        QmkAction::Hyper(a) => {
            v.push(KeyCode::LCtrl);
            v.push(KeyCode::LShift);
            v.push(KeyCode::LAlt);
            v.push(KeyCode::LGui);
            try_multiple_keycodes(*a, v)
        }
        _ => Err(format!("can not convert {:?} to keyberon action", action)),
    }
}

impl TryFrom<QmkAction> for Action {
    type Error = String;

    fn try_from(qmk: QmkAction) -> Result<Self, Self::Error> {
        match qmk {
            QmkAction::KeyCode(QmkKeyCode::KcNo) => Ok(Action::NoOp),
            QmkAction::KeyCode(QmkKeyCode::KcTransparent) => Ok(Action::Trans),
            QmkAction::KeyCode(kc) => Ok(Action::KeyCode(KeyCode::try_from(kc)?)),
            QmkAction::Any(a) => try_multiple_keycodes(*a, vec![]),
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
            _ => Err(format!("can not convert {:?} to keyberon action", qmk)),
        }
    }
}

#[cfg(test)]
mod convert_tests {
    use crate::keyberon::Action;
    use crate::qmk::QmkAction;
    use crate::qmk_keycodes::QmkKeyCode::*;
    use keyberon::key_code::KeyCode::{self, *};

    #[test]
    fn test_convert_keycodes() {
        // A
        let kc_res = KeyCode::try_from(KcA);
        assert!(kc_res.is_ok());
        assert_eq!(kc_res.unwrap(), A);
        // Calculator
        let kc_res = KeyCode::try_from(KcCalculator);
        assert!(kc_res.is_ok());
        assert_eq!(kc_res.unwrap(), MediaCalc);
    }

    #[test]
    fn test_convert_action_keycodes() {
        let res = Action::try_from(QmkAction::KeyCode(KcA));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::KeyCode(A));
        // Calculator
        let res = Action::try_from(QmkAction::KeyCode(KcCalculator));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::KeyCode(MediaCalc));
        // No
        let res = Action::try_from(QmkAction::KeyCode(KcNo));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::NoOp);
        // Trans
        let res = Action::try_from(QmkAction::KeyCode(KcTransparent));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::Trans);
    }

    #[test]
    fn test_multiple_keycodes() {
        // LSFT(KC_A),
        let res = Action::try_from(QmkAction::LeftShift(Box::new(QmkAction::KeyCode(KcA))));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), Action::MultipleKeyCodes(vec![LShift, A]));
        // ANY(LCTL(LSFT(KC_B)))
        let res = Action::try_from(QmkAction::Any(Box::new(QmkAction::LeftControl(Box::new(
            QmkAction::LeftShift(Box::new(QmkAction::KeyCode(KcB))),
        )))));
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Action::MultipleKeyCodes(vec![LCtrl, LShift, B])
        );
        // ANY(RCTL(RALT(KC_DELETE)))
        let res = Action::try_from(QmkAction::Any(Box::new(QmkAction::RightControl(Box::new(
            QmkAction::RightAlt(Box::new(QmkAction::KeyCode(KcDel))),
        )))));
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            Action::MultipleKeyCodes(vec![RCtrl, RAlt, Delete])
        );
    }
}
