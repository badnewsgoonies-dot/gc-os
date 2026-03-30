use bevy::prelude::*;

mod components;
mod systems;
mod levels;

use components::*;
use systems::*;
use levels::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Platform Jumper".into(),
                resolution: (960.0, 640.0).into(),
                canvas: Some("#game-canvas".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .init_resource::<GameData>()
        .add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup::<MenuTag>)
        .add_systems(OnEnter(GameState::Playing), setup_level)
        .add_systems(
            Update,
            (
                player_input,
                apply_gravity,
                apply_velocity,
                check_platform_collisions,
                moving_platform_system,
                coin_collection,
                enemy_ai,
                enemy_collision,
                camera_follow,
                check_death_zone,
                check_level_complete,
                update_hud,
                pause_input,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup::<LevelTag>)
        .add_systems(OnEnter(GameState::LevelComplete), spawn_level_complete_screen)
        .add_systems(
            Update,
            level_complete_input.run_if(in_state(GameState::LevelComplete)),
        )
        .add_systems(OnExit(GameState::LevelComplete), cleanup::<MenuTag>)
        .add_systems(OnEnter(GameState::GameOver), spawn_game_over_screen)
        .add_systems(
            Update,
            game_over_input.run_if(in_state(GameState::GameOver)),
        )
        .add_systems(OnExit(GameState::GameOver), cleanup::<MenuTag>)
        .insert_resource(ClearColor(Color::srgb(0.15, 0.15, 0.25)))
        .run();
}
