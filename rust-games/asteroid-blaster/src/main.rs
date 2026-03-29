use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy::math::vec2;
use rand::Rng;
use std::f32::consts::PI;

const SHIP_SIZE: f32 = 20.0;
const BULLET_SPEED: f32 = 500.0;
const BULLET_LIFETIME: f32 = 1.5;
const ASTEROID_SPEED_MIN: f32 = 50.0;
const ASTEROID_SPEED_MAX: f32 = 150.0;
const ROTATION_SPEED: f32 = 5.0;
const THRUST: f32 = 300.0;
const DRAG: f32 = 0.98;
const MAX_SPEED: f32 = 400.0;
const SPAWN_INTERVAL: f32 = 2.0;
const PARTICLE_COUNT: usize = 8;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState { #[default] Menu, Playing, GameOver }

#[derive(Component)]
struct Ship { rotation: f32 }

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Bullet { lifetime: f32 }

#[derive(Component)]
struct Asteroid { size: AsteroidSize }

#[derive(Clone, Copy)]
enum AsteroidSize { Large, Medium, Small }

impl AsteroidSize {
    fn radius(&self) -> f32 {
        match self { Self::Large => 40.0, Self::Medium => 20.0, Self::Small => 10.0 }
    }
    fn points(&self) -> u32 {
        match self { Self::Large => 20, Self::Medium => 50, Self::Small => 100 }
    }
    fn split(&self) -> Option<Self> {
        match self { Self::Large => Some(Self::Medium), Self::Medium => Some(Self::Small), Self::Small => None }
    }
}

#[derive(Component)]
struct Particle { lifetime: f32, max_lifetime: f32 }

#[derive(Component)]
struct MenuText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct GameOverText;

#[derive(Resource)]
struct GameScore { score: u32, lives: u32, high_score: u32 }

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Resource)]
struct Bounds { w: f32, h: f32 }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroid Blaster".into(),
                resolution: WindowResolution::new(800.0, 600.0),
                canvas: Some("#game-canvas".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.08)))
        .insert_resource(GameScore { score: 0, lives: 3, high_score: 0 })
        .insert_resource(SpawnTimer(Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating)))
        .insert_resource(Bounds { w: 800.0, h: 600.0 })
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(OnExit(GameState::Menu), cleanup::<MenuText>)
        .add_systems(OnEnter(GameState::Playing), spawn_ship)
        .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
        .add_systems(OnExit(GameState::GameOver), cleanup::<GameOverText>)
        .add_systems(Update, (
            menu_input,
        ).run_if(in_state(GameState::Menu)))
        .add_systems(Update, (
            ship_input,
            move_entities,
            wrap_positions,
            spawn_asteroids,
            bullet_lifetime,
            check_bullet_asteroid,
            check_ship_asteroid,
            update_particles,
            update_score_ui,
        ).run_if(in_state(GameState::Playing)))
        .add_systems(Update, (
            game_over_input,
        ).run_if(in_state(GameState::GameOver)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        Text2d::new("ASTEROID BLASTER"),
        TextFont { font_size: 48.0, ..default() },
        TextColor(Color::srgb(0.3, 0.8, 1.0)),
        Transform::from_xyz(0.0, 60.0, 10.0),
        MenuText,
    ));
    commands.spawn((
        Text2d::new("Click or Press SPACE to Start"),
        TextFont { font_size: 20.0, ..default() },
        TextColor(Color::srgb(0.6, 0.6, 0.7)),
        Transform::from_xyz(0.0, -30.0, 10.0),
        MenuText,
    ));
}

fn menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut next: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) || touches.any_just_pressed() {
        next.set(GameState::Playing);
    }
}

fn spawn_ship(mut commands: Commands, mut score: ResMut<GameScore>) {
    score.score = 0;
    score.lives = 3;

    // Ship body (triangle made of sprites)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.9, 0.4),
            custom_size: Some(Vec2::new(SHIP_SIZE, SHIP_SIZE * 1.5)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Ship { rotation: 0.0 },
        Velocity(Vec2::ZERO),
    ));

    // Score UI
    commands.spawn((
        Text2d::new("Score: 0"),
        TextFont { font_size: 22.0, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(-300.0, 270.0, 10.0),
        ScoreText,
    ));
}

