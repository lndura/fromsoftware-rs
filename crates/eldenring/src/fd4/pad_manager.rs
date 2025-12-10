use shared::program::Program;
use pelite::pe64::Pe;
use std::ptr::NonNull;
use vtable_rs::VPtr;
use crate::Tree;
use std::mem::transmute;

const POLL_INPUT_RVA: u32 = 0x2665050;

#[repr(i32)]
pub enum ActionInput {
    Attack = 7,
    StrongAttack = 8,
    Guard = 9,
    Skill = 10,
    /// Also Dodge Roll and Dash.
    /// Stays true while held.
    BackstepHeld = 12,
    /// Also Dodge Roll and Dash
    /// Briefly true when tapped.
    BackstepTapped = 13,
    Jump = 14,
    UseItem = 15,
    CameraReset = 20,
    Crouch = 21,
    LockOn = 26,
}

// source of name: singleton reflection data
#[repr(C)]
#[shared::singleton("FD4PadManager")]
pub struct FD4PadManager {
    allocator: usize,
    unk8: [u8; 0x40],
    pub game_pad_tree: NonNull<Tree<GamePadNode>>,
}

impl FD4PadManager {
    /// Fetches the CSPad safely.
    pub fn get_cs_pad(&self) -> Option<&CSPad> {
        let rva: u32 = 0x3d5df27;
        let va = Program::current().rva_to_va(rva).ok()?;

        let tree = unsafe{self.game_pad_tree.as_ref()};
        if let Some(node) = tree.iter()
            .take_while(|n| n.unk1 <= va)
            .last()
        {
            node.saturated = true;
            let user_input = unsafe{node.pad.as_ref()};
            Some(user_input)
        } else {
            None
        }
    }
}

/// source of name: pulled it out of my ass
#[repr(C)]
pub struct GamePadNode {
    /// Some
    pub unk1: u64,
    /// Layer where we can grab user inputs.
    /// Keyboard, Mouse and Controller are already abstracted here.
    pub pad: NonNull<CSPad>,
    /// This is toggled by the lookup in the decomp.
    /// We mimic that behavior. In get_cs_pad().
    saturated: bool,
    pad2: [u8; 7],
}

/// source of name: pulled it out of my ass
#[repr(C)]
pub struct UserInputNode {
    input_code: i32,
    unk4: [u8; 28]
}

/// Source of name: ?Servername? reverse engineering channel.
/// Dasaav showcased some reverse engineered info about this struct there.
/// There's 2 rtti references in the Tree. 
/// Which are CSInGamePad_UserInput1 and CSMenuViewerPad.
/// Both of those seem to be the structure below.
#[repr(C)]
pub struct CSPad {
    vftable: VPtr<dyn CSPadVmt, Self>,
    allocator1: usize,
    virtual_multi_device: usize,
    unk18: usize,
    unk20: u32,
    unk24: u32,
    in_game_key_assign: NonNull<CSInGameKeyAssign>,
    input_tree: Tree<UserInputNode>,
    unk40: usize,
    allocator2: usize,
    unk50: [u32; 10],
    unk80: Tree<UserInputNode>,
}

#[vtable_rs::vtable]
pub trait CSPadVmt {
    fn unk0(&self);
    fn unk1(&self);
    fn unk2(&self);
    fn unk3(&self);
    fn poll_stick_input(&self, buffer: &mut (f32, f32), input1: i32, input2: i32);
    fn unk5(&self);
    fn unk6(&self);
}

impl CSPad {
    /// Polls Axis movements.
    /// keyboard and mouse: Mouse_X, Mouse_Y and WASD.
    /// Controller: Joystick movement.
    pub fn poll_stick_input(&self, input1: i32, input2: i32) -> (f32, f32) {
        let mut buffer: (f32, f32) = (0.0, 0.0);
        (self.vftable.poll_stick_input)(self, &mut buffer, input1, input2);
        buffer
    }

    /// Used to check Action Buttons, such as Jump or Crouch.
    /// Returns true if pressed.
    /// Takes the raw input integer instead of the defined actions.
    pub fn poll_action_input(&self, input: i32) -> bool {
        let Some(va) = Program::current()
            .rva_to_va(POLL_INPUT_RVA)
            .ok()
        else {
            return false;
        };

        let call = unsafe {transmute::<u64, fn(&CSPad, i32) -> bool>(va)};

        call(self, input)
    }

    /// Used to poll inputs using the ActionInput enum.
    /// Opted for enum over multiple impl's so impl is cleaner.
    /// Example: poll_action(ActionInput::Jump)
    /// Returns true if space/jump input is pressed.
    pub fn poll_action(&self, action: ActionInput) -> bool {
        self.poll_action_input(action as i32)
    }

    /// Mouse (up/down, left/right) as tuple f32.
    pub fn poll_direction_xy(&self) -> (f32, f32) {
        self.poll_stick_input(4, 5)
    }
    /// Stick up/down and W/S direction.
    pub fn poll_movement_y(&self) -> f32 {
        let movement = self.poll_stick_input(417, 418);
        movement.0 + movement.1
    }
    /// Stick right/left and A/D direction.
    pub fn poll_movement_x(&self) -> f32 {
        let movement = self.poll_stick_input(419, 420);
        movement.0 + movement.1
    }
}

// source of name: rtti
#[repr(C)]
pub struct CSInGameKeyAssign {

}