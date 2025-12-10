use std::{iter, mem::transmute, time::{Duration, Instant}};

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, ChrIns, HKBCharacter, WorldChrMan},
    fd4::FD4TaskData,
    util::system::wait_for_system_init,
};
use shared::{FromStatic, Program, SharedTaskImpExt};

use pelite::pe64::Pe;

const PLAY_ANIMATION_BY_NAME_RVA: u32 = 0x0;
const JUMP_TIME_FRAME: Duration = Duration::from_millis(250);

struct WallJumpManager {
    last_jump_time: Instant,
    has_jumped: bool,
}

impl WallJumpManager {
    fn new() -> Self {
        WallJumpManager { 
            has_jumped: true,
            last_jump_time: Instant::now()
        }
    }
}

enum JumpType {
    None,
    SlideJump,
    WallClimb,
}


fn play_animation_by_name<S: AsRef<str>>(character: &ChrIns, animation: S) -> bool {
    let Some(va) = Program::current()
        .rva_to_va(PLAY_ANIMATION_BY_NAME_RVA)
        .ok()
    else {
        return false;
    };

    let play_animation_by_name = unsafe { transmute::<u64, extern "C" fn(&HKBCharacter, *const u16) -> u32>(va) };

    let wide_c_string: Vec<u16> = animation
        .as_ref()
        .encode_utf16()
        .chain(iter::once(0))
        .collect();
    
    let hkb_character = character.module_container.behavior.beh_character.hkb_character.as_ref();

    let result = play_animation_by_name(hkb_character, wide_c_string.as_ptr());

    result != u32::MAX
}    

fn init_wall_jump_task() {
    let mut wj_man = WallJumpManager::new();

    let cs_task = (unsafe { CSTaskImp::instance() }).expect("Could not obtain CSTaskImp instance.");
    cs_task.run_recurring(
        move |_: &FD4TaskData| {
            let Some(main_player) = unsafe { WorldChrMan::instance() }
                .ok()
                .and_then(|wrld_chr_man| wrld_chr_man.main_player.as_ref())
            else {
                return;
            };
            let now = Instant::now();
            
            let physics = &main_player.chr_ins.module_container.physics.as_ref();

            if wj_man.has_jumped {
                wj_man.has_jumped = physics.is_jumping;
                return;
            }

            let mut jump_type = JumpType::None;

            // Are we on a sliding cliff/surface?
            let slide_info = &physics.slide_info;
            if !physics.is_touching_ground && slide_info.is_sliding {
                jump_type = JumpType::SlideJump;
            };
            
            // TODO: raycast to check wall verticality.
            let _material_info = &physics.material_info;
            let scaleable_wall = false;

            // We should be allowed to jump if we can either scale the wall or the slope.
            let can_wall_jump = scaleable_wall || scaleable_slope;

            // Are we inside the input time frame?
            let in_time_frame = now - wj_man.last_jump_time <= JUMP_TIME_FRAME;
            if in_time_frame {
                wj_man.has_jumped = true;
                play_animation_by_name(&main_player.chr_ins, "W_Jump");
            }

            wj_man.last_jump_time = now;
            
        },
        CSTaskGroupIndex::ChrIns_PostPhysicsSafe,
    );
}

// Exposed for dll loaders, a.e ModEngine 3.
#[unsafe(no_mangle)]
unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason != 1 {
        return true;
    }

    std::thread::spawn(move || {
        // Wait for the game to initialize. Panic if it doesn't.
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Could not await system init.");
        
        init_wall_jump_task();
    });
    true
}
