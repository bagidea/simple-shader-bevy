use bevy::ecs::system::Resource;

pub mod components;

pub const DEFAULT_WINDOW_WIDTH: f32 = 1280.;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 720.;

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32
}