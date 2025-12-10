use std::{
    //fs::File, os::windows::io::AsRawHandle, 
    fmt, sync::{LazyLock,  Mutex}, thread, time::{Duration, Instant}
};

use tracing_subscriber::EnvFilter;
use windows::{Win32::{
    Foundation::{
        HANDLE, 
        HWND
    }, 
    System::Console::{
        ATTACH_PARENT_PROCESS, AllocConsole, AttachConsole, GetConsoleWindow, GetStdHandle, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE, SetStdHandle
    }
}, core::Error};

use tracing::{
    error, info, level_filters::LevelFilter
};

use shared::{
    F32Matrix4x4, F32Vector4, FromStatic, program::Program, task::*
};

use eldenring::{
    cs::{CSFlipper, CSTaskImp}, 
    fd4::{FD4TaskData, FD4PadManager, ActionInput},
    util::{input::is_key_pressed, system::wait_for_system_init}
};

// Keybinds for toggling the free camera on and off.
// It checks if "ALLOW_TOGGLE_KEY" is held down and then checks if you also pressed "TOGGLE_KEY".
// That way you don't accidentally toggle the free camera as easily.
const ALLOW_TOGGLE_KEY: i32 = 17; // Ctrl key
const TOGGLE_KEY: i32 = 70;       // F key

// Time delay between toggles to prevent rapid switching.
const DEBOUNCE_DELAY: Duration = Duration::from_millis(250);

struct FreeCamera {
    /// Current matrix of the free camera.
    matrix: F32Matrix4x4,
    /// World up vector. (0, 1, 0)
    world_up: F32Vector4,
    /// Last time the free camera was toggled.
    last_toggled: Instant,
    /// Makes camera snap to current information once when enabled.
    updated: bool,
    /// Whether the free camera is enabled.
    enabled: bool,
}

// Trait to bind impl's to F32Vector4 without editing shared/vector.rs.
trait VectorMathUtils{
    fn cross(&self, other: &F32Vector4) -> F32Vector4;
    fn normalize(&self) -> F32Vector4;
}

// Add vector math utility methods to F32Vector4.
impl VectorMathUtils for F32Vector4 {
    fn cross(&self, other: &F32Vector4) -> F32Vector4 {
        F32Vector4(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
            0.0,
        )
    }

    fn normalize(&self) -> F32Vector4 {
        let len = (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt();
        if len == 0.0 || !len.is_finite() {
            return F32Vector4(0.0, 0.0, 0.0, 0.0);
        }
        F32Vector4(self.0 / len, self.1 / len, self.2 / len, 0.0)
    }
}

/// Utility methods for the FreeCamera.
impl FreeCamera {
    /// Directs the camera to look towards a given direction vector.
    fn look_to(&mut self, dir: F32Vector4) {
        self.matrix.2 = dir.normalize();
        self.matrix.0 = dir.cross(&self.world_up).normalize();
        self.matrix.1 = self.matrix.0.cross(&dir).normalize();
    }

    /// Moves the camera along a given axis by a specified step.
    fn move_along(&mut self, axis: F32Vector4, step: f32) {
        self.matrix.3 .0 += axis .0 * step;
        self.matrix.3 .1 += axis .1 * step;
        self.matrix.3 .2 += axis .2 * step;
    }

