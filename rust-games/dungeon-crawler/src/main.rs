use bevy::prelude::*;
use rand::Rng;

// ── Constants ──────────────────────────────────────────────────────────
const TILE_SIZE: f32 = 24.0;
const MAP_WIDTH: usize = 60;
const MAP_HEIGHT: usize = 40;
const MAX_ROOMS: usize = 12;
const MIN_ROOM_SIZE: usize = 4;
const MAX_ROOM_SIZE: usize = 10;
const PLAYER_SPEED: f32 = 150.0;
const ENEMY_SPEED: f32 = 60.0;
const PLAYER_MAX_HP: i32 = 100;
const PLAYER_ATTACK: i32 = 15;
const ENEMY_HP: i32 = 30;
const ENEMY_ATTACK: i32 = 8;
const ATTACK_COOLDOWN: f32 = 0.5;
const ENEMY_ATTACK_COOLDOWN: f32 = 1.0;
const HEAL_AMOUNT: i32 = 25;

// ── Tile Types ─────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Wall,
    Floor,
    Exit,
}

// ── Resources ──────────────────────────────────────────────────────────
#[derive(Resource)]
struct DungeonMap {
    tiles: Vec<Vec<TileType>>,
    rooms: Vec<Rect>,
}

#[derive(Resource)]
struct DungeonLevel(u32);

#[derive(Resource)]
struct GameOver(bool);

// ── Components ─────────────────────────────────────────────────────────
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Item {
    kind: ItemKind,
}

#[derive(Clone, Copy)]
enum ItemKind {
    HealthPotion,
    Sword,
    Shield,
}

#[derive(Component)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Component)]
struct Attack(i32);

#[derive(Component)]
struct Defense(i32);

#[derive(Component)]
struct AttackTimer(f32);

#[derive(Component)]
struct GridPos {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct MapTile;

#[derive(Component)]
struct HpBar;

#[derive(Component)]
struct HpBarBg;

#[derive(Component)]
struct LevelText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct DamageFlash(f32);

#[derive(Component)]
struct FloatingText {
    timer: f32,
    velocity: Vec2,
}

#[derive(Component)]
struct MainCamera;

// ── Rect helper ────────────────────────────────────────────────────────
#[derive(Clone)]
struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Rect {
    fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }
    fn center(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
    fn intersects(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
}

// ── App ────────────────────────────────────────────────────────────────
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dungeon Crawler".to_string(),
                resolution: bevy::window::WindowResolution::new(960.0, 640.0),
                canvas: Some("#bevy-canvas".to_string()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .insert_resource(DungeonLevel(1))
        .insert_resource(GameOver(false))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement,
            camera_follow,
            enemy_ai,
            combat_system,
            item_pickup,
            check_exit,
            update_hp_bar,
            update_ui_text,
            damage_flash_system,
            floating_text_system,
            game_over_system,
        ))
        .run();
}

// ── Dungeon Generation ────────────────────────────────────────────────
fn generate_dungeon(level: u32) -> DungeonMap {
    let mut rng = rand::thread_rng();
    let mut tiles = vec![vec![TileType::Wall; MAP_WIDTH]; MAP_HEIGHT];
    let mut rooms: Vec<Rect> = Vec::new();

    let num_rooms = MAX_ROOMS + (level as usize / 2).min(4);

    for _ in 0..100 {
        if rooms.len() >= num_rooms {
            break;
        }
        let w = rng.gen_range(MIN_ROOM_SIZE..=MAX_ROOM_SIZE);
        let h = rng.gen_range(MIN_ROOM_SIZE..=MAX_ROOM_SIZE);
        let x = rng.gen_range(1..MAP_WIDTH - w - 1);
        let y = rng.gen_range(1..MAP_HEIGHT - h - 1);
        let new_room = Rect::new(x, y, w, h);

        if rooms.iter().any(|r| new_room.intersects(r)) {
            continue;
        }

        // Carve room
        for ry in new_room.y1..new_room.y2 {
            for rx in new_room.x1..new_room.x2 {
                tiles[ry][rx] = TileType::Floor;
            }
        }

        // Carve corridor to previous room
        if !rooms.is_empty() {
            let (cx, cy) = new_room.center();
            let (px, py) = rooms.last().unwrap().center();

            if rng.gen_bool(0.5) {
                carve_h_corridor(&mut tiles, cx, px, cy);
                carve_v_corridor(&mut tiles, cy, py, px);
            } else {
                carve_v_corridor(&mut tiles, cy, py, cx);
                carve_h_corridor(&mut tiles, cx, px, py);
            }
        }

        rooms.push(new_room);
    }

    // Place exit in last room
    if let Some(last) = rooms.last() {
        let (ex, ey) = last.center();
        tiles[ey][ex] = TileType::Exit;
    }

    DungeonMap { tiles, rooms }
}