fn ship_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    time: Res<Time>,
    mut ship_q: Query<(&mut Ship, &mut Velocity, &Transform)>,
    mut commands: Commands,
) {
    let Ok((mut ship, mut vel, transform)) = ship_q.get_single_mut() else { return };

    // Rotation
    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
        ship.rotation += ROTATION_SPEED * time.delta_secs();
    }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        ship.rotation -= ROTATION_SPEED * time.delta_secs();
    }

    // Thrust
    if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
        let dir = Vec2::new(-ship.rotation.sin(), ship.rotation.cos());
        vel.0 += dir * THRUST * time.delta_secs();
        if vel.0.length() > MAX_SPEED {
            vel.0 = vel.0.normalize() * MAX_SPEED;
        }
    }

    vel.0 *= DRAG;

    // Shoot
    if keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        let dir = Vec2::new(-ship.rotation.sin(), ship.rotation.cos());
        let pos = transform.translation.truncate() + dir * SHIP_SIZE;
        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 1.0, 0.3),
                custom_size: Some(Vec2::new(3.0, 8.0)),
                ..default()
            },
            Transform::from_xyz(pos.x, pos.y, 1.0)
                .with_rotation(Quat::from_rotation_z(ship.rotation)),
            Bullet { lifetime: BULLET_LIFETIME },
            Velocity(dir * BULLET_SPEED + vel.0),
        ));
    }
}

fn move_entities(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform, Option<&Ship>)>,
) {
    for (vel, mut transform, ship) in &mut query {
        transform.translation.x += vel.0.x * time.delta_secs();
        transform.translation.y += vel.0.y * time.delta_secs();
        if let Some(s) = ship {
            transform.rotation = Quat::from_rotation_z(s.rotation);
        }
    }
}

fn wrap_positions(bounds: Res<Bounds>, mut query: Query<&mut Transform, With<Velocity>>) {
    let hw = bounds.w / 2.0 + 50.0;
    let hh = bounds.h / 2.0 + 50.0;
    for mut t in &mut query {
        if t.translation.x > hw { t.translation.x = -hw; }
        if t.translation.x < -hw { t.translation.x = hw; }
        if t.translation.y > hh { t.translation.y = -hh; }
        if t.translation.y < -hh { t.translation.y = hh; }
    }
}

fn spawn_asteroids(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    bounds: Res<Bounds>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() { return; }

    let mut rng = rand::thread_rng();
    let hw = bounds.w / 2.0;
    let hh = bounds.h / 2.0;

    let (x, y) = match rng.gen_range(0..4) {
        0 => (-hw - 30.0, rng.gen_range(-hh..hh)),
        1 => (hw + 30.0, rng.gen_range(-hh..hh)),
        2 => (rng.gen_range(-hw..hw), hh + 30.0),
        _ => (rng.gen_range(-hw..hw), -hh - 30.0),
    };

    let angle = rng.gen_range(0.0..PI * 2.0);
    let speed = rng.gen_range(ASTEROID_SPEED_MIN..ASTEROID_SPEED_MAX);
    let size = AsteroidSize::Large;

    spawn_asteroid(&mut commands, Vec2::new(x, y), angle, speed, size);
}

fn spawn_asteroid(commands: &mut Commands, pos: Vec2, angle: f32, speed: f32, size: AsteroidSize) {
    let r = size.radius();
    let color = match size {
        AsteroidSize::Large => Color::srgb(0.6, 0.5, 0.4),
        AsteroidSize::Medium => Color::srgb(0.7, 0.6, 0.5),
        AsteroidSize::Small => Color::srgb(0.8, 0.7, 0.6),
    };
    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(r * 2.0)),
            ..default()
        },
        Transform::from_xyz(pos.x, pos.y, 0.5),
        Asteroid { size },
        Velocity(Vec2::new(angle.cos(), angle.sin()) * speed),
    ));
}

