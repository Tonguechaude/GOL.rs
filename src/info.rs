use bevy::prelude::*;
use bevy::diagnostic::*;
use std::time::Duration;

#[derive(Resource)]
pub struct FpsConfig(pub Timer, pub bool);

impl FpsConfig {
    pub fn new(timer: Timer, show_detailed: bool) -> Self {
        Self (timer, show_detailed)
    }
    
    pub fn default() -> Self {
        Self (Timer::new(Duration::from_secs(3), TimerMode::Repeating), false)
    }
}

/// Version with timer, printing FPS every x secs
pub fn print_fps_timed(
    diagnostics: Res<DiagnosticsStore>,
    mut timer: ResMut<FpsConfig>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    
    if timer.0.just_finished() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                println!("FPS: {:.2}", value);
            }
        }
    }
}