fn carve_h_corridor(tiles: &mut [Vec<TileType>], x1: usize, x2: usize, y: usize) {
    let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    for x in start..=end {
        if y < MAP_HEIGHT && x < MAP_WIDTH {
            tiles[y][x] = TileType::Floor;
        }
    }
}

fn carve_v_corridor(tiles: &mut [Vec<TileType>], y1: usize, y2: usize, x: usize) {
    let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
    for y in start..=end {
        if y < MAP_HEIGHT && x < MAP_WIDTH {
            tiles[y][x] = TileType::Floor;
        }
    }
}

// ── Setup ──────────────────────────────────────────────────────────────
fn setup(mut commands: Commands, level: Res<DungeonLevel>) {
    // Camera
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));

    spawn_level(&mut commands, level.0);
}

fn spawn_level(commands: &mut Commands, level: u32) {
    let dungeon = generate_dungeon(level);
    let mut rng = rand::thread_rng();

    // Render map tiles
    for (y, row) in dungeon.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let color = match tile {
                TileType::Wall => Color::srgb(0.15, 0.12, 0.2),
                TileType::Floor => Color::srgb(0.25, 0.22, 0.3),
                TileType::Exit => Color::srgb(0.2, 0.8, 0.4),
            };

            let pos = grid_to_world(x as i32, y as i32);
            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(TILE_SIZE - 1.0)),
                    ..default()
                },
                Transform::from_translation(pos.extend(0.0)),
                MapTile,
            ));

            // Add stairs icon for exit
            if *tile == TileType::Exit {
                commands.spawn((
                    Text2d::new("▼"),
                    TextColor(Color::srgb(0.1, 0.5, 0.2)),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    Transform::from_translation(pos.extend(1.0)),
                    MapTile,
                ));
            }
        }
    }

    // Spawn player in first room
    let player_room = &dungeon.rooms[0];
    let (px, py) = player_room.center();
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.6, 1.0),
            custom_size: Some(Vec2::splat(TILE_SIZE - 4.0)),
            ..default()
        },
        Transform::from_translation(grid_to_world(px as i32, py as i32).extend(5.0)),
        Player,
        Health { current: PLAYER_MAX_HP, max: PLAYER_MAX_HP },
        Attack(PLAYER_ATTACK),
        Defense(0),
        AttackTimer(0.0),
        GridPos { x: px as i32, y: py as i32 },
    ));

    // Player icon
    commands.spawn((
        Text2d::new("@"),
        TextColor(Color::srgb(0.3, 0.7, 1.0)),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Transform::from_translation(grid_to_world(px as i32, py as i32).extend(6.0)),
        Player,
        GridPos { x: px as i32, y: py as i32 },
    ));

    // Spawn enemies in rooms (skip first room)
    let enemy_count = 2 + level as usize;
    for (i, room) in dungeon.rooms.iter().enumerate().skip(1) {
        let enemies_in_room = if i == dungeon.rooms.len() - 1 { 2 } else { rng.gen_range(1..=enemy_count.min(3)) };
        for _ in 0..enemies_in_room {
            let ex = rng.gen_range(room.x1 + 1..room.x2.saturating_sub(1).max(room.x1 + 2));
            let ey = rng.gen_range(room.y1 + 1..room.y2.saturating_sub(1).max(room.y1 + 2));
            let hp = ENEMY_HP + (level as i32 - 1) * 10;
            let atk = ENEMY_ATTACK + (level as i32 - 1) * 3;

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.9, 0.2, 0.2),
                    custom_size: Some(Vec2::splat(TILE_SIZE - 6.0)),
                    ..default()
                },
                Transform::from_translation(grid_to_world(ex as i32, ey as i32).extend(4.0)),
                Enemy,
                Health { current: hp, max: hp },
                Attack(atk),
                AttackTimer(0.0),
                GridPos { x: ex as i32, y: ey as i32 },
            ));
        }
    }

    // Spawn items in random rooms
    for room in dungeon.rooms.iter().skip(1) {
        if rng.gen_bool(0.6) {
            let ix = rng.gen_range(room.x1 + 1..room.x2.saturating_sub(1).max(room.x1 + 2));
            let iy = rng.gen_range(room.y1 + 1..room.y2.saturating_sub(1).max(room.y1 + 2));
            let kind = match rng.gen_range(0..3) {
                0 => ItemKind::HealthPotion,
                1 => ItemKind::Sword,
                _ => ItemKind::Shield,
            };
            let (color, icon) = match kind {
                ItemKind::HealthPotion => (Color::srgb(1.0, 0.3, 0.5), "+"),
                ItemKind::Sword => (Color::srgb(1.0, 0.8, 0.2), "†"),
                ItemKind::Shield => (Color::srgb(0.5, 0.7, 1.0), "O"),
            };
            commands.spawn((
                Text2d::new(icon),
                TextColor(color),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                Transform::from_translation(grid_to_world(ix as i32, iy as i32).extend(3.0)),
                Item { kind },
                GridPos { x: ix as i32, y: iy as i32 },
            ));
        }
    }

    commands.insert_resource(dungeon);

    // UI - HP Bar background
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            width: Val::Px(204.0),
            height: Val::Px(24.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        HpBarBg,
    )).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
            HpBar,
        ));
    });

    // Level text
    commands.spawn((
        Text::new(format!("Level: {}", level)),
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(50.0),
            ..default()
        },
        LevelText,
    ));

    // Score/info text
    commands.spawn((
        Text::new("HP: 100/100 | ATK: 15 | DEF: 0"),
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(240.0),
            top: Val::Px(22.0),
            ..default()
        },
        ScoreText,
    ));
}

