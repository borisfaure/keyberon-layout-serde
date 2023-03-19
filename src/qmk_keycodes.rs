use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum QmkKeyCode {
    #[serde(rename = "KC_NO")]
    #[serde(alias = "XXXXXXX")]
    KcNo,
    #[serde(rename = "KC_TRANSPARENT")]
    #[serde(alias = "_______")]
    #[serde(alias = "KC_TRNS")]
    KcTransparent,
    #[serde(rename = "KC_A")]
    KcA,
    #[serde(rename = "KC_B")]
    KcB,
    #[serde(rename = "KC_C")]
    KcC,
    #[serde(rename = "KC_D")]
    KcD,
    #[serde(rename = "KC_E")]
    KcE,
    #[serde(rename = "KC_F")]
    KcF,
    #[serde(rename = "KC_G")]
    KcG,
    #[serde(rename = "KC_H")]
    KcH,
    #[serde(rename = "KC_I")]
    KcI,
    #[serde(rename = "KC_J")]
    KcJ,
    #[serde(rename = "KC_K")]
    KcK,
    #[serde(rename = "KC_L")]
    KcL,
    #[serde(rename = "KC_M")]
    KcM,
    #[serde(rename = "KC_N")]
    KcN,
    #[serde(rename = "KC_O")]
    KcO,
    #[serde(rename = "KC_P")]
    KcP,
    #[serde(rename = "KC_Q")]
    KcQ,
    #[serde(rename = "KC_R")]
    KcR,
    #[serde(rename = "KC_S")]
    KcS,
    #[serde(rename = "KC_T")]
    KcT,
    #[serde(rename = "KC_U")]
    KcU,
    #[serde(rename = "KC_V")]
    KcV,
    #[serde(rename = "KC_W")]
    KcW,
    #[serde(rename = "KC_X")]
    KcX,
    #[serde(rename = "KC_Y")]
    KcY,
    #[serde(rename = "KC_Z")]
    KcZ,
    #[serde(rename = "KC_1")]
    Kc1,
    #[serde(rename = "KC_2")]
    Kc2,
    #[serde(rename = "KC_3")]
    Kc3,
    #[serde(rename = "KC_4")]
    Kc4,
    #[serde(rename = "KC_5")]
    Kc5,
    #[serde(rename = "KC_6")]
    Kc6,
    #[serde(rename = "KC_7")]
    Kc7,
    #[serde(rename = "KC_8")]
    Kc8,
    #[serde(rename = "KC_9")]
    Kc9,
    #[serde(rename = "KC_0")]
    Kc0,
    #[serde(rename = "KC_ENTER")]
    #[serde(alias = "KC_ENT")]
    KcEnter,
    #[serde(rename = "KC_ESCAPE")]
    #[serde(alias = "KC_ESC")]
    KcEscape,
    #[serde(rename = "KC_BACKSPACE")]
    #[serde(alias = "KC_BSPC")]
    KcBackSpace,
    #[serde(rename = "KC_TAB")]
    KcTab,
    #[serde(rename = "KC_SPACE")]
    #[serde(alias = "KC_SPC")]
    KcSpace,
    #[serde(rename = "KC_MINUS")]
    #[serde(alias = "KC_MINS")]
    KcMinus,
    #[serde(rename = "KC_EQUAL")]
    #[serde(alias = "KC_EQL")]
    KcEqual,
    #[serde(rename = "KC_LEFT_BRACKET")]
    #[serde(alias = "KC_LBRC")]
    KcLeftBracket,
    #[serde(rename = "KC_RIGHT_BRACKET")]
    #[serde(alias = "KC_RBRC")]
    KcRightBracket,
    #[serde(rename = "KC_BACKSLASH")]
    #[serde(alias = "KC_BSLS")]
    KcBackSlash,
    #[serde(rename = "KC_NONUS_HASH")]
    #[serde(alias = "KC_NUHS")]
    KcNonusHash,
    #[serde(rename = "KC_SEMICOLON")]
    #[serde(alias = "KC_SCLN")]
    KcSemiColon,
    #[serde(rename = "KC_QUOTE")]
    #[serde(alias = "KC_QUOT")]
    KcQuote,
    #[serde(rename = "KC_GRAVE")]
    #[serde(alias = "KC_GRV")]
    KcGrave,
    #[serde(rename = "KC_COMMA")]
    #[serde(alias = "KC_COMM")]
    KcComma,
    #[serde(rename = "KC_DOT")]
    KcDot,
    #[serde(rename = "KC_SLASH")]
    #[serde(alias = "KC_SLSH")]
    KcSlash,
    #[serde(rename = "KC_CAPS_LOCK")]
    #[serde(alias = "KC_CAPS")]
    KcCapsLock,
    #[serde(rename = "KC_F1")]
    KcF1,
    #[serde(rename = "KC_F2")]
    KcF2,
    #[serde(rename = "KC_F3")]
    KcF3,
    #[serde(rename = "KC_F4")]
    KcF4,
    #[serde(rename = "KC_F5")]
    KcF5,
    #[serde(rename = "KC_F6")]
    KcF6,
    #[serde(rename = "KC_F7")]
    KcF7,
    #[serde(rename = "KC_F8")]
    KcF8,
    #[serde(rename = "KC_F9")]
    KcF9,
    #[serde(rename = "KC_F10")]
    KcF10,
    #[serde(rename = "KC_F11")]
    KcF11,
    #[serde(rename = "KC_F12")]
    KcF12,
    #[serde(rename = "KC_PRINT_SCREEN")]
    #[serde(alias = "KC_PSCR")]
    KcPrintScreen,
    #[serde(rename = "KC_SCROLL_LOCK")]
    #[serde(alias = "KC_SCRL")]
    #[serde(alias = "KC_BRMD")]
    KcScrollLock,
    #[serde(rename = "KC_PAUSE")]
    #[serde(alias = "KC_PAUS")]
    #[serde(alias = "KC_BRK")]
    #[serde(alias = "KC_BMU")]
    KcPause,
    #[serde(rename = "KC_INSERT")]
    #[serde(alias = "KC_INS")]
    KcInsert,
    #[serde(rename = "KC_HOME")]
    KcHome,
    #[serde(rename = "KC_PAGE_UP")]
    #[serde(alias = "KC_PGUP")]
    KcPageUp,
    #[serde(rename = "KC_DELETE")]
    #[serde(alias = "KC_DEL")]
    KcDel,
    #[serde(rename = "KC_END")]
    KcEnd,
    #[serde(rename = "KC_PAGE_DOWN")]
    #[serde(alias = "KC_PGDN")]
    KcPageDown,
    #[serde(rename = "KC_RIGHT")]
    #[serde(alias = "KC_RGHT")]
    KcRight,
    #[serde(rename = "KC_LEFT")]
    KcLeft,
    #[serde(rename = "KC_DOWN")]
    KcDown,
    #[serde(rename = "KC_UP")]
    KcUp,
    #[serde(rename = "KC_NUM_LOCK")]
    #[serde(alias = "KC_NUM")]
    KcNumLock,
    #[serde(rename = "KC_KP_SLASH")]
    #[serde(alias = "KC_PSLS")]
    KcKpSlash,
    #[serde(rename = "KC_KP_ASTERISK")]
    #[serde(alias = "KC_PAST")]
    KcKpAsterix,
    #[serde(rename = "KC_KP_MINUS")]
    #[serde(alias = "KC_PMNS")]
    KcKpMinus,
    #[serde(rename = "KC_KP_PLUS")]
    #[serde(alias = "KC_PPLS")]
    KcKpPlus,
    #[serde(rename = "KC_KP_ENTER")]
    #[serde(alias = "KC_PENT")]
    KcKpEnter,
    #[serde(rename = "KC_KP_1")]
    KcKp1,
    #[serde(rename = "KC_KP_2")]
    KcKp2,
    #[serde(rename = "KC_KP_3")]
    KcKp3,
    #[serde(rename = "KC_KP_4")]
    KcKp4,
    #[serde(rename = "KC_KP_5")]
    KcKp5,
    #[serde(rename = "KC_KP_6")]
    KcKp6,
    #[serde(rename = "KC_KP_7")]
    KcKp7,
    #[serde(rename = "KC_KP_8")]
    KcKp8,
    #[serde(rename = "KC_KP_9")]
    KcKp9,
    #[serde(rename = "KC_KP_0")]
    KcKp0,
    #[serde(rename = "KC_KP_DOT")]
    #[serde(alias = "KC_PDOT")]
    KcKpDot,
    #[serde(rename = "KC_NONUS_BACKSLASH")]
    #[serde(alias = "KC_NUBS")]
    KcNonusBackslash,
    #[serde(rename = "KC_APPLICATION")]
    #[serde(alias = "KC_APP")]
    KcApplication,
    #[serde(rename = "KC_KB_POWER")]
    KcKbPower,
    #[serde(rename = "KC_KP_EQUAL")]
    #[serde(alias = "KC_PEQL")]
    KcKpEqual,
    #[serde(rename = "KC_F13")]
    KcF13,
    #[serde(rename = "KC_F14")]
    KcF14,
    #[serde(rename = "KC_F15")]
    KcF15,
    #[serde(rename = "KC_F16")]
    KcF16,
    #[serde(rename = "KC_F17")]
    KcF17,
    #[serde(rename = "KC_F18")]
    KcF18,
    #[serde(rename = "KC_F19")]
    KcF19,
    #[serde(rename = "KC_F20")]
    KcF20,
    #[serde(rename = "KC_F21")]
    KcF21,
    #[serde(rename = "KC_F22")]
    KcF22,
    #[serde(rename = "KC_F23")]
    KcF23,
    #[serde(rename = "KC_F24")]
    KcF24,
    #[serde(rename = "KC_EXECUTE")]
    #[serde(alias = "KC_EXEC")]
    KcExecute,
    #[serde(rename = "KC_HELP")]
    KcHelp,
    #[serde(rename = "KC_MENU")]
    KcMenu,
    #[serde(rename = "KC_SELECT")]
    #[serde(alias = "KC_SLCT")]
    KcSelect,
    #[serde(rename = "KC_STOP")]
    KcStop,
    #[serde(rename = "KC_AGAIN")]
    #[serde(alias = "KC_AGIN")]
    KcAgain,
    #[serde(rename = "KC_UNDO")]
    KcUndo,
    #[serde(rename = "KC_CUT")]
    KcCut,
    #[serde(rename = "KC_COPY")]
    KcCopy,
    #[serde(rename = "KC_PASTE")]
    #[serde(alias = "KC_PSTE")]
    KcPaster,
    #[serde(rename = "KC_FIND")]
    KcFind,
    #[serde(rename = "KC_KB_MUTE")]
    KcKbMute,
    #[serde(rename = "KC_KB_VOLUME_UP")]
    KcKbVolumeUp,
    #[serde(rename = "KC_KB_VOLUME_DOWN")]
    KcKbVolumeDown,
    #[serde(rename = "KC_LOCKING_CAPS_LOCK")]
    #[serde(alias = "KC_LCAP")]
    KcLockingCapsLock,
    #[serde(rename = "KC_LOCKING_NUM_LOCK")]
    #[serde(alias = "KC_LNUM")]
    KcLockingNumLock,
    #[serde(rename = "KC_LOCKING_SCROLL_LOCK")]
    #[serde(alias = "KC_LSCR")]
    KcLockingScrollLock,
    #[serde(rename = "KC_KP_COMMA")]
    #[serde(alias = "KC_PCMM")]
    KcKpComma,
    #[serde(rename = "KC_KP_EQUAL_AS400")]
    KcKpEqualAs400,
    #[serde(rename = "KC_INTERNATIONAL_1")]
    #[serde(alias = "KC_INT1")]
    KcIinternational1,
    #[serde(rename = "KC_INTERNATIONAL_2")]
    #[serde(alias = "KC_INT2")]
    KcIinternational2,
    #[serde(rename = "KC_INTERNATIONAL_3")]
    #[serde(alias = "KC_INT3")]
    KcIinternational3,
    #[serde(rename = "KC_INTERNATIONAL_4")]
    #[serde(alias = "KC_INT4")]
    KcIinternational4,
    #[serde(rename = "KC_INTERNATIONAL_5")]
    #[serde(alias = "KC_INT5")]
    KcIinternational5,
    #[serde(rename = "KC_INTERNATIONAL_6")]
    #[serde(alias = "KC_INT6")]
    KcIinternational6,
    #[serde(rename = "KC_INTERNATIONAL_7")]
    #[serde(alias = "KC_INT7")]
    KcIinternational7,
    #[serde(rename = "KC_INTERNATIONAL_8")]
    #[serde(alias = "KC_INT8")]
    KcIinternational8,
    #[serde(rename = "KC_INTERNATIONAL_9")]
    #[serde(alias = "KC_INT9")]
    KcIinternational9,
    #[serde(rename = "KC_LANGUAGE_1")]
    #[serde(alias = "KC_LNG1")]
    KcLanguage1,
    #[serde(rename = "KC_LANGUAGE_2")]
    #[serde(alias = "KC_LNG2")]
    KcLanguage2,
    #[serde(rename = "KC_LANGUAGE_3")]
    #[serde(alias = "KC_LNG3")]
    KcLanguage3,
    #[serde(rename = "KC_LANGUAGE_4")]
    #[serde(alias = "KC_LNG4")]
    KcLanguage4,
    #[serde(rename = "KC_LANGUAGE_5")]
    #[serde(alias = "KC_LNG5")]
    KcLanguage5,
    #[serde(rename = "KC_LANGUAGE_6")]
    #[serde(alias = "KC_LNG6")]
    KcLanguage6,
    #[serde(rename = "KC_LANGUAGE_7")]
    #[serde(alias = "KC_LNG7")]
    KcLanguage7,
    #[serde(rename = "KC_LANGUAGE_8")]
    #[serde(alias = "KC_LNG8")]
    KcLanguage8,
    #[serde(rename = "KC_LANGUAGE_9")]
    #[serde(alias = "KC_LNG9")]
    KcLanguage9,
    #[serde(rename = "KC_ALTERNATE_ERASE")]
    #[serde(alias = "KC_ERAS")]
    KcAlternateErase,
    #[serde(rename = "KC_SYSTEM_REQUEST")]
    #[serde(alias = "KC_SYRQ")]
    KcSystemRequest,
    #[serde(rename = "KC_CANCEL")]
    #[serde(alias = "KC_CNCL")]
    KcCancel,
    #[serde(rename = "KC_CLEAR")]
    #[serde(alias = "KC_CLR")]
    KcClear,
    #[serde(rename = "KC_PRIOR")]
    #[serde(alias = "KC_PRIR")]
    KcPrior,
    #[serde(rename = "KC_RETURN")]
    #[serde(alias = "KC_RETN")]
    KcReturn,
    #[serde(rename = "KC_SEPARATOR")]
    #[serde(alias = "KC_SEPR")]
    KcSeparator,
    #[serde(rename = "KC_OUT")]
    KcOut,
    #[serde(rename = "KC_OPER")]
    KcOper,
    #[serde(rename = "KC_CLEAR_AGAIN")]
    #[serde(alias = "KC_CLAG")]
    KcClearAgain,
    #[serde(rename = "KC_CRSEL")]
    #[serde(alias = "KC_CRSL")]
    KcCrsel,
    #[serde(rename = "KC_EXSEL")]
    #[serde(alias = "KC_EXSL")]
    KcExsel,
    #[serde(rename = "KC_SYSTEM_POWER")]
    #[serde(alias = "KC_PWR")]
    KcSystemPower,
    #[serde(rename = "KC_SYSTEM_SLEEP")]
    #[serde(alias = "KC_SLEP")]
    KcSystemSleep,
    #[serde(rename = "KC_SYSTEM_WAKE")]
    #[serde(alias = "KC_WAKE")]
    KcSystemWake,
    #[serde(rename = "KC_AUDIO_MUTE")]
    #[serde(alias = "KC_MUTE")]
    KcAudioMute,
    #[serde(rename = "KC_AUDIO_VOL_UP")]
    #[serde(alias = "KC_VOLU")]
    KcAudioVolUp,
    #[serde(rename = "KC_AUDIO_VOL_DOWN")]
    #[serde(alias = "KC_VOLD")]
    KcAudioVolDown,
    #[serde(rename = "KC_MEDIA_NEXT_TRACK")]
    #[serde(alias = "KC_MNXT")]
    KcMediaNextTrack,
    #[serde(rename = "KC_MEDIA_PREV_TRACK")]
    #[serde(alias = "KC_MPRV")]
    KcMediaPrevTrack,
    #[serde(rename = "KC_MEDIA_STOP")]
    #[serde(alias = "KC_MSTP")]
    KcMediaStop,
    #[serde(rename = "KC_MEDIA_PLAY_PAUSE")]
    #[serde(alias = "KC_MPLY")]
    KcMediaPlayPause,
    #[serde(rename = "KC_MEDIA_SELECT")]
    #[serde(alias = "KC_MSEL")]
    KcMediaSelect,
    #[serde(rename = "KC_MEDIA_EJECT")]
    #[serde(alias = "KC_EJCT")]
    KcMediaEject,
    #[serde(rename = "KC_MAIL")]
    KcMail,
    #[serde(rename = "KC_CALCULATOR")]
    #[serde(alias = "KC_CALC")]
    KcCalculator,
    #[serde(rename = "KC_MY_COMPUTER")]
    #[serde(alias = "KC_MYCM")]
    KcMyComputer,
    #[serde(rename = "KC_WWW_SEARCH")]
    #[serde(alias = "KC_WSCH")]
    KcWwwSearch,
    #[serde(rename = "KC_WWW_HOME")]
    #[serde(alias = "KC_WHOM")]
    KcWwwHome,
    #[serde(rename = "KC_WWW_BACK")]
    #[serde(alias = "KC_WBAK")]
    KcWwwBack,
    #[serde(rename = "KC_WWW_FORWARD")]
    #[serde(alias = "KC_WFWD")]
    KcWwwForward,
    #[serde(rename = "KC_WWW_STOP")]
    #[serde(alias = "KC_WSTP")]
    KcWwwStop,
    #[serde(rename = "KC_WWW_REFRESH")]
    #[serde(alias = "KC_WREF")]
    KcWwwRefresh,
    #[serde(rename = "KC_WWW_FAVORITES")]
    #[serde(alias = "KC_WFAV")]
    KcWwwFavorites,
    #[serde(rename = "KC_MEDIA_FAST_FORWARD")]
    #[serde(alias = "KC_MFFD")]
    KcMediaFastForward,
    #[serde(rename = "KC_MEDIA_REWIND")]
    #[serde(alias = "KC_MRWD")]
    KcMediaRewind,
    #[serde(rename = "KC_BRIGHTNESS_UP")]
    #[serde(alias = "KC_BRIU")]
    KcBrightnessUp,
    #[serde(rename = "KC_BRIGHTNESS_DOWN")]
    #[serde(alias = "KC_BRID")]
    KcBrightnessDown,
    #[serde(rename = "KC_CONTROL_PANEL")]
    #[serde(alias = "KC_CPNL")]
    KcControlPanel,
    #[serde(rename = "KC_ASSISTANT")]
    #[serde(alias = "KC_ASST")]
    KcAssistant,
    #[serde(rename = "KC_MISSION_CONTROL")]
    #[serde(alias = "KC_MCTL")]
    KcMissionControl,
    #[serde(rename = "KC_LAUNCHPAD")]
    #[serde(alias = "KC_LPAD")]
    KcLaunchpad,
    #[serde(rename = "KC_MS_UP")]
    #[serde(alias = "KC_MS_U")]
    KcMsUp,
    #[serde(rename = "KC_MS_DOWN")]
    #[serde(alias = "KC_MS_D")]
    KcMsDown,
    #[serde(rename = "KC_MS_LEFT")]
    #[serde(alias = "KC_MS_L")]
    KcMsLeft,
    #[serde(rename = "KC_MS_RIGHT")]
    #[serde(alias = "KC_MS_R")]
    KcMsRight,
    #[serde(rename = "KC_MS_BTN1")]
    #[serde(alias = "KC_BTN1")]
    KcMsBtn1,
    #[serde(rename = "KC_MS_BTN2")]
    #[serde(alias = "KC_BTN2")]
    KcMsBtn2,
    #[serde(rename = "KC_MS_BTN3")]
    #[serde(alias = "KC_BTN3")]
    KcMsBtn3,
    #[serde(rename = "KC_MS_BTN4")]
    #[serde(alias = "KC_BTN4")]
    KcMsBtn4,
    #[serde(rename = "KC_MS_BTN5")]
    #[serde(alias = "KC_BTN5")]
    KcMsBtn5,
    #[serde(rename = "KC_MS_BTN6")]
    #[serde(alias = "KC_BTN6")]
    KcMsBtn6,
    #[serde(rename = "KC_MS_BTN7")]
    #[serde(alias = "KC_BTN7")]
    KcMsBtn7,
    #[serde(rename = "KC_MS_BTN8")]
    #[serde(alias = "KC_BTN8")]
    KcMsBtn8,
    #[serde(rename = "KC_MS_WH_UP")]
    #[serde(alias = "KC_WH_U")]
    KcMsWhUp,
    #[serde(rename = "KC_MS_WH_DOWN")]
    #[serde(alias = "KC_WH_D")]
    KcMsWhDown,
    #[serde(rename = "KC_MS_WH_LEFT")]
    #[serde(alias = "KC_WH_L")]
    KcMsWhLeft,
    #[serde(rename = "KC_MS_WH_RIGHT")]
    #[serde(alias = "KC_WH_R")]
    KcMsWhRight,
    #[serde(rename = "KC_MS_ACCEL0")]
    #[serde(alias = "KC_ACL0")]
    KcMsAccel0,
    #[serde(rename = "KC_MS_ACCEL1")]
    #[serde(alias = "KC_ACL1")]
    KcMsAccel1,
    #[serde(rename = "KC_MS_ACCEL2")]
    #[serde(alias = "KC_ACL2")]
    KcMsAccel2,
    #[serde(rename = "KC_LEFT_CTRL")]
    #[serde(alias = "KC_LCTL")]
    KcLeftCtrl,
    #[serde(rename = "KC_LEFT_SHIFT")]
    #[serde(alias = "KC_LSFT")]
    KcLeftShift,
    #[serde(rename = "KC_LEFT_ALT")]
    #[serde(alias = "KC_LALT")]
    KcLeftAlt,
    #[serde(rename = "KC_LEFT_GUI")]
    #[serde(alias = "KC_LGUI")]
    KcLeftGui,
    #[serde(rename = "KC_RIGHT_CTRL")]
    #[serde(alias = "KC_RCTL")]
    KcRightCtrl,
    #[serde(rename = "KC_RIGHT_SHIFT")]
    #[serde(alias = "KC_RSFT")]
    KcRightShift,
    #[serde(rename = "KC_RIGHT_ALT")]
    #[serde(alias = "KC_RALT")]
    KcRightAlt,
    #[serde(rename = "KC_RIGHT_GUI")]
    #[serde(alias = "KC_RGUI")]
    KcRightGui,
    #[serde(rename = "QK_SWAP_HANDS_TOGGLE")]
    #[serde(alias = "SH_TOGG")]
    QkSwapHandsToggle,
    #[serde(rename = "QK_SWAP_HANDS_TAP_TOGGLE")]
    #[serde(alias = "SH_TT")]
    QkSwapHandsTapToggle,
    #[serde(rename = "QK_SWAP_HANDS_MOMENTARY_ON")]
    #[serde(alias = "SH_MON")]
    QkSwapHandsMomentaryOn,
    #[serde(rename = "QK_SWAP_HANDS_MOMENTARY_OFF")]
    #[serde(alias = "SH_MOFF")]
    QkSwapHandsMomentaryOff,
    #[serde(rename = "QK_SWAP_HANDS_OFF")]
    #[serde(alias = "SH_OFF")]
    QkSwapHandsOff,
    #[serde(rename = "QK_SWAP_HANDS_ON")]
    #[serde(alias = "SH_ON")]
    QkSwapHandsOn,
    #[serde(rename = "QK_SWAP_HANDS_ONE_SHOT")]
    #[serde(alias = "SH_OS")]
    QkSwapHandsOneShot,
    #[serde(rename = "QK_MAGIC_SWAP_CONTROL_CAPS_LOCK")]
    #[serde(alias = "CL_SWAP")]
    QkMagicSwapControlCapsLock,
    #[serde(rename = "QK_MAGIC_UNSWAP_CONTROL_CAPS_LOCK")]
    #[serde(alias = "CL_NORM")]
    QkMagicUnswapControlCapsLock,
    #[serde(rename = "QK_MAGIC_TOGGLE_CONTROL_CAPS_LOCK")]
    #[serde(alias = "CL_TOGG")]
    QkMagicToggleControlCapsLock,
    #[serde(rename = "QK_MAGIC_CAPS_LOCK_AS_CONTROL_OFF")]
    #[serde(alias = "CL_CAPS")]
    QkMagicCapsLockAsControlOff,
    #[serde(rename = "QK_MAGIC_CAPS_LOCK_AS_CONTROL_ON")]
    #[serde(alias = "CL_CTRL")]
    QkMagicCapsLockAsControlOn,
    #[serde(rename = "QK_MAGIC_SWAP_LALT_LGUI")]
    #[serde(alias = "AG_LSWP")]
    QkMagicSwapLAltLGui,
    #[serde(rename = "QK_MAGIC_UNSWAP_LALT_LGUI")]
    #[serde(alias = "AG_LNRM")]
    QkMagicUnswapLAltLGui,
    #[serde(rename = "QK_MAGIC_SWAP_RALT_RGUI")]
    #[serde(alias = "AG_RSWP")]
    QkMagicSwapRAltRGui,
    #[serde(rename = "QK_MAGIC_UNSWAP_RALT_RGUI")]
    #[serde(alias = "AG_RNRM")]
    QkMagicUnswapRAltRgui,
    #[serde(rename = "QK_MAGIC_GUI_ON")]
    #[serde(alias = "GU_ON")]
    QkMagicGuiOn,
    #[serde(rename = "QK_MAGIC_GUI_OFF")]
    #[serde(alias = "GU_OFF")]
    QkMagicGuiOff,
    #[serde(rename = "QK_MAGIC_TOGGLE_GUI")]
    #[serde(alias = "GU_TOGG")]
    QkMagicToggleGui,
    #[serde(rename = "QK_MAGIC_SWAP_GRAVE_ESC")]
    #[serde(alias = "GE_SWAP")]
    QkMagicSwapGraveEsc,
    #[serde(rename = "QK_MAGIC_UNSWAP_GRAVE_ESC")]
    #[serde(alias = "GE_NORM")]
    QkMagicUnswapGraveEsc,
    #[serde(rename = "QK_MAGIC_SWAP_BACKSLASH_BACKSPACE")]
    #[serde(alias = "BS_SWAP")]
    QkMagicSwapBackslashBackspace,
    #[serde(rename = "QK_MAGIC_UNSWAP_BACKSLASH_BACKSPACE")]
    #[serde(alias = "BS_NORM")]
    QkMagicUnswapBackslashBackspace,
    #[serde(rename = "QK_MAGIC_TOGGLE_BACKSLASH_BACKSPACE")]
    #[serde(alias = "BS_TOGG")]
    QkMagicToggleBackslashBackspace,
    #[serde(rename = "QK_MAGIC_NKRO_ON")]
    #[serde(alias = "NK_ON")]
    QkMagicNkroOn,
    #[serde(rename = "QK_MAGIC_NKRO_OFF")]
    #[serde(alias = "NK_OFF")]
    QkMagicNkroOff,
    #[serde(rename = "QK_MAGIC_TOGGLE_NKRO")]
    #[serde(alias = "NK_TOGG")]
    QkMagicToggleNkro,
    #[serde(rename = "QK_MAGIC_SWAP_ALT_GUI")]
    #[serde(alias = "AG_SWAP")]
    QkMagicSwapAltGui,
    #[serde(rename = "QK_MAGIC_UNSWAP_ALT_GUI")]
    #[serde(alias = "AG_NORM")]
    QkMagicUnswapAltGui,
    #[serde(rename = "QK_MAGIC_TOGGLE_ALT_GUI")]
    #[serde(alias = "AG_TOGG")]
    QkMagicToggleAltGui,
    #[serde(rename = "QK_MAGIC_SWAP_LCTL_LGUI")]
    #[serde(alias = "CG_LSWP")]
    QkMagicSwapLCtlLGui,
    #[serde(rename = "QK_MAGIC_UNSWAP_LCTL_LGUI")]
    #[serde(alias = "CG_LNRM")]
    QkMagicUnswapLCtlLGui,
    #[serde(rename = "QK_MAGIC_SWAP_RCTL_RGUI")]
    #[serde(alias = "CG_RSWP")]
    QkMagicSwapRCtlRGui,
    #[serde(rename = "QK_MAGIC_UNSWAP_RCTL_RGUI")]
    #[serde(alias = "CG_RNRM")]
    QkMagicUnswapRCtlRGui,
    #[serde(rename = "QK_MAGIC_SWAP_CTL_GUI")]
    #[serde(alias = "CG_SWAP")]
    QkMagicSwapCtlGui,
    #[serde(rename = "QK_MAGIC_UNSWAP_CTL_GUI")]
    #[serde(alias = "CG_NORM")]
    QkMagicUnswapCtlGui,
    #[serde(rename = "QK_MAGIC_TOGGLE_CTL_GUI")]
    #[serde(alias = "CG_TOGG")]
    QkMagicToggleCtlGui,
    #[serde(rename = "QK_MAGIC_EE_HANDS_LEFT")]
    #[serde(alias = "EH_LEFT")]
    QkMagicEeHandsLeft,
    #[serde(rename = "QK_MAGIC_EE_HANDS_RIGHT")]
    #[serde(alias = "EH_RGHT")]
    QkMagicEeHandsRight,
    #[serde(rename = "QK_MAGIC_SWAP_ESCAPE_CAPS_LOCK")]
    #[serde(alias = "EC_SWAP")]
    QkMagicSwapEscapeCapsLock,
    #[serde(rename = "QK_MAGIC_UNSWAP_ESCAPE_CAPS_LOCK")]
    #[serde(alias = "EC_NORM")]
    QkMagicUnswapEscapeCapsLock,
    #[serde(rename = "QK_MAGIC_TOGGLE_ESCAPE_CAPS_LOCK")]
    #[serde(alias = "EC_TOGG")]
    QkMagicToggleEscapeCapsLock,
    #[serde(rename = "QK_MIDI_ON")]
    #[serde(alias = "MI_ON")]
    QkMidiOn,
    #[serde(rename = "QK_MIDI_OFF")]
    #[serde(alias = "MI_OFF")]
    QkMidiOff,
    #[serde(rename = "QK_MIDI_TOGGLE")]
    #[serde(alias = "MI_TOGG")]
    QkMidiToggle,
    #[serde(rename = "QK_MIDI_NOTE_C_0")]
    #[serde(alias = "MI_C")]
    QkMidiNoteC0,
    #[serde(rename = "QK_MIDI_NOTE_C_SHARP_0")]
    #[serde(alias = "MI_Cs")]
    #[serde(alias = "MI_Db")]
    QkMidiNoteCSharp0,
    #[serde(rename = "QK_MIDI_NOTE_D_0")]
    #[serde(alias = "MI_D")]
    QkMidiNoteD0,
    #[serde(rename = "QK_MIDI_NOTE_D_SHARP_0")]
    #[serde(alias = "MI_Ds")]
    #[serde(alias = "MI_Eb")]
    QkMidiNoteDSharp0,
    #[serde(rename = "QK_MIDI_NOTE_E_0")]
    #[serde(alias = "MI_E")]
    QkMidiNoteE0,
    #[serde(rename = "QK_MIDI_NOTE_F_0")]
    #[serde(alias = "MI_F")]
    QkMidiNoteF0,
    #[serde(rename = "QK_MIDI_NOTE_F_SHARP_0")]
    #[serde(alias = "MI_Fs")]
    #[serde(alias = "MI_Gb")]
    QkMidiNoteFSharp0,
    #[serde(rename = "QK_MIDI_NOTE_G_0")]
    #[serde(alias = "MI_G")]
    QkMidiNoteG0,
    #[serde(rename = "QK_MIDI_NOTE_G_SHARP_0")]
    #[serde(alias = "MI_Gs")]
    #[serde(alias = "MI_Ab")]
    QkMidiNoteGSharp0,
    #[serde(rename = "QK_MIDI_NOTE_A_0")]
    #[serde(alias = "MI_A")]
    QkMidiNoteA0,
    #[serde(rename = "QK_MIDI_NOTE_A_SHARP_0")]
    #[serde(alias = "MI_As")]
    #[serde(alias = "MI_Bb")]
    QkMidiNoteASharp0,
    #[serde(rename = "QK_MIDI_NOTE_B_0")]
    #[serde(alias = "MI_B")]
    QkMidiNoteB0,
    #[serde(rename = "QK_MIDI_NOTE_C_1")]
    #[serde(alias = "MI_C1")]
    QkMidiNoteC1,
    #[serde(rename = "QK_MIDI_NOTE_C_SHARP_1")]
    #[serde(alias = "MI_Cs1")]
    #[serde(alias = "MI_Db1")]
    QkMidiNoteCSharp1,
    #[serde(rename = "QK_MIDI_NOTE_D_1")]
    #[serde(alias = "MI_D1")]
    QkMidiNoteD1,
    #[serde(rename = "QK_MIDI_NOTE_D_SHARP_1")]
    #[serde(alias = "MI_Ds1")]
    #[serde(alias = "MI_Eb1")]
    QkMidiNoteDSharp1,
    #[serde(rename = "QK_MIDI_NOTE_E_1")]
    #[serde(alias = "MI_E1")]
    QkMidiNoteE1,
    #[serde(rename = "QK_MIDI_NOTE_F_1")]
    #[serde(alias = "MI_F1")]
    QkMidiNoteF1,
    #[serde(rename = "QK_MIDI_NOTE_F_SHARP_1")]
    #[serde(alias = "MI_Fs1")]
    #[serde(alias = "MI_Gb1")]
    QkMidiNoteFSharp1,
    #[serde(rename = "QK_MIDI_NOTE_G_1")]
    #[serde(alias = "MI_G1")]
    QkMidiNoteG1,
    #[serde(rename = "QK_MIDI_NOTE_G_SHARP_1")]
    #[serde(alias = "MI_Gs1")]
    #[serde(alias = "MI_Ab1")]
    QkMidiNoteGSharp1,
    #[serde(rename = "QK_MIDI_NOTE_A_1")]
    #[serde(alias = "MI_A1")]
    QkMidiNoteA1,
    #[serde(rename = "QK_MIDI_NOTE_A_SHARP_1")]
    #[serde(alias = "MI_As1")]
    #[serde(alias = "MI_Bb1")]
    QkMidiNoteASharp1,
    #[serde(rename = "QK_MIDI_NOTE_B_1")]
    #[serde(alias = "MI_B1")]
    QkMidiNoteB1,
    #[serde(rename = "QK_MIDI_NOTE_C_2")]
    #[serde(alias = "MI_C2")]
    QkMidiNoteC2,
    #[serde(rename = "QK_MIDI_NOTE_C_SHARP_2")]
    #[serde(alias = "MI_Cs2")]
    #[serde(alias = "MI_Db2")]
    QkMidiNoteCSharp2,
    #[serde(rename = "QK_MIDI_NOTE_D_2")]
    #[serde(alias = "MI_D2")]
    QkMidiNoteD2,
    #[serde(rename = "QK_MIDI_NOTE_D_SHARP_2")]
    #[serde(alias = "MI_Ds2")]
    #[serde(alias = "MI_Eb2")]
    QkMidiNoteDSharp2,
    #[serde(rename = "QK_MIDI_NOTE_E_2")]
    #[serde(alias = "MI_E2")]
    QkMidiNoteE2,
    #[serde(rename = "QK_MIDI_NOTE_F_2")]
    #[serde(alias = "MI_F2")]
    QkMidiNoteF2,
    #[serde(rename = "QK_MIDI_NOTE_F_SHARP_2")]
    #[serde(alias = "MI_Fs2")]
    #[serde(alias = "MI_Gb2")]
    QkMidiNoteFSharp2,
    #[serde(rename = "QK_MIDI_NOTE_G_2")]
    #[serde(alias = "MI_G2")]
    QkMidiNoteG2,
    #[serde(rename = "QK_MIDI_NOTE_G_SHARP_2")]
    #[serde(alias = "MI_Gs2")]
    #[serde(alias = "MI_Eb2")]
    QkMidiNoteGSharp2,
    #[serde(rename = "QK_MIDI_NOTE_A_2")]
    #[serde(alias = "MI_A2")]
    QkMidiNoteA2,
    #[serde(rename = "QK_MIDI_NOTE_A_SHARP_2")]
    #[serde(alias = "MI_As2")]
    #[serde(alias = "MI_Bb2")]
    QkMidiNoteASharp2,
    #[serde(rename = "QK_MIDI_NOTE_B_2")]
    #[serde(alias = "MI_B2")]
    QkMidiNoteB2,
    #[serde(rename = "QK_MIDI_NOTE_C_3")]
    #[serde(alias = "MI_C3")]
    QkMidiNoteC3,
    #[serde(rename = "QK_MIDI_NOTE_C_SHARP_3")]
    #[serde(alias = "MI_Cs3")]
    #[serde(alias = "MI_Db3")]
    QkMidiNoteCSharp3,
    #[serde(rename = "QK_MIDI_NOTE_D_3")]
    #[serde(alias = "MI_D3")]
    QkMidiNoteD3,
    #[serde(rename = "QK_MIDI_NOTE_D_SHARP_3")]
    #[serde(alias = "MI_Ds3")]
    #[serde(alias = "MI_Eb3")]
    QkMidiNoteDSharp3,
    #[serde(rename = "QK_MIDI_NOTE_E_3")]
    #[serde(alias = "MI_E3")]
    QkMidiNoteE3,
    #[serde(rename = "QK_MIDI_NOTE_F_3")]
    #[serde(alias = "MI_F3")]
    QkMidiNoteF3,
    #[serde(rename = "QK_MIDI_NOTE_F_SHARP_3")]
    #[serde(alias = "MI_Fs3")]
    #[serde(alias = "MI_Gb3")]
    QkMidiNoteFSharp3,
    #[serde(rename = "QK_MIDI_NOTE_G_3")]
    #[serde(alias = "MI_G3")]
    QkMidiNoteG3,
    #[serde(rename = "QK_MIDI_NOTE_G_SHARP_3")]
    #[serde(alias = "MI_Gs3")]
    #[serde(alias = "MI_Ab3")]
    QkMidiNoteGSharp3,
    #[serde(rename = "QK_MIDI_NOTE_A_3")]
    #[serde(alias = "MI_A3")]
    QkMidiNoteA3,
    #[serde(rename = "QK_MIDI_NOTE_A_SHARP_3")]
    #[serde(alias = "MI_As3")]
    #[serde(alias = "MI_Bb3")]
    QkMidiNoteASharp3,
    #[serde(rename = "QK_MIDI_NOTE_B_3")]
    #[serde(alias = "MI_B3")]
    QkMidiNoteB3,
    #[serde(rename = "QK_MIDI_NOTE_C_4")]
    #[serde(alias = "MI_C4")]
    QkMidiNoteC4,
    #[serde(rename = "QK_MIDI_NOTE_C_SHARP_4")]
    #[serde(alias = "MI_Cs4")]
    #[serde(alias = "MI_Db4")]
    QkMidiNoteCSharp4,
    #[serde(rename = "QK_MIDI_NOTE_D_4")]
    #[serde(alias = "MI_D4")]
    QkMidiNoteD4,
    #[serde(rename = "QK_MIDI_NOTE_D_SHARP_4")]
    #[serde(alias = "MI_As4")]
    #[serde(alias = "MI_Bb4")]
    QkMidiNoteDSharp4,
    #[serde(rename = "QK_MIDI_NOTE_E_4")]
    #[serde(alias = "MI_E4")]
    QkMidiNoteE4,
    #[serde(rename = "QK_MIDI_NOTE_F_4")]
    #[serde(alias = "MI_F4")]
    QkMidiNoteF4,
    #[serde(rename = "QK_MIDI_NOTE_F_SHARP_4")]
    #[serde(alias = "MI_Fs4")]
    #[serde(alias = "MI_Gb4")]
    QkMidiNoteFSharp4,
    #[serde(rename = "QK_MIDI_NOTE_G_4")]
    #[serde(alias = "MI_G4")]
    QkMidiNoteG4,
    #[serde(rename = "QK_MIDI_NOTE_G_SHARP_4")]
    #[serde(alias = "MI_Gs4")]
    #[serde(alias = "MI_Ab4")]
    QkMidiNoteGSharp4,
    #[serde(rename = "QK_MIDI_NOTE_A_4")]
    #[serde(alias = "MI_A4")]
    QkMidiNoteA4,
    #[serde(rename = "QK_MIDI_NOTE_A_SHARP_4")]
    #[serde(alias = "MI_As4")]
    #[serde(alias = "MI_Bb4")]
    QkMidiNoteASharp4,
    #[serde(rename = "QK_MIDI_NOTE_B_4")]
    #[serde(alias = "MI_B4")]
    QkMidiNoteB4,
    #[serde(rename = "QK_MIDI_NOTE_C_5")]
    #[serde(alias = "MI_C5")]
    QkMidiNoteC5,
    #[serde(rename = "QK_MIDI_NOTE_C_SHARP_5")]
    #[serde(alias = "MI_Cs5")]
    #[serde(alias = "MI_Db5")]
    QkMidiNoteCSharp5,
    #[serde(rename = "QK_MIDI_NOTE_D_5")]
    #[serde(alias = "MI_D5")]
    QkMidiNoteD5,
    #[serde(rename = "QK_MIDI_NOTE_D_SHARP_5")]
    #[serde(alias = "MI_Ds5")]
    #[serde(alias = "MI_Eb5")]
    QkMidiNoteDSharp5,
    #[serde(rename = "QK_MIDI_NOTE_E_5")]
    #[serde(alias = "MI_E5")]
    QkMidiNoteE5,
    #[serde(rename = "QK_MIDI_NOTE_F_5")]
    #[serde(alias = "MI_F5")]
    QkMidiNoteF5,
    #[serde(rename = "QK_MIDI_NOTE_F_SHARP_5")]
    #[serde(alias = "MI_Fs5")]
    #[serde(alias = "MI_Gb5")]
    QkMidiNoteFSharp5,
    #[serde(rename = "QK_MIDI_NOTE_G_5")]
    #[serde(alias = "MI_G5")]
    QkMidiNoteG5,
    #[serde(rename = "QK_MIDI_NOTE_G_SHARP_5")]
    #[serde(alias = "MI_Gs5")]
    #[serde(alias = "MI_Ab5")]
    QkMidiNoteGSharp5,
    #[serde(rename = "QK_MIDI_NOTE_A_5")]
    #[serde(alias = "MI_A5")]
    QkMidiNoteA5,
    #[serde(rename = "QK_MIDI_NOTE_A_SHARP_5")]
    #[serde(alias = "MI_As5")]
    #[serde(alias = "MI_Bb5")]
    QkMidiNoteASharp5,
    #[serde(rename = "QK_MIDI_NOTE_B_5")]
    #[serde(alias = "MI_B5")]
    QkMidiNoteB5,
    #[serde(rename = "QK_MIDI_OCTAVE_N2")]
    #[serde(alias = "MI_OCN2")]
    QkMidiOctaveN2,
    #[serde(rename = "QK_MIDI_OCTAVE_N1")]
    #[serde(alias = "MI_OCN1")]
    QkMidiOctaveN1,
    #[serde(rename = "QK_MIDI_OCTAVE_0")]
    #[serde(alias = "MI_OC0")]
    QkMidiOctave0,
    #[serde(rename = "QK_MIDI_OCTAVE_1")]
    #[serde(alias = "MI_OC1")]
    QkMidiOctave1,
    #[serde(rename = "QK_MIDI_OCTAVE_2")]
    #[serde(alias = "MI_OC2")]
    QkMidiOctave2,
    #[serde(rename = "QK_MIDI_OCTAVE_3")]
    #[serde(alias = "MI_OC3")]
    QkMidiOctave3,
    #[serde(rename = "QK_MIDI_OCTAVE_4")]
    #[serde(alias = "MI_OC4")]
    QkMidiOctave4,
    #[serde(rename = "QK_MIDI_OCTAVE_5")]
    #[serde(alias = "MI_OC5")]
    QkMidiOctave5,
    #[serde(rename = "QK_MIDI_OCTAVE_6")]
    #[serde(alias = "MI_OC6")]
    QkMidiOctave6,
    #[serde(rename = "QK_MIDI_OCTAVE_7")]
    #[serde(alias = "MI_OC7")]
    QkMidiOctave7,
    #[serde(rename = "QK_MIDI_OCTAVE_DOWN")]
    #[serde(alias = "MI_OCTD")]
    QkMidiOctaveDown,
    #[serde(rename = "QK_MIDI_OCTAVE_UP")]
    #[serde(alias = "MI_OCTU")]
    QkMidiOctaveUp,
    #[serde(rename = "QK_MIDI_TRANSPOSE_N6")]
    #[serde(alias = "MI_TRN6")]
    QkMidiTransposeN6,
    #[serde(rename = "QK_MIDI_TRANSPOSE_N5")]
    #[serde(alias = "MI_TRN5")]
    QkMidiTransposeN5,
    #[serde(rename = "QK_MIDI_TRANSPOSE_N4")]
    #[serde(alias = "MI_TRN4")]
    QkMidiTransposeN4,
    #[serde(rename = "QK_MIDI_TRANSPOSE_N3")]
    #[serde(alias = "MI_TRN3")]
    QkMidiTransposeN3,
    #[serde(rename = "QK_MIDI_TRANSPOSE_N2")]
    #[serde(alias = "MI_TRN2")]
    QkMidiTransposeN2,
    #[serde(rename = "QK_MIDI_TRANSPOSE_N1")]
    #[serde(alias = "MI_TRN1")]
    QkMidiTransposeN1,
    #[serde(rename = "QK_MIDI_TRANSPOSE_0")]
    #[serde(alias = "MI_TR0")]
    QkMidiTranspose0,
    #[serde(rename = "QK_MIDI_TRANSPOSE_1")]
    #[serde(alias = "MI_TR1")]
    QkMidiTranspose1,
    #[serde(rename = "QK_MIDI_TRANSPOSE_2")]
    #[serde(alias = "MI_TR2")]
    QkMidiTranspose2,
    #[serde(rename = "QK_MIDI_TRANSPOSE_3")]
    #[serde(alias = "MI_TR3")]
    QkMidiTranspose3,
    #[serde(rename = "QK_MIDI_TRANSPOSE_4")]
    #[serde(alias = "MI_TR4")]
    QkMidiTranspose4,
    #[serde(rename = "QK_MIDI_TRANSPOSE_5")]
    #[serde(alias = "MI_TR5")]
    QkMidiTranspose5,
    #[serde(rename = "QK_MIDI_TRANSPOSE_6")]
    #[serde(alias = "MI_TR6")]
    QkMidiTranspose6,
    #[serde(rename = "QK_MIDI_TRANSPOSE_DOWN")]
    #[serde(alias = "MI_TRSD")]
    QkMidiTransposeDown,
    #[serde(rename = "QK_MIDI_TRANSPOSE_UP")]
    #[serde(alias = "MI_TRSU")]
    QkMidiTransposeUp,
    #[serde(rename = "QK_MIDI_VELOCITY_0")]
    #[serde(alias = "MI_VL0")]
    QkMidiVelocity0,
    #[serde(rename = "QK_MIDI_VELOCITY_1")]
    #[serde(alias = "MI_VL1")]
    QkMidiVelocity1,
    #[serde(rename = "QK_MIDI_VELOCITY_2")]
    #[serde(alias = "MI_VL2")]
    QkMidiVelocity2,
    #[serde(rename = "QK_MIDI_VELOCITY_3")]
    #[serde(alias = "MI_VL3")]
    QkMidiVelocity3,
    #[serde(rename = "QK_MIDI_VELOCITY_4")]
    #[serde(alias = "MI_VL4")]
    QkMidiVelocity4,
    #[serde(rename = "QK_MIDI_VELOCITY_5")]
    #[serde(alias = "MI_VL5")]
    QkMidiVelocity5,
    #[serde(rename = "QK_MIDI_VELOCITY_6")]
    #[serde(alias = "MI_VL6")]
    QkMidiVelocity6,
    #[serde(rename = "QK_MIDI_VELOCITY_7")]
    #[serde(alias = "MI_VL7")]
    QkMidiVelocity7,
    #[serde(rename = "QK_MIDI_VELOCITY_8")]
    #[serde(alias = "MI_VL8")]
    QkMidiVelocity8,
    #[serde(rename = "QK_MIDI_VELOCITY_9")]
    #[serde(alias = "MI_VL9")]
    QkMidiVelocity9,
    #[serde(rename = "QK_MIDI_VELOCITY_10")]
    #[serde(alias = "MI_VL10")]
    QkMidiVelocity10,
    #[serde(rename = "QK_MIDI_VELOCITY_DOWN")]
    #[serde(alias = "MI_VELD")]
    QkMidiVelocityDown,
    #[serde(rename = "QK_MIDI_VELOCITY_UP")]
    #[serde(alias = "MI_VELU")]
    QkMidiVelocityUp,
    #[serde(rename = "QK_MIDI_CHANNEL_1")]
    #[serde(alias = "MI_CH1")]
    QkMidiChannel1,
    #[serde(rename = "QK_MIDI_CHANNEL_2")]
    #[serde(alias = "MI_CH2")]
    QkMidiChannel2,
    #[serde(rename = "QK_MIDI_CHANNEL_3")]
    #[serde(alias = "MI_CH3")]
    QkMidiChannel3,
    #[serde(rename = "QK_MIDI_CHANNEL_4")]
    #[serde(alias = "MI_CH4")]
    QkMidiChannel4,
    #[serde(rename = "QK_MIDI_CHANNEL_5")]
    #[serde(alias = "MI_CH5")]
    QkMidiChannel5,
    #[serde(rename = "QK_MIDI_CHANNEL_6")]
    #[serde(alias = "MI_CH6")]
    QkMidiChannel6,
    #[serde(rename = "QK_MIDI_CHANNEL_7")]
    #[serde(alias = "MI_CH7")]
    QkMidiChannel7,
    #[serde(rename = "QK_MIDI_CHANNEL_8")]
    #[serde(alias = "MI_CH8")]
    QkMidiChannel8,
    #[serde(rename = "QK_MIDI_CHANNEL_9")]
    #[serde(alias = "MI_CH9")]
    QkMidiChannel9,
    #[serde(rename = "QK_MIDI_CHANNEL_10")]
    #[serde(alias = "MI_CH10")]
    QkMidiChannel10,
    #[serde(rename = "QK_MIDI_CHANNEL_11")]
    #[serde(alias = "MI_CH11")]
    QkMidiChannel11,
    #[serde(rename = "QK_MIDI_CHANNEL_12")]
    #[serde(alias = "MI_CH12")]
    QkMidiChannel12,
    #[serde(rename = "QK_MIDI_CHANNEL_13")]
    #[serde(alias = "MI_CH13")]
    QkMidiChannel13,
    #[serde(rename = "QK_MIDI_CHANNEL_14")]
    #[serde(alias = "MI_CH14")]
    QkMidiChannel14,
    #[serde(rename = "QK_MIDI_CHANNEL_15")]
    #[serde(alias = "MI_CH15")]
    QkMidiChannel15,
    #[serde(rename = "QK_MIDI_CHANNEL_16")]
    #[serde(alias = "MI_CH16")]
    QkMidiChannel16,
    #[serde(rename = "QK_MIDI_CHANNEL_DOWN")]
    #[serde(alias = "MI_CHND")]
    QkMidiChannelDown,
    #[serde(rename = "QK_MIDI_CHANNEL_UP")]
    #[serde(alias = "MI_CHNU")]
    QkMidiChannelUp,
    #[serde(rename = "QK_MIDI_ALL_NOTES_OFF")]
    #[serde(alias = "MI_AOFF")]
    QkMidiAllNotesOff,
    #[serde(rename = "QK_MIDI_SUSTAIN")]
    #[serde(alias = "MI_SUST")]
    QkMidiSustain,
    #[serde(rename = "QK_MIDI_PORTAMENTO")]
    #[serde(alias = "MI_PORT")]
    QkMidiPortamento,
    #[serde(rename = "QK_MIDI_SOSTENUTO")]
    #[serde(alias = "MI_SOST")]
    QkMidiSostenuto,
    #[serde(rename = "QK_MIDI_SOFT")]
    #[serde(alias = "MI_SOFT")]
    QkMidiSoft,
    #[serde(rename = "QK_MIDI_LEGATO")]
    #[serde(alias = "MI_LEG")]
    QkMidiLegato,
    #[serde(rename = "QK_MIDI_MODULATION")]
    #[serde(alias = "MI_MOD")]
    QkMidiModulation,
    #[serde(rename = "QK_MIDI_MODULATION_SPEED_DOWN")]
    #[serde(alias = "MI_MODD")]
    QkMidiModulationSpeedDown,
    #[serde(rename = "QK_MIDI_MODULATION_SPEED_UP")]
    #[serde(alias = "MI_MODU")]
    QkMidiModulationSpeedUp,
    #[serde(rename = "QK_MIDI_PITCH_BEND_DOWN")]
    #[serde(alias = "MI_BNDD")]
    QkMidiPitchBendDown,
    #[serde(rename = "QK_MIDI_PITCH_BEND_UP")]
    #[serde(alias = "MI_BNDU")]
    QkMidiPitchBendUp,
    #[serde(rename = "QK_SEQUENCER_ON")]
    #[serde(alias = "SQ_ON")]
    QkSequencerOn,
    #[serde(rename = "QK_SEQUENCER_OFF")]
    #[serde(alias = "SQ_OFF")]
    QkSequencerOff,
    #[serde(rename = "QK_SEQUENCER_TOGGLE")]
    #[serde(alias = "SQ_TOGG")]
    QkSequencerToggle,
    #[serde(rename = "QK_SEQUENCER_TEMPO_DOWN")]
    #[serde(alias = "SQ_TMPD")]
    QkSequencerTempoDown,
    #[serde(rename = "QK_SEQUENCER_TEMPO_UP")]
    #[serde(alias = "SQ_TMPU")]
    QkSequencerTempoUp,
    #[serde(rename = "QK_SEQUENCER_RESOLUTION_DOWN")]
    #[serde(alias = "SQ_RESD")]
    QkSequencerResolutionDown,
    #[serde(rename = "QK_SEQUENCER_RESOLUTION_UP")]
    #[serde(alias = "SQ_RESU")]
    QkSequencerResolutionUp,
    #[serde(rename = "QK_SEQUENCER_STEPS_ALL")]
    #[serde(alias = "SQ_SALL")]
    QkSequencerStepsAll,
    #[serde(rename = "QK_SEQUENCER_STEPS_CLEAR")]
    #[serde(alias = "SQ_SCLR")]
    QkSequencerStepsClear,
    #[serde(rename = "QK_JOYSTICK_BUTTON_0")]
    #[serde(alias = "JS_0")]
    QkJoystickButton0,
    #[serde(rename = "QK_JOYSTICK_BUTTON_1")]
    #[serde(alias = "JS_1")]
    QkJoystickButton1,
    #[serde(rename = "QK_JOYSTICK_BUTTON_2")]
    #[serde(alias = "JS_2")]
    QkJoystickButton2,
    #[serde(rename = "QK_JOYSTICK_BUTTON_3")]
    #[serde(alias = "JS_3")]
    QkJoystickButton3,
    #[serde(rename = "QK_JOYSTICK_BUTTON_4")]
    #[serde(alias = "JS_4")]
    QkJoystickButton4,
    #[serde(rename = "QK_JOYSTICK_BUTTON_5")]
    #[serde(alias = "JS_5")]
    QkJoystickButton5,
    #[serde(rename = "QK_JOYSTICK_BUTTON_6")]
    #[serde(alias = "JS_6")]
    QkJoystickButton6,
    #[serde(rename = "QK_JOYSTICK_BUTTON_7")]
    #[serde(alias = "JS_7")]
    QkJoystickButton7,
    #[serde(rename = "QK_JOYSTICK_BUTTON_8")]
    #[serde(alias = "JS_8")]
    QkJoystickButton8,
    #[serde(rename = "QK_JOYSTICK_BUTTON_9")]
    #[serde(alias = "JS_9")]
    QkJoystickButton9,
    #[serde(rename = "QK_JOYSTICK_BUTTON_10")]
    #[serde(alias = "JS_10")]
    QkJoystickButton10,
    #[serde(rename = "QK_JOYSTICK_BUTTON_11")]
    #[serde(alias = "JS_11")]
    QkJoystickButton11,
    #[serde(rename = "QK_JOYSTICK_BUTTON_12")]
    #[serde(alias = "JS_12")]
    QkJoystickButton12,
    #[serde(rename = "QK_JOYSTICK_BUTTON_13")]
    #[serde(alias = "JS_13")]
    QkJoystickButton13,
    #[serde(rename = "QK_JOYSTICK_BUTTON_14")]
    #[serde(alias = "JS_14")]
    QkJoystickButton14,
    #[serde(rename = "QK_JOYSTICK_BUTTON_15")]
    #[serde(alias = "JS_15")]
    QkJoystickButton15,
    #[serde(rename = "QK_JOYSTICK_BUTTON_16")]
    #[serde(alias = "JS_16")]
    QkJoystickButton16,
    #[serde(rename = "QK_JOYSTICK_BUTTON_17")]
    #[serde(alias = "JS_17")]
    QkJoystickButton17,
    #[serde(rename = "QK_JOYSTICK_BUTTON_18")]
    #[serde(alias = "JS_18")]
    QkJoystickButton18,
    #[serde(rename = "QK_JOYSTICK_BUTTON_19")]
    #[serde(alias = "JS_19")]
    QkJoystickButton19,
    #[serde(rename = "QK_JOYSTICK_BUTTON_20")]
    #[serde(alias = "JS_20")]
    QkJoystickButton20,
    #[serde(rename = "QK_JOYSTICK_BUTTON_21")]
    #[serde(alias = "JS_21")]
    QkJoystickButton21,
    #[serde(rename = "QK_JOYSTICK_BUTTON_22")]
    #[serde(alias = "JS_22")]
    QkJoystickButton22,
    #[serde(rename = "QK_JOYSTICK_BUTTON_23")]
    #[serde(alias = "JS_23")]
    QkJoystickButton23,
    #[serde(rename = "QK_JOYSTICK_BUTTON_24")]
    #[serde(alias = "JS_24")]
    QkJoystickButton24,
    #[serde(rename = "QK_JOYSTICK_BUTTON_25")]
    #[serde(alias = "JS_25")]
    QkJoystickButton25,
    #[serde(rename = "QK_JOYSTICK_BUTTON_26")]
    #[serde(alias = "JS_26")]
    QkJoystickButton26,
    #[serde(rename = "QK_JOYSTICK_BUTTON_27")]
    #[serde(alias = "JS_27")]
    QkJoystickButton27,
    #[serde(rename = "QK_JOYSTICK_BUTTON_28")]
    #[serde(alias = "JS_28")]
    QkJoystickButton28,
    #[serde(rename = "QK_JOYSTICK_BUTTON_29")]
    #[serde(alias = "JS_29")]
    QkJoystickButton29,
    #[serde(rename = "QK_JOYSTICK_BUTTON_30")]
    #[serde(alias = "JS_30")]
    QkJoystickButton30,
    #[serde(rename = "QK_JOYSTICK_BUTTON_31")]
    #[serde(alias = "JS_31")]
    QkJoystickButton31,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_1")]
    #[serde(alias = "PB_1")]
    QkProgrammableButton1,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_2")]
    #[serde(alias = "PB_2")]
    QkProgrammableButton2,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_3")]
    #[serde(alias = "PB_3")]
    QkProgrammableButton3,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_4")]
    #[serde(alias = "PB_4")]
    QkProgrammableButton4,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_5")]
    #[serde(alias = "PB_5")]
    QkProgrammableButton5,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_6")]
    #[serde(alias = "PB_6")]
    QkProgrammableButton6,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_7")]
    #[serde(alias = "PB_7")]
    QkProgrammableButton7,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_8")]
    #[serde(alias = "PB_8")]
    QkProgrammableButton8,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_9")]
    #[serde(alias = "PB_9")]
    QkProgrammableButton9,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_10")]
    #[serde(alias = "PB_10")]
    QkProgrammableButton10,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_11")]
    #[serde(alias = "PB_11")]
    QkProgrammableButton11,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_12")]
    #[serde(alias = "PB_12")]
    QkProgrammableButton12,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_13")]
    #[serde(alias = "PB_13")]
    QkProgrammableButton13,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_14")]
    #[serde(alias = "PB_14")]
    QkProgrammableButton14,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_15")]
    #[serde(alias = "PB_15")]
    QkProgrammableButton15,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_16")]
    #[serde(alias = "PB_16")]
    QkProgrammableButton16,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_17")]
    #[serde(alias = "PB_17")]
    QkProgrammableButton17,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_18")]
    #[serde(alias = "PB_18")]
    QkProgrammableButton18,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_19")]
    #[serde(alias = "PB_19")]
    QkProgrammableButton19,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_20")]
    #[serde(alias = "PB_20")]
    QkProgrammableButton20,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_21")]
    #[serde(alias = "PB_21")]
    QkProgrammableButton21,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_22")]
    #[serde(alias = "PB_22")]
    QkProgrammableButton22,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_23")]
    #[serde(alias = "PB_23")]
    QkProgrammableButton23,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_24")]
    #[serde(alias = "PB_24")]
    QkProgrammableButton24,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_25")]
    #[serde(alias = "PB_25")]
    QkProgrammableButton25,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_26")]
    #[serde(alias = "PB_26")]
    QkProgrammableButton26,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_27")]
    #[serde(alias = "PB_27")]
    QkProgrammableButton27,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_28")]
    #[serde(alias = "PB_28")]
    QkProgrammableButton28,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_29")]
    #[serde(alias = "PB_29")]
    QkProgrammableButton29,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_30")]
    #[serde(alias = "PB_30")]
    QkProgrammableButton30,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_31")]
    #[serde(alias = "PB_31")]
    QkProgrammableButton31,
    #[serde(rename = "QK_PROGRAMMABLE_BUTTON_32")]
    #[serde(alias = "PB_32")]
    QkProgrammableButton32,
    #[serde(rename = "QK_AUDIO_ON")]
    #[serde(alias = "AU_ON")]
    QkAudioOn,
    #[serde(rename = "QK_AUDIO_OFF")]
    #[serde(alias = "AU_OFF")]
    QkAudioOff,
    #[serde(rename = "QK_AUDIO_TOGGLE")]
    #[serde(alias = "AU_TOGG")]
    QkAudioToggle,
    #[serde(rename = "QK_AUDIO_CLICKY_TOGGLE")]
    #[serde(alias = "CK_TOGG")]
    QkAudioClickyToggle,
    #[serde(rename = "QK_AUDIO_CLICKY_ON")]
    #[serde(alias = "CK_ON")]
    QkAudioClickyOn,
    #[serde(rename = "QK_AUDIO_CLICKY_OFF")]
    #[serde(alias = "CK_OFF")]
    QkAudioClickyOff,
    #[serde(rename = "QK_AUDIO_CLICKY_UP")]
    #[serde(alias = "CK_UP")]
    QkAudioClickyUp,
    #[serde(rename = "QK_AUDIO_CLICKY_DOWN")]
    #[serde(alias = "CK_DOWN")]
    QkAudioClickyDown,
    #[serde(rename = "QK_AUDIO_CLICKY_RESET")]
    #[serde(alias = "CK_RST")]
    QkAudioClickyReset,
    #[serde(rename = "QK_MUSIC_ON")]
    #[serde(alias = "MU_ON")]
    QkMusicOn,
    #[serde(rename = "QK_MUSIC_OFF")]
    #[serde(alias = "MU_OFF")]
    QkMusicOff,
    #[serde(rename = "QK_MUSIC_TOGGLE")]
    #[serde(alias = "MU_TOGG")]
    QkMusicToggle,
    #[serde(rename = "QK_MUSIC_MODE_NEXT")]
    #[serde(alias = "MU_NEXT")]
    QkMusicModeNext,
    #[serde(rename = "QK_AUDIO_VOICE_NEXT")]
    #[serde(alias = "AU_NEXT")]
    QkAudioVoiceNext,
    #[serde(rename = "QK_AUDIO_VOICE_PREVIOUS")]
    #[serde(alias = "AU_PREV")]
    QkAudioVoicePrevious,
    #[serde(rename = "QK_STENO_BOLT")]
    QkStenoBolt,
    #[serde(rename = "QK_STENO_GEMINI")]
    QkStenoGemini,
    #[serde(rename = "QK_STENO_COMB")]
    QkStenoComb,
    #[serde(rename = "QK_STENO_COMB_MAX")]
    QkStenoCombMax,
    #[serde(rename = "QK_MACRO_0")]
    #[serde(alias = "MC_0")]
    QkMacro0,
    #[serde(rename = "QK_MACRO_1")]
    #[serde(alias = "MC_1")]
    QkMacro1,
    #[serde(rename = "QK_MACRO_2")]
    #[serde(alias = "MC_2")]
    QkMacro2,
    #[serde(rename = "QK_MACRO_3")]
    #[serde(alias = "MC_3")]
    QkMacro3,
    #[serde(rename = "QK_MACRO_4")]
    #[serde(alias = "MC_4")]
    QkMacro4,
    #[serde(rename = "QK_MACRO_5")]
    #[serde(alias = "MC_5")]
    QkMacro5,
    #[serde(rename = "QK_MACRO_6")]
    #[serde(alias = "MC_6")]
    QkMacro6,
    #[serde(rename = "QK_MACRO_7")]
    #[serde(alias = "MC_7")]
    QkMacro7,
    #[serde(rename = "QK_MACRO_8")]
    #[serde(alias = "MC_8")]
    QkMacro8,
    #[serde(rename = "QK_MACRO_9")]
    #[serde(alias = "MC_9")]
    QkMacro9,
    #[serde(rename = "QK_MACRO_10")]
    #[serde(alias = "MC_10")]
    QkMacro10,
    #[serde(rename = "QK_MACRO_11")]
    #[serde(alias = "MC_11")]
    QkMacro11,
    #[serde(rename = "QK_MACRO_12")]
    #[serde(alias = "MC_12")]
    QkMacro12,
    #[serde(rename = "QK_MACRO_13")]
    #[serde(alias = "MC_13")]
    QkMacro13,
    #[serde(rename = "QK_MACRO_14")]
    #[serde(alias = "MC_14")]
    QkMacro14,
    #[serde(rename = "QK_MACRO_15")]
    #[serde(alias = "MC_15")]
    QkMacro15,
    #[serde(rename = "QK_MACRO_16")]
    #[serde(alias = "MC_16")]
    QkMacro16,
    #[serde(rename = "QK_MACRO_17")]
    #[serde(alias = "MC_17")]
    QkMacro17,
    #[serde(rename = "QK_MACRO_18")]
    #[serde(alias = "MC_18")]
    QkMacro18,
    #[serde(rename = "QK_MACRO_19")]
    #[serde(alias = "MC_19")]
    QkMacro19,
    #[serde(rename = "QK_MACRO_20")]
    #[serde(alias = "MC_20")]
    QkMacro20,
    #[serde(rename = "QK_MACRO_21")]
    #[serde(alias = "MC_21")]
    QkMacro21,
    #[serde(rename = "QK_MACRO_22")]
    #[serde(alias = "MC_22")]
    QkMacro22,
    #[serde(rename = "QK_MACRO_23")]
    #[serde(alias = "MC_23")]
    QkMacro23,
    #[serde(rename = "QK_MACRO_24")]
    #[serde(alias = "MC_24")]
    QkMacro24,
    #[serde(rename = "QK_MACRO_25")]
    #[serde(alias = "MC_25")]
    QkMacro25,
    #[serde(rename = "QK_MACRO_26")]
    #[serde(alias = "MC_26")]
    QkMacro26,
    #[serde(rename = "QK_MACRO_27")]
    #[serde(alias = "MC_27")]
    QkMacro27,
    #[serde(rename = "QK_MACRO_28")]
    #[serde(alias = "MC_28")]
    QkMacro28,
    #[serde(rename = "QK_MACRO_29")]
    #[serde(alias = "MC_29")]
    QkMacro29,
    #[serde(rename = "QK_MACRO_30")]
    #[serde(alias = "MC_30")]
    QkMacro30,
    #[serde(rename = "QK_MACRO_31")]
    #[serde(alias = "MC_31")]
    QkMacro31,
    #[serde(rename = "QK_BACKLIGHT_ON")]
    #[serde(alias = "BL_ON")]
    QkBacklightOn,
    #[serde(rename = "QK_BACKLIGHT_OFF")]
    #[serde(alias = "BL_OFF")]
    QkBacklightOff,
    #[serde(rename = "QK_BACKLIGHT_TOGGLE")]
    #[serde(alias = "BL_TOGG")]
    QkBacklightToggle,
    #[serde(rename = "QK_BACKLIGHT_DOWN")]
    #[serde(alias = "BL_DOWN")]
    QkBacklightDown,
    #[serde(rename = "QK_BACKLIGHT_UP")]
    #[serde(alias = "BL_UP")]
    QkBacklightUp,
    #[serde(rename = "QK_BACKLIGHT_STEP")]
    #[serde(alias = "BL_STEP")]
    QkBacklightStep,
    #[serde(rename = "QK_BACKLIGHT_TOGGLE_BREATHING")]
    #[serde(alias = "BL_BRTG")]
    QkBacklightToggleBreathing,
    #[serde(rename = "RGB_TOG")]
    RgbTog,
    #[serde(rename = "RGB_MODE_FORWARD")]
    #[serde(alias = "RGB_MOD")]
    RgbModeForward,
    #[serde(rename = "RGB_MODE_REVERSE")]
    #[serde(alias = "RGB_RMOD")]
    RgbModeReverse,
    #[serde(rename = "RGB_HUI")]
    RgbHui,
    #[serde(rename = "RGB_HUD")]
    RgbHud,
    #[serde(rename = "RGB_SAI")]
    RgbSai,
    #[serde(rename = "RGB_SAD")]
    RgbSad,
    #[serde(rename = "RGB_VAI")]
    RgbVai,
    #[serde(rename = "RGB_VAD")]
    RgbVad,
    #[serde(rename = "RGB_SPI")]
    RgbSpi,
    #[serde(rename = "RGB_SPD")]
    RgbSpd,
    #[serde(rename = "RGB_MODE_PLAIN")]
    #[serde(alias = "RGB_M_P")]
    RgbModePlain,
    #[serde(rename = "RGB_MODE_BREATHE")]
    #[serde(alias = "RGB_M_B")]
    RgbModeBreathe,
    #[serde(rename = "RGB_MODE_RAINBOW")]
    #[serde(alias = "RGB_M_R")]
    RgbModeRainbow,
    #[serde(rename = "RGB_MODE_SWIRL")]
    #[serde(alias = "RGB_M_SW")]
    RgbModeSwirl,
    #[serde(rename = "RGB_MODE_SNAKE")]
    #[serde(alias = "RGB_M_SN")]
    RgbModeSnake,
    #[serde(rename = "RGB_MODE_KNIGHT")]
    #[serde(alias = "RGB_M_K")]
    RgbModeKnight,
    #[serde(rename = "RGB_MODE_XMAS")]
    #[serde(alias = "RGB_M_X")]
    RgbModeXmas,
    #[serde(rename = "RGB_MODE_GRADIENT")]
    #[serde(alias = "RGB_M_G")]
    RgbModeGradient,
    #[serde(rename = "RGB_MODE_RGBTEST")]
    #[serde(alias = "RGB_M_T")]
    RgbModeRgbtest,
    #[serde(rename = "RGB_MODE_TWINKLE")]
    #[serde(alias = "RGB_M_TW")]
    RgbModeTwinkle,
    #[serde(rename = "QK_BOOTLOADER")]
    #[serde(alias = "QK_BOOT")]
    QkBootloader,
    #[serde(rename = "QK_REBOOT")]
    #[serde(alias = "QK_RBT")]
    QkReboot,
    #[serde(rename = "QK_DEBUG_TOGGLE")]
    #[serde(alias = "DB_TOGG")]
    QkDebugToggle,
    #[serde(rename = "QK_CLEAR_EEPROM")]
    #[serde(alias = "EE_CLR")]
    QkClearEeprom,
    #[serde(rename = "QK_MAKE")]
    QkMake,
    #[serde(rename = "QK_AUTO_SHIFT_DOWN")]
    #[serde(alias = "AS_DOWN")]
    QkAutoShiftDown,
    #[serde(rename = "QK_AUTO_SHIFT_UP")]
    #[serde(alias = "AS_UP")]
    QkAutoShiftUp,
    #[serde(rename = "QK_AUTO_SHIFT_REPORT")]
    #[serde(alias = "AS_RPT")]
    QkAutoShiftReport,
    #[serde(rename = "QK_AUTO_SHIFT_ON")]
    #[serde(alias = "AS_ON")]
    QkAutoShiftOn,
    #[serde(rename = "QK_AUTO_SHIFT_OFF")]
    #[serde(alias = "AS_OFF")]
    QkAutoShiftOff,
    #[serde(rename = "QK_AUTO_SHIFT_TOGGLE")]
    #[serde(alias = "AS_TOGG")]
    QkAutoShiftToggle,
    #[serde(rename = "QK_GRAVE_ESCAPE")]
    #[serde(alias = "QK_GESC")]
    QkGraveEscape,
    #[serde(rename = "QK_VELOCIKEY_TOGGLE")]
    #[serde(alias = "VK_TOGG")]
    QkVelocikeyToggle,
    #[serde(rename = "QK_SPACE_CADET_LEFT_CTRL_PARENTHESIS_OPEN")]
    #[serde(alias = "SC_LCPO")]
    QkSpaceCadetLeftCtrlParenthesisOpen,
    #[serde(rename = "QK_SPACE_CADET_RIGHT_CTRL_PARENTHESIS_CLOSE")]
    #[serde(alias = "SC_RCPC")]
    QkSpaceCadetRightCtrlParenthesisClose,
    #[serde(rename = "QK_SPACE_CADET_LEFT_SHIFT_PARENTHESIS_OPEN")]
    #[serde(alias = "SC_LSPO")]
    QkSpaceCadetLeftShiftParenthesisOpen,
    #[serde(rename = "QK_SPACE_CADET_RIGHT_SHIFT_PARENTHESIS_CLOSE")]
    #[serde(alias = "SC_RSPC")]
    QkSpaceCadetRightShiftParenthesisClose,
    #[serde(rename = "QK_SPACE_CADET_LEFT_ALT_PARENTHESIS_OPEN")]
    #[serde(alias = "SC_LAPO")]
    QkSpaceCadetLeftAltParenthesisOpen,
    #[serde(rename = "QK_SPACE_CADET_RIGHT_ALT_PARENTHESIS_CLOSE")]
    #[serde(alias = "SC_RAPC")]
    QkSpaceCadetRightAltParenthesisClose,
    #[serde(rename = "QK_SPACE_CADET_RIGHT_SHIFT_ENTER")]
    #[serde(alias = "SC_SENT")]
    QkSpaceCadetRightShiftEnter,
    #[serde(rename = "QK_OUTPUT_AUTO")]
    #[serde(alias = "OU_AUTO")]
    QkOutputAuto,
    #[serde(rename = "QK_OUTPUT_USB")]
    #[serde(alias = "OU_USB")]
    QkOutputUsb,
    #[serde(rename = "QK_OUTPUT_BLUETOOTH")]
    #[serde(alias = "OU_BT")]
    QkOutputBluetooth,
    #[serde(rename = "QK_UNICODE_MODE_NEXT")]
    #[serde(alias = "UC_NEXT")]
    QkUnicodeModeNext,
    #[serde(rename = "QK_UNICODE_MODE_PREVIOUS")]
    #[serde(alias = "UC_PREV")]
    QkUnicodeModePrevious,
    #[serde(rename = "QK_UNICODE_MODE_MACOS")]
    #[serde(alias = "UC_MAC")]
    QkUnicodeModeMacos,
    #[serde(rename = "QK_UNICODE_MODE_LINUX")]
    #[serde(alias = "UC_LINX")]
    QkUnicodeModeLinux,
    #[serde(rename = "QK_UNICODE_MODE_WINDOWS")]
    #[serde(alias = "UC_WIN")]
    QkUnicodeModeWindows,
    #[serde(rename = "QK_UNICODE_MODE_BSD")]
    #[serde(alias = "UC_BSD")]
    QkUnicodeModeBsd,
    #[serde(rename = "QK_UNICODE_MODE_WINCOMPOSE")]
    #[serde(alias = "UC_WINC")]
    QkUnicodeModeWincompose,
    #[serde(rename = "QK_UNICODE_MODE_EMACS")]
    #[serde(alias = "UC_EMAC")]
    QkUnicodeModeEmacs,
    #[serde(rename = "QK_HAPTIC_ON")]
    #[serde(alias = "HF_ON")]
    QkHapticOn,
    #[serde(rename = "QK_HAPTIC_OFF")]
    #[serde(alias = "HF_OFF")]
    QkHapticOff,
    #[serde(rename = "QK_HAPTIC_TOGGLE")]
    #[serde(alias = "HF_TOGG")]
    QkHapticToggle,
    #[serde(rename = "QK_HAPTIC_RESET")]
    #[serde(alias = "HF_RST")]
    QkHapticReset,
    #[serde(rename = "QK_HAPTIC_FEEDBACK_TOGGLE")]
    #[serde(alias = "HF_FDBK")]
    QkHapticFeedbackToggle,
    #[serde(rename = "QK_HAPTIC_BUZZ_TOGGLE")]
    #[serde(alias = "HF_BUZZ")]
    QkHapticBuzzToggle,
    #[serde(rename = "QK_HAPTIC_MODE_NEXT")]
    #[serde(alias = "HF_NEXT")]
    QkHapticModeNext,
    #[serde(rename = "QK_HAPTIC_MODE_PREVIOUS")]
    #[serde(alias = "HF_PREV")]
    QkHapticModePrevious,
    #[serde(rename = "QK_HAPTIC_CONTINUOUS_TOGGLE")]
    #[serde(alias = "HF_CONT")]
    QkHapticContinuousToggle,
    #[serde(rename = "QK_HAPTIC_CONTINUOUS_UP")]
    #[serde(alias = "HF_CONU")]
    QkHapticContinuousUp,
    #[serde(rename = "QK_HAPTIC_CONTINUOUS_DOWN")]
    #[serde(alias = "HF_COND")]
    QkHapticContinuousDown,
    #[serde(rename = "QK_HAPTIC_DWELL_UP")]
    #[serde(alias = "HF_DWLU")]
    QkHapticDwellUp,
    #[serde(rename = "QK_HAPTIC_DWELL_DOWN")]
    #[serde(alias = "HF_DWLD")]
    QkHapticDwellDown,
    #[serde(rename = "QK_COMBO_ON")]
    #[serde(alias = "CM_ON")]
    QkComboOn,
    #[serde(rename = "QK_COMBO_OFF")]
    #[serde(alias = "CM_OFF")]
    QkComboOff,
    #[serde(rename = "QK_COMBO_TOGGLE")]
    #[serde(alias = "CM_TOGG")]
    QkComboToggle,
    #[serde(rename = "QK_DYNAMIC_MACRO_RECORD_START_1")]
    #[serde(alias = "DM_REC1")]
    QkDynamicMacroRecordStart1,
    #[serde(rename = "QK_DYNAMIC_MACRO_RECORD_START_2")]
    #[serde(alias = "DM_REC2")]
    QkDynamicMacroRecordStart2,
    #[serde(rename = "QK_DYNAMIC_MACRO_RECORD_STOP")]
    #[serde(alias = "DM_RSTP")]
    QkDynamicMacroRecordStop,
    #[serde(rename = "QK_DYNAMIC_MACRO_PLAY_1")]
    #[serde(alias = "DM_PLY1")]
    QkDynamicMacroPlay1,
    #[serde(rename = "QK_DYNAMIC_MACRO_PLAY_2")]
    #[serde(alias = "DM_PLY2")]
    QkDynamicMacroPlay2,
    #[serde(rename = "QK_LEADER")]
    #[serde(alias = "QK_LEAD")]
    QkLeader,
    #[serde(rename = "QK_LOCK")]
    QkLock,
    #[serde(rename = "QK_ONE_SHOT_ON")]
    #[serde(alias = "OS_ON")]
    QkOneShotOn,
    #[serde(rename = "QK_ONE_SHOT_OFF")]
    #[serde(alias = "OS_OFF")]
    QkOneShotOff,
    #[serde(rename = "QK_ONE_SHOT_TOGGLE")]
    #[serde(alias = "OS_TOGG")]
    QkOneShotToggle,
    #[serde(rename = "QK_KEY_OVERRIDE_TOGGLE")]
    #[serde(alias = "KO_TOGG")]
    QkKeyOverrideToggle,
    #[serde(rename = "QK_KEY_OVERRIDE_ON")]
    #[serde(alias = "KO_ON")]
    QkKeyOverrideOn,
    #[serde(rename = "QK_KEY_OVERRIDE_OFF")]
    #[serde(alias = "KO_OFF")]
    QkKeyOverrideOff,
    #[serde(rename = "QK_SECURE_LOCK")]
    #[serde(alias = "SE_LOCK")]
    QkSecureLock,
    #[serde(rename = "QK_SECURE_UNLOCK")]
    #[serde(alias = "SE_UNLK")]
    QkSecureUnlock,
    #[serde(rename = "QK_SECURE_TOGGLE")]
    #[serde(alias = "SE_TOGG")]
    QkSecureToggle,
    #[serde(rename = "QK_SECURE_REQUEST")]
    #[serde(alias = "SE_REQ")]
    QkSecureRequest,
    #[serde(rename = "QK_DYNAMIC_TAPPING_TERM_PRINT")]
    #[serde(alias = "DT_PRNT")]
    QkDynamicTappingTermPrint,
    #[serde(rename = "QK_DYNAMIC_TAPPING_TERM_UP")]
    #[serde(alias = "DT_UP")]
    QkDynamicTappingTermUp,
    #[serde(rename = "QK_DYNAMIC_TAPPING_TERM_DOWN")]
    #[serde(alias = "DT_DOWN")]
    QkDynamicTappingTermDown,
    #[serde(rename = "QK_CAPS_WORD_TOGGLE")]
    QkCapsWordToggle,
    #[serde(rename = "QK_AUTOCORRECT_ON")]
    #[serde(alias = "AC_ON")]
    QkAutocorrectOn,
    #[serde(rename = "QK_AUTOCORRECT_OFF")]
    #[serde(alias = "AC_OFF")]
    QkAutocorrectOff,
    #[serde(rename = "QK_AUTOCORRECT_TOGGLE")]
    #[serde(alias = "AC_TOGG")]
    QkAutocorrectToggle,
    #[serde(rename = "QK_TRI_LAYER_LOWER")]
    #[serde(alias = "TL_LOWR")]
    QkTriLayerLower,
    #[serde(rename = "QK_TRI_LAYER_UPPER")]
    #[serde(alias = "TL_UPPR")]
    QkTriLayerUpper,
    #[serde(rename = "QK_KB_0")]
    QkKb0,
    #[serde(rename = "QK_KB_1")]
    QkKb1,
    #[serde(rename = "QK_KB_2")]
    QkKb2,
    #[serde(rename = "QK_KB_3")]
    QkKb3,
    #[serde(rename = "QK_KB_4")]
    QkKb4,
    #[serde(rename = "QK_KB_5")]
    QkKb5,
    #[serde(rename = "QK_KB_6")]
    QkKb6,
    #[serde(rename = "QK_KB_7")]
    QkKb7,
    #[serde(rename = "QK_KB_8")]
    QkKb8,
    #[serde(rename = "QK_KB_9")]
    QkKb9,
    #[serde(rename = "QK_KB_10")]
    QkKb10,
    #[serde(rename = "QK_KB_11")]
    QkKb11,
    #[serde(rename = "QK_KB_12")]
    QkKb12,
    #[serde(rename = "QK_KB_13")]
    QkKb13,
    #[serde(rename = "QK_KB_14")]
    QkKb14,
    #[serde(rename = "QK_KB_15")]
    QkKb15,
    #[serde(rename = "QK_KB_16")]
    QkKb16,
    #[serde(rename = "QK_KB_17")]
    QkKb17,
    #[serde(rename = "QK_KB_18")]
    QkKb18,
    #[serde(rename = "QK_KB_19")]
    QkKb19,
    #[serde(rename = "QK_KB_20")]
    QkKb20,
    #[serde(rename = "QK_KB_21")]
    QkKb21,
    #[serde(rename = "QK_KB_22")]
    QkKb22,
    #[serde(rename = "QK_KB_23")]
    QkKb23,
    #[serde(rename = "QK_KB_24")]
    QkKb24,
    #[serde(rename = "QK_KB_25")]
    QkKb25,
    #[serde(rename = "QK_KB_26")]
    QkKb26,
    #[serde(rename = "QK_KB_27")]
    QkKb27,
    #[serde(rename = "QK_KB_28")]
    QkKb28,
    #[serde(rename = "QK_KB_29")]
    QkKb29,
    #[serde(rename = "QK_KB_32")]
    QkKb30,
    #[serde(rename = "QK_KB_31")]
    QkKb31,
    #[serde(rename = "QK_USER_0")]
    QkUser0,
    #[serde(rename = "QK_USER_1")]
    QkUser1,
    #[serde(rename = "QK_USER_2")]
    QkUser2,
    #[serde(rename = "QK_USER_3")]
    QkUser3,
    #[serde(rename = "QK_USER_4")]
    QkUser4,
    #[serde(rename = "QK_USER_5")]
    QkUser5,
    #[serde(rename = "QK_USER_6")]
    QkUser6,
    #[serde(rename = "QK_USER_7")]
    QkUser7,
    #[serde(rename = "QK_USER_8")]
    QkUser8,
    #[serde(rename = "QK_USER_9")]
    QkUser9,
    #[serde(rename = "QK_USER_10")]
    QkUser10,
    #[serde(rename = "QK_USER_11")]
    QkUser11,
    #[serde(rename = "QK_USER_12")]
    QkUser12,
    #[serde(rename = "QK_USER_13")]
    QkUser13,
    #[serde(rename = "QK_USER_14")]
    QkUser14,
    #[serde(rename = "QK_USER_15")]
    QkUser15,
    #[serde(rename = "QK_USER_16")]
    QkUser16,
    #[serde(rename = "QK_USER_17")]
    QkUser17,
    #[serde(rename = "QK_USER_18")]
    QkUser18,
    #[serde(rename = "QK_USER_19")]
    QkUser19,
    #[serde(rename = "QK_USER_20")]
    QkUser20,
    #[serde(rename = "QK_USER_21")]
    QkUser21,
    #[serde(rename = "QK_USER_22")]
    QkUser22,
    #[serde(rename = "QK_USER_23")]
    QkUser23,
    #[serde(rename = "QK_USER_24")]
    QkUser24,
    #[serde(rename = "QK_USER_25")]
    QkUser25,
    #[serde(rename = "QK_USER_26")]
    QkUser26,
    #[serde(rename = "QK_USER_27")]
    QkUser27,
    #[serde(rename = "QK_USER_28")]
    QkUser28,
    #[serde(rename = "QK_USER_29")]
    QkUser29,
    #[serde(rename = "QK_USER_30")]
    QkUser30,
    #[serde(rename = "QK_USER_31")]
    QkUser31,
}
