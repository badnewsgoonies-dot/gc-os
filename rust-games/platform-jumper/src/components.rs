use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    LevelComplete,
    GameOver,
}

#[derive(Resource, Default)]
pub struct GameData {
    pub current_level: usize,
    pub score: u32,
    pub lives: u32,
    pub coins_collected: u32,
    pub coins_total: u32,
}

#[derive(Component)]
pub struct Player {
    pub is_grounded: bool,
    pub jump_timer: f32,
    pub coyote_time: f32,
    pub facing_right: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            is_grounded: false,
            jump_timer: 0.0,
            coyote_time: 0.0,
            facing_right: true,
        }
    }
}

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Platform {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct MovingPlatform {
    pub start: Vec2,
    pub end: Vec2,
    pub speed: f32,
    pub progress: f32,
    pub forward: bool,
}

#[derive(Component)]
pub struct Coin;

#[derive(Component)]
pub struct CoinBob {
    pub base_y: f32,
    pub time: f32,
}

#[derive(Component)]
pub struct Enemy {
    pub patrol_left: f32,
    pub patrol_right: f32,
    pub speed: f32,
    pub moving_right: bool,
}

#[derive(Component)]
pub struct LevelExit;

#[derive(Component)]
pub struct LevelTag;

#[derive(Component)]
pub struct MenuTag;

#[derive(Component)]
pub struct HudScore;

#[derive(Component)]
pub struct HudLives;

#[derive(Component)]
pub struct HudCoins;

pub const GRAVITY: f32 = -1200.0;
pub const PLAYER_SPEED: f32 = 300.0;
pub const JUMP_FORCE: f32 = 520.0;
pub const PLAYER_SIZE: Vec2 = Vec2::new(28.0, 36.0);
pub const COYOTE_TIME: f32 = 0.1;
pub const MAX_FALL_SPEED: f32 = -600.0;
pub const DEATH_ZONE_Y: f32 = -400.0;