fn grid_to_world(x: i32, y: i32) -> Vec2 {
    Vec2::new(
        x as f32 * TILE_SIZE - (MAP_WIDTH as f32 * TILE_SIZE / 2.0),
        -(y as f32 * TILE_SIZE - (MAP_HEIGHT as f32 * TILE_SIZE / 2.0)),
    )
}

fn world_to_grid(pos: Vec2) -> (i32, i32) {
    let x = ((pos.x + MAP_WIDTH as f32 * TILE_SIZE / 2.0) / TILE_SIZE).round() as i32;
    let y = (-(pos.y - MAP_HEIGHT as f32 * TILE_SIZE / 2.0) / TILE_SIZE).round() as i32;
    (x, y)
}

fn is_walkable(map: &DungeonMap, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 || x >= MAP_WIDTH as i32 || y >= MAP_HEIGHT as i32 {
        return false;
    }
    map.tiles[y as usize][x as usize] != TileType::Wall
}

// ── Player Movement ───────────────────────────────────────────────────
fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    map: Res<DungeonMap>,
    game_over: Res<GameOver>,
    mut query: Query<(&mut Transform, &mut GridPos), With<Player>>,
) {
    if game_over.0 {
        return;
    }

    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }

    if dir == Vec2::ZERO {
        return;
    }

    dir = dir.normalize();
    let delta = dir * PLAYER_SPEED * time.delta_secs();

    for (mut transform, mut grid_pos) in query.iter_mut() {
        let new_pos = transform.translation.truncate() + delta;
        let (gx, gy) = world_to_grid(new_pos);

        if is_walkable(&map, gx, gy) {
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
            grid_pos.x = gx;
            grid_pos.y = gy;
        }
    }
}

// ── Camera Follow ─────────────────────────────────────────────────────
fn camera_follow(
    player_q: Query<&Transform, (With<Player>, Without<MainCamera>, Without<Enemy>)>,
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
) {
    let mut player_pos = None;
    for t in player_q.iter() {
        player_pos = Some(t.translation);
    }
    if let Some(pos) = player_pos {
        for mut cam_t in camera_q.iter_mut() {
            cam_t.translation.x += (pos.x - cam_t.translation.x) * 0.1;
            cam_t.translation.y += (pos.y - cam_t.translation.y) * 0.1;
        }
    }
}

// ── Enemy AI ──────────────────────────────────────────────────────────
fn enemy_ai(
    time: Res<Time>,
    map: Res<DungeonMap>,
    game_over: Res<GameOver>,
    player_q: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<(&mut Transform, &mut GridPos), With<Enemy>>,
) {
    if game_over.0 {
        return;
    }

    let mut player_pos = None;
    for t in player_q.iter() {
        player_pos = Some(t.translation.truncate());
        break;
    }
    let player_pos = match player_pos {
        Some(p) => p,
        None => return,
    };

    for (mut transform, mut grid_pos) in enemy_q.iter_mut() {
        let enemy_pos = transform.translation.truncate();
        let dist = enemy_pos.distance(player_pos);

        // Only chase if within range
        if dist < TILE_SIZE * 8.0 && dist > TILE_SIZE * 0.8 {
            let dir = (player_pos - enemy_pos).normalize();
            let delta = dir * ENEMY_SPEED * time.delta_secs();
            let new_pos = enemy_pos + delta;
            let (gx, gy) = world_to_grid(new_pos);

            if is_walkable(&map, gx, gy) {
                transform.translation.x = new_pos.x;
                transform.translation.y = new_pos.y;
                grid_pos.x = gx;
                grid_pos.y = gy;
            }
        }
    }
}