fn bullet_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in &mut query {
        bullet.lifetime -= time.delta_secs();
        if bullet.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn check_bullet_asteroid(
    mut commands: Commands,
    mut score: ResMut<GameScore>,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    asteroids: Query<(Entity, &Transform, &Asteroid)>,
) {
    for (b_entity, b_transform) in &bullets {
        for (a_entity, a_transform, asteroid) in &asteroids {
            let dist = b_transform.translation.truncate()
                .distance(a_transform.translation.truncate());
            if dist < asteroid.size.radius() {
                commands.entity(b_entity).despawn();
                commands.entity(a_entity).despawn();
                score.score += asteroid.size.points();

                // Spawn particles
                let pos = a_transform.translation.truncate();
                spawn_particles(&mut commands, pos, asteroid.size.radius());

                // Split asteroid
                if let Some(smaller) = asteroid.size.split() {
                    let mut rng = rand::thread_rng();
                    for _ in 0..2 {
                        let angle = rng.gen_range(0.0..PI * 2.0);
                        let speed = rng.gen_range(ASTEROID_SPEED_MIN..ASTEROID_SPEED_MAX) * 1.3;
                        spawn_asteroid(&mut commands, pos, angle, speed, smaller);
                    }
                }
                break;
            }
        }
    }
}

fn check_ship_asteroid(
    mut commands: Commands,
    mut score: ResMut<GameScore>,
    mut next: ResMut<NextState<GameState>>,
    ship_q: Query<(Entity, &Transform), With<Ship>>,
    asteroids: Query<(&Transform, &Asteroid)>,
    score_text: Query<Entity, With<ScoreText>>,
) {
    let Ok((ship_entity, ship_transform)) = ship_q.get_single() else { return };

    for (a_transform, asteroid) in &asteroids {
        let dist = ship_transform.translation.truncate()
            .distance(a_transform.translation.truncate());
        if dist < asteroid.size.radius() + SHIP_SIZE * 0.5 {
            let pos = ship_transform.translation.truncate();
            spawn_particles(&mut commands, pos, 30.0);

            score.lives = score.lives.saturating_sub(1);
            if score.lives == 0 {
                if score.score > score.high_score {
                    score.high_score = score.score;
                }
                commands.entity(ship_entity).despawn();
                for e in &score_text { commands.entity(e).despawn(); }
                // Despawn all asteroids and bullets
                next.set(GameState::GameOver);
            } else {
                // Respawn ship at center
                commands.entity(ship_entity).insert(Transform::from_xyz(0.0, 0.0, 1.0));
                commands.entity(ship_entity).insert(Velocity(Vec2::ZERO));
            }
            break;
        }
    }
}

fn spawn_particles(commands: &mut Commands, pos: Vec2, spread: f32) {
    let mut rng = rand::thread_rng();
    for _ in 0..PARTICLE_COUNT {
        let angle = rng.gen_range(0.0..PI * 2.0);
        let speed = rng.gen_range(50.0..200.0);
        let lifetime = rng.gen_range(0.3..0.8);
        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.8, 0.3),
                custom_size: Some(Vec2::splat(rng.gen_range(2.0..6.0))),
                ..default()
            },
            Transform::from_xyz(
                pos.x + rng.gen_range(-spread..spread) * 0.3,
                pos.y + rng.gen_range(-spread..spread) * 0.3,
                2.0,
            ),
            Velocity(Vec2::new(angle.cos(), angle.sin()) * speed),
            Particle { lifetime, max_lifetime: lifetime },
        ));
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Sprite)>,
) {
    for (entity, mut particle, mut sprite) in &mut query {
        particle.lifetime -= time.delta_secs();
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            let alpha = particle.lifetime / particle.max_lifetime;
            sprite.color = Color::srgba(1.0, 0.8, 0.3, alpha);
        }
    }
}

fn update_score_ui(
    score: Res<GameScore>,
    mut query: Query<&mut Text2d, With<ScoreText>>,
) {
    for mut text in &mut query {
        **text = format!("Score: {}  Lives: {}", score.score, score.lives);
    }
}

fn spawn_game_over(mut commands: Commands, score: Res<GameScore>) {
    commands.spawn((
        Text2d::new("GAME OVER"),
        TextFont { font_size: 48.0, ..default() },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
        Transform::from_xyz(0.0, 60.0, 10.0),
        GameOverText,
    ));
    commands.spawn((
        Text2d::new(format!("Score: {}  Best: {}", score.score, score.high_score)),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 10.0),
        GameOverText,
    ));
    commands.spawn((
        Text2d::new("Click or SPACE to Restart"),
        TextFont { font_size: 18.0, ..default() },
        TextColor(Color::srgb(0.6, 0.6, 0.7)),
        Transform::from_xyz(0.0, -40.0, 10.0),
        GameOverText,
    ));
}

fn game_over_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut next: ResMut<NextState<GameState>>,
    mut commands: Commands,
    all_asteroids: Query<Entity, With<Asteroid>>,
    all_bullets: Query<Entity, With<Bullet>>,
    all_particles: Query<Entity, With<Particle>>,
) {
    if keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) || touches.any_just_pressed() {
        // Clean up everything
        for e in &all_asteroids { commands.entity(e).despawn(); }
        for e in &all_bullets { commands.entity(e).despawn(); }
        for e in &all_particles { commands.entity(e).despawn(); }
        next.set(GameState::Playing);
    }
}

fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
