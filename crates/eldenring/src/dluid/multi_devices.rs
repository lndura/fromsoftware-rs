use std::ptr::NonNull;

use bitfield::bitfield;

use crate::{Vector, dluid::DLUserInputDeviceImpl};

/// TODO:
/// Check if VirtMulDevice refrences fields in Mouse, Keyboard and Pad devices.
#[repr(C)]
pub struct MultiDevices {
    vftable: usize,
    pub virtual_multi_device: NonNull<VirtualMultiDevice>,
    pub pad_devices: [NonNull<PadDevice>; 4],
    unk30: [u8; 0x10],
    pub mouse_device: NonNull<MouseDevice>,
    pub keyboard_device: NonNull<KeyboardDevice>,
    unk50: [u8; 0x28],
    pub unk78: MultiDevices_0x78,
    unk3b0: [u8; 16],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct MultiDevices_0x78 {
    vftable: usize,
    allocator: usize,
    pub bitset_fallback: [bool; 162],
    padding: [u8; 6],
    unkb8: [u8; 0x280],
    unk334: u8,
}

// #[repr(C)]
// pub struct MultiDevices_0x78ArrayEntry {
//     pub unk00: u32,
//     pub unk04: u32,
//     pub unk08: bool,
//     padding: [u8; 7],
// }

/// Source of name: RTTI
#[repr(C)]
pub struct VirtualMultiDevice {
    pub user_input_device_impl: DLUserInputDeviceImpl,
    /// Contains a list of pointers to PadDevice, MouseDevice and KeyboardDevice instances.
    device_list: Vector<NonNull<DLUserInputDeviceImpl>>,
}

/// Source of name: RTTI
/// 
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
pub struct DummyDevice {
    user_input_device_impl: DLUserInputDeviceImpl,
}

/// Source of name: RTTI
/// 
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
pub struct PadDevice {
    pub user_input_device_impl: DLUserInputDeviceImpl,
    //unk7d8: [u8; 0x290],
    unk7d8: i32,
    unk7dc: [u8; 4],
    unk7e0: [u8; 0x60],
    /// set by memset in vfptr[43]
    unk840: [u8; 80],
    /// `WORD` bitfield of `XInputGetState()`'s wButtons field.
    pub w_buttons: WButtons,
    // unk892: u16,
    /// Index of the user's controller. Can be a value from 0 to 3.
    pub dw_user_index: i32,
    unk898: [u8; 4],
    pub s_thumb_lx: f32,
    pub s_thumb_ly: f32,
    unk8a4: [u8; 4],
    pub s_thumb_rx: f32,
    pub s_thumb_ry: f32,
    unk8b0: [u8; 12],
    pub b_left_trigger: f32,
    pub b_right_trigger: f32,
    //unk8c4: [u8; 0x1A4]
    // TODO: fill this out...
}

bitfield! {
    /// Source: https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad
    #[repr(C)]
    pub struct WButtons(u16);
    impl Debug;

    pub dpad_up,        set_dpad_up:        0;
    pub dpad_down,      set_dpad_down:      1;
    pub dpad_left,      set_dpad_left:      2;
    pub dpad_right,     set_dpad_right:     3;

    pub start,          set_start:          4;
    pub back,           set_back:           5;

    pub left_thumb,     set_left_thumb:     6;
    pub right_thumb,    set_right_thumb:    7;

    pub left_shoulder,  set_left_shoulder:  8;
    pub right_shoulder, set_right_shoulder: 9;

    pub button_a,       set_a:              12;
    pub button_b,       set_b:              13;
    pub button_x,       set_x:              14;
    pub button_y,       set_y:              15;
}


/// Source of name: RTTI
/// 
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
pub struct MouseDevice {
    pub user_input_device_impl: DLUserInputDeviceImpl,
    unk7d8: i32,
    unk7dc: [u8; 4],
    // Pointer to some Steam Gameoverlay bullshit that makes DirectInput calls to mouse and keyboard devices in DirectX itself.
    unk7e0: usize,
    /// Result of DirectX8 `GetDeviceState`.
    pub mouse_state: DIMouseState2,
    unk7fc: bool,
    unk7fd: u8,
    unk7fe: u8,
    unk7ff: u8,
    /// Horizontal mouse movement.
    pub normalized_lx: f32,
    /// Vertical mouse movement.
    pub normalized_ly: f32,
    /// Scroll mouse movement.
    pub normalized_lz: f32,
}

/// Source of name: https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee416631(v=vs.85)
#[repr(C)]
pub struct DIMouseState2 {
    /// Horizontal mouse movement.
    pub lx: i32,
    /// Vertical mouse movement.
    pub ly: i32,
    /// Scroll mouse movement.
    pub lz: i32,
    /// Mouse buttons 1-8
    pub buttons: [u8; 8],
}

impl DIMouseState2 {
    /// See [DIMouseButton] for reference.
    pub fn pressed<K: Into<usize>>(&self, button: K) -> bool {
        self.buttons[button.into()] & 0x80 != 0
    }
}

/// Source of name: RTTI
/// 
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
pub struct KeyboardDevice {
    pub user_input_device_impl: DLUserInputDeviceImpl,
    unk7d8: i32,
    unk7dc: [u8; 4],
    unk7e0: usize,
    pub di_kb_state: [u8; 256],
    unk8e8: [u8; 8]
}

impl KeyboardDevice {
    /// See [DIMouseButton] for reference.
    pub fn pressed<K: Into<usize>>(&self, key: K) -> bool {
        self.di_kb_state[key.into()] & 0x80 != 0
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DIMouseButton {
    Left    = 0x00,
    Right   = 0x01,
    Middle  = 0x02,
    Button4 = 0x03,
    Button5 = 0x04,
    Button6 = 0x05,
    Button7 = 0x06,
    Button8 = 0x07,
}

impl From<DIMouseButton> for usize {
    fn from(button: DIMouseButton) -> Self {
            button as usize
    }
}

/// Source: https://learn.microsoft.com/en-us/previous-versions/windows/desktop/bb321074(v=vs.85)
#[repr(u8)]
#[allow(nonstandard_style)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DIKey {
    ESCAPE       = 0x01,
    _1           = 0x02,
    _2           = 0x03,
    _3           = 0x04,
    _4           = 0x05,
    _5           = 0x06,
    _6           = 0x07,
    _7           = 0x08,
    _8           = 0x09,
    _9           = 0x0A,
    _0           = 0x0B,
    MINUS        = 0x0C,    // - on main keyboard 
    EQUALS       = 0x0D,
    BACK         = 0x0E,    // backspace 
    TAB          = 0x0F,
    _Q           = 0x10,
    _W           = 0x11,
    _E           = 0x12,
    _R           = 0x13,
    _T           = 0x14,
    _Y           = 0x15,
    _U           = 0x16,
    _I           = 0x17,
    _O           = 0x18,
    _P           = 0x19,
    LBRACKET     = 0x1A,
    RBRACKET     = 0x1B,
    RETURN       = 0x1C,    // Enter on main keyboard 
    LCONTROL     = 0x1D,
    _A           = 0x1E,
    _S           = 0x1F,
    _D           = 0x20,
    _F           = 0x21,
    _G           = 0x22,
    _H           = 0x23,
    _J           = 0x24,
    _K           = 0x25,
    _L           = 0x26,
    SEMICOLON    = 0x27,
    APOSTROPHE   = 0x28,
    GRAVE        = 0x29,    // accent grave 
    LSHIFT       = 0x2A,
    BACKSLASH    = 0x2B,
    _Z           = 0x2C,
    _X           = 0x2D,
    _C           = 0x2E,
    _V           = 0x2F,
    _B           = 0x30,
    _N           = 0x31,
    _M           = 0x32,
    COMMA        = 0x33,
    PERIOD       = 0x34,    // . on main keyboard 
    SLASH        = 0x35,    // / on main keyboard 
    RSHIFT       = 0x36,
    MULTIPLY     = 0x37,    // * on numeric keypad 
    LMENU        = 0x38,    // left Alt 
    SPACE        = 0x39,
    CAPITAL      = 0x3A,
    F1           = 0x3B,
    F2           = 0x3C,
    F3           = 0x3D,
    F4           = 0x3E,
    F5           = 0x3F,
    F6           = 0x40,
    F7           = 0x41,
    F8           = 0x42,
    F9           = 0x43,
    F10          = 0x44,
    NUMLOCK      = 0x45,
    SCROLL       = 0x46,    // Scroll Lock 
    NUMPAD7      = 0x47,
    NUMPAD8      = 0x48,
    NUMPAD9      = 0x49,
    SUBTRACT     = 0x4A,    // - on numeric keypad 
    NUMPAD4      = 0x4B,
    NUMPAD5      = 0x4C,
    NUMPAD6      = 0x4D,
    ADD          = 0x4E,    // + on numeric keypad 
    NUMPAD1      = 0x4F,
    NUMPAD2      = 0x50,
    NUMPAD3      = 0x51,
    NUMPAD0      = 0x52,
    DECIMAL      = 0x53,    // . on numeric keypad 
    OEM_102      = 0x56,    // <> or \| on RT 102-key keyboard (Non-U.S.) 
    F11          = 0x57,
    F12          = 0x58,
    F13          = 0x64,    //                     (NEC PC98) 
    F14          = 0x65,    //                     (NEC PC98) 
    F15          = 0x66,    //                     (NEC PC98) 
    KANA         = 0x70,    // (Japanese keyboard)            
    ABNT_C1      = 0x73,    // /? on Brazilian keyboard 
    CONVERT      = 0x79,    // (Japanese keyboard)            
    NOCONVERT    = 0x7B,    // (Japanese keyboard)            
    YEN          = 0x7D,    // (Japanese keyboard)            
    ABNT_C2      = 0x7E,    // Numpad . on Brazilian keyboard 
    NUMPADEQUALS = 0x8D,    // = on numeric keypad (NEC PC98) 
    PREVTRACK    = 0x90,    // Previous Track (CIRCUMFLEX on Japanese keyboard) 
    AT           = 0x91,    //                     (NEC PC98) 
    COLON        = 0x92,    //                     (NEC PC98) 
    UNDERLINE    = 0x93,    //                     (NEC PC98) 
    KANJI        = 0x94,    // (Japanese keyboard)            
    STOP         = 0x95,    //                     (NEC PC98) 
    AX           = 0x96,    //                     (Japan AX) 
    UNLABELED    = 0x97,    //                        (J3100) 
    NEXTTRACK    = 0x99,    // Next Track 
    NUMPADENTER  = 0x9C,    // Enter on numeric keypad 
    RCONTROL     = 0x9D,
    MUTE         = 0xA0,    // Mute 
    CALCULATOR   = 0xA1,    // Calculator 
    PLAYPAUSE    = 0xA2,    // Play / Pause 
    MEDIASTOP    = 0xA4,    // Media Stop 
    VOLUMEDOWN   = 0xAE,    // Volume - 
    VOLUMEUP     = 0xB0,    // Volume + 
    WEBHOME      = 0xB2,    // Web home 
    NUMPADCOMMA  = 0xB3,    // , on numeric keypad (NEC PC98) 
    DIVIDE       = 0xB5,    // / on numeric keypad 
    SYSRQ        = 0xB7,
    RMENU        = 0xB8,    // right Alt 
    PAUSE        = 0xC5,    // Pause 
    HOME         = 0xC7,    // Home on arrow keypad 
    UP           = 0xC8,    // UpArrow on arrow keypad 
    PRIOR        = 0xC9,    // PgUp on arrow keypad 
    LEFT         = 0xCB,    // LeftArrow on arrow keypad 
    RIGHT        = 0xCD,    // RightArrow on arrow keypad 
    END          = 0xCF,    // End on arrow keypad 
    DOWN         = 0xD0,    // DownArrow on arrow keypad 
    NEXT         = 0xD1,    // PgDn on arrow keypad 
    INSERT       = 0xD2,    // Insert on arrow keypad 
    DELETE       = 0xD3,    // Delete on arrow keypad 
    LWIN         = 0xDB,    // Left Windows key 
    RWIN         = 0xDC,    // Right Windows key 
    APPS         = 0xDD,    // AppMenu key 
    POWER        = 0xDE,    // System Power 
    SLEEP        = 0xDF,    // System Sleep 
    WAKE         = 0xE3,    // System Wake 
    WEBSEARCH    = 0xE5,    // Web Search 
    WEBFAVORITES = 0xE6,    // Web Favorites 
    WEBREFRESH   = 0xE7,    // Web Refresh 
    WEBSTOP      = 0xE8,    // Web Stop 
    WEBFORWARD   = 0xE9,    // Web Forward 
    WEBBACK      = 0xEA,    // Web Back 
    MYCOMPUTER   = 0xEB,    // My Computer 
    MAIL         = 0xEC,    // Mail 
    MEDIASELECT  = 0xED,    // Media Select 
}

impl From<DIKey> for usize {
    fn from(key: DIKey) -> Self {
            key as usize
    }
}