    /// Updates the free camera's position and orientation based on input.
    fn update(&mut self, step: f32) {
        // If the free camera is not enabled, do nothing.
        if !self.enabled {
            self.updated = false;
            return;
        }

        // Retrieve the CSCamera.
        let Ok(cs_cam) = (unsafe { eldenring::cs::CSCamera::instance()})
        else {
            self.updated = false;
            return;
        };

        // Retrieve the CSPad.
        let Some(cs_pad) = (unsafe{FD4PadManager::instance()}) 
            .ok()
            .and_then(|m| m.get_cs_pad())
        else {
            return;
        };

        let pers_cam = &mut cs_cam.pers_cam_1;
        if !self.updated {
            // Copy the current camera matrix into our free camera matrix.
            self.matrix = pers_cam.matrix;
            self.updated = true;
        } else {
            // Look towards the current camera forward vector.
            self.look_to(pers_cam.matrix.2);
        }

        // Stick L & R / KBM A & D movement.
        let dir_x = cs_pad.poll_movement_x();
        self.move_along(self.matrix.0, step * dir_x);

        // Stick U & D / KBM W & S movement.
        let dir_y = cs_pad.poll_movement_y();
        self.move_along(self.matrix.2, step * dir_y);

        // Jump and Crouch movement.
        let mut up_down: f32 = 0.0;
        let jump_pressed = cs_pad.poll_action(ActionInput::Jump);
        if jump_pressed {
            up_down += 1.0;
        }
        let crouch_pressed = cs_pad.poll_action(ActionInput::Crouch);
        if crouch_pressed {
            up_down -= 1.0;
        }
        self.move_along(self.matrix.1, step * up_down);

        pers_cam.matrix = self.matrix;

    }
}

/// Global instance of our free camera information.
static FREE_CAMERA: LazyLock<Mutex<FreeCamera>> = LazyLock::new(|| {
    Mutex::new(
        FreeCamera {
            matrix: F32Matrix4x4::new(
                F32Vector4(0.0, 0.0, 0.0, 0.0), // right
                F32Vector4(0.0, 0.0, 0.0, 0.0), // up
                F32Vector4(0.0, 0.0, 0.0, 0.0), // forward
                F32Vector4(0.0, 0.0, 0.0, 0.0), // position
            ),
            world_up: F32Vector4(0.0, 1.0, 0.0, 0.0),
            last_toggled: Instant::now(),
            updated: false,
            enabled: true,
        }
    )
});

fn init_console() -> windows::core::Result<()> {
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS)
            .expect("erm...");

        let filter = EnvFilter::from_default_env()
            .add_directive(LevelFilter::TRACE.into());

        let stdout_log = tracing_subscriber::fmt::layer().pretty();
        
        let default_handle = HANDLE::default();
        let handle = GetStdHandle(STD_OUTPUT_HANDLE) 
                .unwrap_or_else(|err| {
                    tracing::info!("{}", err);
                    HANDLE::default()
                });

        if handle == default_handle {
            AllocConsole()
                .unwrap_or_else(|err| {
                    info!("{}", err);
                });
        }



        Ok(())
    }
}

/// Inserts the free camera task into the game's task system.
fn init_free_camera_task() {

    // Try to obtain the CSTaskImp singleton instance.
    // If it isn't available we panic.
    let Ok(cs_task) = (unsafe{CSTaskImp::instance()})
    else {
        return;
    };

    // We register a recurring task that updates every frame.
    cs_task.run_recurring(
        // the "move" keyword is used to capture our "enable_free_camera" inside the closure.
        |_: &FD4TaskData| {

            // Obtain the CSFlipper instance for the task delta time.
            let Ok(cs_flipper) = (unsafe{CSFlipper::instance()})
            else {
                return;
            };

            // Move 10 in-game units per second, regardless of frame rate.
            let step = cs_flipper.task_delta * 10.0;

            // Obtain access to our free camera instance.
            let Ok(mut free_camera) = FREE_CAMERA
                .lock()
            else {
                return;
            };

            // Handle input for toggling the free camera.
            if is_key_pressed(ALLOW_TOGGLE_KEY) {
                if Instant::now() - free_camera.last_toggled >= DEBOUNCE_DELAY {
                    if is_key_pressed(TOGGLE_KEY) {
                        free_camera.enabled = !free_camera.enabled;

                    }
                }
            }

            free_camera.update(step);
            free_camera.last_toggled = Instant::now();
        },
        // After camera step, but before draw phases.
        eldenring::cs::CSTaskGroupIndex::ChrIns_PostPhysics
    );
}

// Exposed for dll loaders, a.e ModEngine 3.
#[unsafe(no_mangle)]
unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason != 1 {
        return true;
    }

    thread::spawn(|| {
        let _ = init_console();

        // Wait for the game to initialize. Panic if it doesn't.
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Could not await system init.");

        // Initialize the free camera task.
        init_free_camera_task();
    });
    true
}