// ── Combat ────────────────────────────────────────────────────────────
fn combat_system(
    mut commands: Commands,
    time: Res<Time>,
    game_over: Res<GameOver>,
    mut player_q: Query<(&Transform, &Attack, &Defense, &mut Health, &mut AttackTimer), (With<Player>, Without<Enemy>)>,
    mut enemy_q: Query<(Entity, &Transform, &Attack, &mut Health, &mut AttackTimer), With<Enemy>>,
) {
    if game_over.0 {
        return;
    }

    for (p_transform, p_attack, p_defense, mut p_health, mut p_atk_timer) in player_q.iter_mut() {
        p_atk_timer.0 -= time.delta_secs();

        for (e_entity, e_transform, e_attack, mut e_health, mut e_atk_timer) in enemy_q.iter_mut() {
            let dist = p_transform.translation.truncate().distance(e_transform.translation.truncate());

            if dist < TILE_SIZE * 1.2 {
                // Player attacks enemy
                if p_atk_timer.0 <= 0.0 {
                    let dmg = p_attack.0;
                    e_health.current -= dmg;
                    p_atk_timer.0 = ATTACK_COOLDOWN;

                    // Spawn damage number
                    spawn_floating_text(&mut commands, e_transform.translation.truncate(), &format!("-{}", dmg), Color::srgb(1.0, 1.0, 0.3));
                }

                // Enemy attacks player
                e_atk_timer.0 -= time.delta_secs();
                if e_atk_timer.0 <= 0.0 {
                    let dmg = (e_attack.0 - p_defense.0).max(1);
                    p_health.current -= dmg;
                    e_atk_timer.0 = ENEMY_ATTACK_COOLDOWN;

                    spawn_floating_text(&mut commands, p_transform.translation.truncate(), &format!("-{}", dmg), Color::srgb(1.0, 0.3, 0.3));
                }

                // Remove dead enemies
                if e_health.current <= 0 {
                    commands.entity(e_entity).despawn();
                    spawn_floating_text(&mut commands, e_transform.translation.truncate(), "SLAIN!", Color::srgb(1.0, 0.5, 0.0));
                }
            }
        }
    }
}

fn spawn_floating_text(commands: &mut Commands, pos: Vec2, text: &str, color: Color) {
    commands.spawn((
        Text2d::new(text.to_string()),
        TextColor(color),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        Transform::from_translation(pos.extend(10.0)),
        FloatingText {
            timer: 1.0,
            velocity: Vec2::new(0.0, 40.0),
        },
    ));
}

fn floating_text_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut FloatingText)>,
) {
    for (entity, mut transform, mut ft) in query.iter_mut() {
        ft.timer -= time.delta_secs();
        transform.translation.x += ft.velocity.x * time.delta_secs();
        transform.translation.y += ft.velocity.y * time.delta_secs();
        if ft.timer <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn damage_flash_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &mut DamageFlash)>,
) {
    for (entity, mut sprite, mut flash) in query.iter_mut() {
        flash.0 -= time.delta_secs();
        if flash.0 <= 0.0 {
            sprite.color = Color::srgb(0.2, 0.6, 1.0);
            commands.entity(entity).remove::<DamageFlash>();
        } else {
            sprite.color = Color::srgb(1.0, 0.3, 0.3);
        }
    }
}

// ── Item Pickup ───────────────────────────────────────────────────────
fn item_pickup(
    mut commands: Commands,
    player_q: Query<&Transform, (With<Player>, Without<Item>)>,
    item_q: Query<(Entity, &Transform, &Item)>,
    mut health_q: Query<&mut Health, With<Player>>,
    mut attack_q: Query<&mut Attack, With<Player>>,
    mut defense_q: Query<&mut Defense, With<Player>>,
) {
    let mut player_pos = None;
    for t in player_q.iter() {
        player_pos = Some(t.translation.truncate());
        break;
    }
    let player_pos = match player_pos {
        Some(p) => p,
        None => return,
    };

    for (entity, transform, item) in item_q.iter() {
        let dist = player_pos.distance(transform.translation.truncate());
        if dist < TILE_SIZE * 0.8 {
            match item.kind {
                ItemKind::HealthPotion => {
                    for mut hp in health_q.iter_mut() {
                        hp.current = (hp.current + HEAL_AMOUNT).min(hp.max);
                    }
                    spawn_floating_text(&mut commands, transform.translation.truncate(), &format!("+{} HP", HEAL_AMOUNT), Color::srgb(0.3, 1.0, 0.5));
                }
                ItemKind::Sword => {
                    for mut atk in attack_q.iter_mut() {
                        atk.0 += 5;
                    }
                    spawn_floating_text(&mut commands, transform.translation.truncate(), "+5 ATK", Color::srgb(1.0, 0.8, 0.2));
                }
                ItemKind::Shield => {
                    for mut def in defense_q.iter_mut() {
                        def.0 += 3;
                    }
                    spawn_floating_text(&mut commands, transform.translation.truncate(), "+3 DEF", Color::srgb(0.5, 0.7, 1.0));
                }
            }
            commands.entity(entity).despawn();
        }
    }
}

// ── Check Exit ────────────────────────────────────────────────────────
fn check_exit(
    mut commands: Commands,
    map: Res<DungeonMap>,
    mut level: ResMut<DungeonLevel>,
    game_over: Res<GameOver>,
    player_q: Query<&GridPos, With<Player>>,
    all_entities: Query<Entity, Or<(With<MapTile>, With<Player>, With<Enemy>, With<Item>, With<HpBar>, With<HpBarBg>, With<LevelText>, With<ScoreText>, With<FloatingText>)>>,
) {
    if game_over.0 {
        return;
    }

    for grid_pos in player_q.iter() {
        let x = grid_pos.x;
        let y = grid_pos.y;
        if x >= 0 && y >= 0 && (x as usize) < MAP_WIDTH && (y as usize) < MAP_HEIGHT {
            if map.tiles[y as usize][x as usize] == TileType::Exit {
                // Clear everything
                for entity in all_entities.iter() {
                    commands.entity(entity).despawn();
                }

                level.0 += 1;
                spawn_level(&mut commands, level.0);
                return;
            }
        }
    }
}

// ── UI Updates ────────────────────────────────────────────────────────
fn update_hp_bar(
    player_q: Query<&Health, With<Player>>,
    mut hp_bar_q: Query<&mut Node, With<HpBar>>,
) {
    for health in player_q.iter() {
        let pct = (health.current as f32 / health.max as f32 * 100.0).max(0.0);
        for mut node in hp_bar_q.iter_mut() {
            node.width = Val::Percent(pct);
        }
        break;
    }
}

fn update_ui_text(
    player_q: Query<(&Health, &Attack, &Defense), With<Player>>,
    level: Res<DungeonLevel>,
    mut level_text_q: Query<&mut Text, (With<LevelText>, Without<ScoreText>)>,
    mut score_text_q: Query<&mut Text, (With<ScoreText>, Without<LevelText>)>,
) {
    for (health, attack, defense) in player_q.iter() {
        for mut text in level_text_q.iter_mut() {
            **text = format!("Level: {}", level.0);
        }
        for mut text in score_text_q.iter_mut() {
            **text = format!("HP: {}/{} | ATK: {} | DEF: {}", health.current, health.max, attack.0, defense.0);
        }
        break;
    }
}

// ── Game Over ─────────────────────────────────────────────────────────
fn game_over_system(
    mut commands: Commands,
    mut game_over: ResMut<GameOver>,
    keys: Res<ButtonInput<KeyCode>>,
    mut level: ResMut<DungeonLevel>,
    player_q: Query<&Health, With<Player>>,
    _game_over_text_q: Query<Entity, With<GameOverText>>,
    all_entities: Query<Entity, Or<(With<MapTile>, With<Player>, With<Enemy>, With<Item>, With<HpBar>, With<HpBarBg>, With<LevelText>, With<ScoreText>, With<FloatingText>, With<GameOverText>)>>,
) {
    // Check if player died
    for health in player_q.iter() {
        if health.current <= 0 && !game_over.0 {
            game_over.0 = true;

            // Show game over
            commands.spawn((
                Text::new(format!("GAME OVER - Reached Level {}\nPress R to restart", level.0)),
                TextColor(Color::srgb(1.0, 0.3, 0.3)),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(25.0),
                    top: Val::Percent(40.0),
                    ..default()
                },
                GameOverText,
            ));
        }
    }

    // Restart
    if game_over.0 && keys.just_pressed(KeyCode::KeyR) {
        for entity in all_entities.iter() {
            commands.entity(entity).despawn();
        }
        game_over.0 = false;
        level.0 = 1;
        spawn_level(&mut commands, 1);
    }
}
