use bevy::prelude::*;
use crate::components::*;
use crate::levels::*;

// ── Cleanup ──

pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ── Menu ──

pub fn spawn_menu(mut commands: Commands, game_data: Res<GameData>) {
    let is_fresh = game_data.score == 0 && game_data.current_level == 0;

    commands.spawn((
        MenuTag,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Platform Jumper"),
            TextFont { font_size: 60.0, ..default() },
            TextColor(Color::srgb(1.0, 0.85, 0.2)),
        ));

        if is_fresh {
            parent.spawn((
                Text::new("A 2D Platformer Adventure"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.8)),
            ));
        }

        parent.spawn((
            Text::new(""),
            TextFont { font_size: 10.0, ..default() },
            TextColor(Color::NONE),
        ));

        let instructions = [
            "Arrow Keys / WASD - Move & Jump",
            "Collect coins, avoid enemies",
            "Jump on enemies to defeat them",
            "Reach the green exit to complete the level",
        ];
        for line in instructions {
            parent.spawn((
                Text::new(line),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.8, 0.8, 0.9)),
            ));
        }

        parent.spawn((
            Text::new(""),
            TextFont { font_size: 10.0, ..default() },
            TextColor(Color::NONE),
        ));

        parent.spawn((
            Text::new("Press ENTER or SPACE to Start"),
            TextFont { font_size: 28.0, ..default() },
            TextColor(Color::srgb(0.4, 1.0, 0.4)),
        ));
    });
}

pub fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        game_data.current_level = 0;
        game_data.score = 0;
        game_data.lives = 3;
        next_state.set(GameState::Playing);
    }
}

// ── Level Setup ──

pub fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_data: ResMut<GameData>,
) {
    let level = get_level(game_data.current_level);
    game_data.coins_collected = 0;
    game_data.coins_total = level.coins.len() as u32;

    // Camera
    commands.spawn((
        LevelTag,
        Camera2d,
    ));

    // Player
    let player_mesh = meshes.add(Rectangle::new(PLAYER_SIZE.x, PLAYER_SIZE.y));
    let player_mat = materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.6, 1.0)));
    commands.spawn((
        LevelTag,
        Player::default(),
        Velocity::default(),
        Mesh2d(player_mesh),
        MeshMaterial2d(player_mat),
        Transform::from_translation(level.player_start.extend(10.0)),
    ));

    // Static platforms
    for p in &level.platforms {
        let mesh = meshes.add(Rectangle::new(p.width, p.height));
        let mat = materials.add(ColorMaterial::from_color(Color::srgb(0.35, 0.25, 0.15)));
        commands.spawn((
            LevelTag,
            Platform { width: p.width, height: p.height },
            Mesh2d(mesh),
            MeshMaterial2d(mat),
            Transform::from_translation(p.pos.extend(0.0)),
        ));

        // Platform top highlight
        let top_mesh = meshes.add(Rectangle::new(p.width, 4.0));
        let top_mat = materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.7, 0.2)));
        commands.spawn((
            LevelTag,
            Mesh2d(top_mesh),
            MeshMaterial2d(top_mat),
            Transform::from_translation(Vec3::new(p.pos.x, p.pos.y + p.height / 2.0 - 2.0, 1.0)),
        ));
    }

    // Moving platforms
    for mp in &level.moving_platforms {
        let mesh = meshes.add(Rectangle::new(mp.width, 20.0));
        let mat = materials.add(ColorMaterial::from_color(Color::srgb(0.6, 0.3, 0.1)));
        commands.spawn((
            LevelTag,
            Platform { width: mp.width, height: 20.0 },
            MovingPlatform {
                start: mp.start,
                end: mp.end,
                speed: mp.speed,
                progress: 0.0,
                forward: true,
            },
            Mesh2d(mesh),
            MeshMaterial2d(mat),
            Transform::from_translation(mp.start.extend(0.0)),
        ));

        // Moving platform highlight
        let top_mesh = meshes.add(Rectangle::new(mp.width, 4.0));
        let top_mat = materials.add(ColorMaterial::from_color(Color::srgb(1.0, 0.6, 0.1)));
        commands.spawn((
            LevelTag,
            Mesh2d(top_mesh),
            MeshMaterial2d(top_mat),
            Transform::from_translation(Vec3::new(mp.start.x, mp.start.y + 8.0, 1.0)),
        ));
    }

    // Coins
    for c in &level.coins {
        let mesh = meshes.add(Circle::new(10.0));
        let mat = materials.add(ColorMaterial::from_color(Color::srgb(1.0, 0.85, 0.0)));
        commands.spawn((
            LevelTag,
            Coin,
            CoinBob { base_y: c.y, time: 0.0 },
            Mesh2d(mesh),
            MeshMaterial2d(mat),
            Transform::from_translation(c.extend(5.0)),
        ));
    }

    // Enemies
    for e in &level.enemies {
        let mesh = meshes.add(Rectangle::new(26.0, 26.0));
        let mat = materials.add(ColorMaterial::from_color(Color::srgb(0.9, 0.15, 0.15)));
        commands.spawn((
            LevelTag,
            Enemy {
                patrol_left: e.patrol_left,
                patrol_right: e.patrol_right,
                speed: e.speed,
                moving_right: true,
            },
            Mesh2d(mesh),
            MeshMaterial2d(mat),
            Transform::from_translation(e.pos.extend(2.0)),
        ));
    }

    // Level Exit
    let exit_mesh = meshes.add(Rectangle::new(30.0, 50.0));
    let exit_mat = materials.add(ColorMaterial::from_color(Color::srgb(0.1, 0.9, 0.3)));
    commands.spawn((
        LevelTag,
        LevelExit,
        Mesh2d(exit_mesh),
        MeshMaterial2d(exit_mat),
        Transform::from_translation(level.exit_pos.extend(0.0)),
    ));

    // Exit arrow indicator
    let arrow_mesh = meshes.add(Triangle2d::new(
        Vec2::new(-12.0, 0.0),
        Vec2::new(12.0, 0.0),
        Vec2::new(0.0, 16.0),
    ));
    let arrow_mat = materials.add(ColorMaterial::from_color(Color::srgb(0.1, 1.0, 0.3)));
    commands.spawn((
        LevelTag,
        Mesh2d(arrow_mesh),
        MeshMaterial2d(arrow_mat),
        Transform::from_translation(Vec3::new(level.exit_pos.x, level.exit_pos.y + 40.0, 1.0)),
    ));

    // HUD
    commands.spawn((
        LevelTag,
        Node {
            width: Val::Percent(100.0),
            height: Val::Auto,
            padding: UiRect::all(Val::Px(12.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            HudScore,
            Text::new(format!("Score: {}", game_data.score)),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
        ));
        parent.spawn((
            HudCoins,
            Text::new(format!("Coins: {}/{}", game_data.coins_collected, game_data.coins_total)),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(1.0, 0.85, 0.0)),
        ));
        parent.spawn((
            HudLives,
            Text::new(format!("Lives: {}", game_data.lives)),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(1.0, 0.3, 0.3)),
        ));
    });

    // Background decoration - stars
    let star_mesh = meshes.add(Circle::new(2.0));
    let star_mat = materials.add(ColorMaterial::from_color(Color::srgba(1.0, 1.0, 1.0, 0.4)));
    let star_positions = [
        (-300.0, 200.0), (-100.0, 250.0), (100.0, 180.0), (300.0, 270.0),
        (500.0, 220.0), (700.0, 260.0), (900.0, 190.0), (1100.0, 240.0),
        (1300.0, 210.0), (1500.0, 270.0), (1700.0, 230.0), (1900.0, 250.0),
        (-200.0, 150.0), (0.0, 300.0), (400.0, 290.0), (800.0, 150.0),
    ];
    for (x, y) in star_positions {
        commands.spawn((
            LevelTag,
            Mesh2d(star_mesh.clone()),
            MeshMaterial2d(star_mat.clone()),
            Transform::from_translation(Vec3::new(x, y, -10.0)),
        ));
    }
}

// ── Player Input ──

pub fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player)>,
    time: Res<Time>,
) {
    for (mut vel, mut player) in &mut query {
        // Horizontal movement
        let mut dir = 0.0;
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            dir -= 1.0;
            player.facing_right = false;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            dir += 1.0;
            player.facing_right = true;
        }

        // Smooth acceleration/deceleration
        let target_vx = dir * PLAYER_SPEED;
        let accel = if dir != 0.0 { 12.0 } else { 18.0 };
        vel.x += (target_vx - vel.x) * (accel * time.delta_secs()).min(1.0);

        // Update coyote time
        if player.is_grounded {
            player.coyote_time = COYOTE_TIME;
        } else {
            player.coyote_time -= time.delta_secs();
        }

        // Jump
        let jump_pressed = keyboard.just_pressed(KeyCode::ArrowUp)
            || keyboard.just_pressed(KeyCode::KeyW)
            || keyboard.just_pressed(KeyCode::Space);

        if jump_pressed && player.coyote_time > 0.0 {
            vel.y = JUMP_FORCE;
            player.is_grounded = false;
            player.coyote_time = 0.0;
            player.jump_timer = 0.25;
        }

        // Variable jump height - hold for higher jump
        let jump_held = keyboard.pressed(KeyCode::ArrowUp)
            || keyboard.pressed(KeyCode::KeyW)
            || keyboard.pressed(KeyCode::Space);

        if jump_held && player.jump_timer > 0.0 {
            player.jump_timer -= time.delta_secs();
        } else if !jump_held && vel.y > 0.0 {
            vel.y *= 0.5;
            player.jump_timer = 0.0;
        }
    }
}

// ── Physics ──

pub fn apply_gravity(
    mut query: Query<(&mut Velocity, &Player)>,
    time: Res<Time>,
) {
    for (mut vel, _player) in &mut query {
        vel.y += GRAVITY * time.delta_secs();
        vel.y = vel.y.max(MAX_FALL_SPEED);
    }
}

pub fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, vel) in &mut query {
        transform.translation.x += vel.x * time.delta_secs();
        transform.translation.y += vel.y * time.delta_secs();
    }
}

// ── Platform Collisions ──

pub fn check_platform_collisions(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Player)>,
    platform_query: Query<(&Transform, &Platform), Without<Player>>,
) {
    for (mut p_tf, mut p_vel, mut player) in &mut player_query {
        player.is_grounded = false;
        let px = p_tf.translation.x;
        let py = p_tf.translation.y;
        let half_pw = PLAYER_SIZE.x / 2.0;
        let half_ph = PLAYER_SIZE.y / 2.0;

        for (plat_tf, plat) in &platform_query {
            let plat_x = plat_tf.translation.x;
            let plat_y = plat_tf.translation.y;
            let half_plat_w = plat.width / 2.0;
            let half_plat_h = plat.height / 2.0;

            // AABB overlap check
            let overlap_x = (half_pw + half_plat_w) - (px - plat_x).abs();
            let overlap_y = (half_ph + half_plat_h) - (py - plat_y).abs();

            if overlap_x > 0.0 && overlap_y > 0.0 {
                // Resolve collision
                if overlap_x < overlap_y {
                    // Side collision
                    if px < plat_x {
                        p_tf.translation.x -= overlap_x;
                    } else {
                        p_tf.translation.x += overlap_x;
                    }
                    p_vel.x = 0.0;
                } else {
                    // Top/bottom collision
                    if py > plat_y {
                        // Landing on top
                        p_tf.translation.y += overlap_y;
                        p_vel.y = 0.0;
                        player.is_grounded = true;
                    } else {
                        // Hitting bottom
                        p_tf.translation.y -= overlap_y;
                        if p_vel.y > 0.0 {
                            p_vel.y = 0.0;
                        }
                    }
                }
            }
        }
    }
}

// ── Moving Platforms ──

pub fn moving_platform_system(
    mut query: Query<(&mut Transform, &mut MovingPlatform)>,
    time: Res<Time>,
) {
    for (mut transform, mut mp) in &mut query {
        if mp.forward {
            mp.progress += mp.speed * time.delta_secs();
            if mp.progress >= 1.0 {
                mp.progress = 1.0;
                mp.forward = false;
            }
        } else {
            mp.progress -= mp.speed * time.delta_secs();
            if mp.progress <= 0.0 {
                mp.progress = 0.0;
                mp.forward = true;
            }
        }

        // Smooth interpolation
        let t = mp.progress;
        let smooth_t = t * t * (3.0 - 2.0 * t);
        let new_pos = mp.start.lerp(mp.end, smooth_t);
        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
    }
}

// ── Coins ──

pub fn coin_collection(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut coin_query: Query<(Entity, &Transform, &mut CoinBob), With<Coin>>,
    mut game_data: ResMut<GameData>,
    time: Res<Time>,
) {
    let Ok(p_tf) = player_query.get_single() else { return };

    for (entity, c_tf, mut bob) in &mut coin_query {
        // Bob animation
        bob.time += time.delta_secs() * 3.0;

        let dx = (p_tf.translation.x - c_tf.translation.x).abs();
        let dy = (p_tf.translation.y - c_tf.translation.y).abs();

        if dx < 24.0 && dy < 28.0 {
            commands.entity(entity).despawn();
            game_data.coins_collected += 1;
            game_data.score += 100;
        }
    }
}

// ── Enemies ──

pub fn enemy_ai(
    mut query: Query<(&mut Transform, &mut Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, mut enemy) in &mut query {
        if enemy.moving_right {
            transform.translation.x += enemy.speed * time.delta_secs();
            if transform.translation.x >= enemy.patrol_right {
                enemy.moving_right = false;
            }
        } else {
            transform.translation.x -= enemy.speed * time.delta_secs();
            if transform.translation.x <= enemy.patrol_left {
                enemy.moving_right = true;
            }
        }
    }
}

pub fn enemy_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Velocity, &mut Player)>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((p_tf, mut p_vel, mut player)) = player_query.get_single_mut() else { return };

    for (entity, e_tf) in &enemy_query {
        let dx = (p_tf.translation.x - e_tf.translation.x).abs();
        let dy = p_tf.translation.y - e_tf.translation.y;

        if dx < 25.0 && dy.abs() < 30.0 {
            if dy > 8.0 && p_vel.y < 0.0 {
                // Stomp enemy from above
                commands.entity(entity).despawn();
                p_vel.y = JUMP_FORCE * 0.7;
                player.is_grounded = false;
                game_data.score += 200;
            } else {
                // Hit by enemy
                game_data.lives = game_data.lives.saturating_sub(1);
                if game_data.lives == 0 {
                    next_state.set(GameState::GameOver);
                } else {
                    // Respawn at level start
                    // Bounce-back approach on hit
                    p_vel.y = JUMP_FORCE * 0.6;
                    p_vel.x = if p_tf.translation.x < e_tf.translation.x { -200.0 } else { 200.0 };
                }
            }
        }
    }
}

// ── Camera ──

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Ok(p_tf) = player_query.get_single() else { return };
    let Ok(mut cam_tf) = camera_query.get_single_mut() else { return };

    let target_x = p_tf.translation.x;
    let target_y = (p_tf.translation.y + 50.0).max(-50.0);

    cam_tf.translation.x += (target_x - cam_tf.translation.x) * 0.08;
    cam_tf.translation.y += (target_y - cam_tf.translation.y) * 0.05;
}

// ── Death Zone ──

pub fn check_death_zone(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((mut p_tf, mut p_vel)) = player_query.get_single_mut() else { return };

    if p_tf.translation.y < DEATH_ZONE_Y {
        game_data.lives = game_data.lives.saturating_sub(1);
        if game_data.lives == 0 {
            next_state.set(GameState::GameOver);
        } else {
            let level = get_level(game_data.current_level);
            p_tf.translation = level.player_start.extend(10.0);
            p_vel.x = 0.0;
            p_vel.y = 0.0;
        }
    }
}

// ── Level Complete ──

pub fn check_level_complete(
    player_query: Query<&Transform, With<Player>>,
    exit_query: Query<&Transform, With<LevelExit>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
) {
    let Ok(p_tf) = player_query.get_single() else { return };

    for e_tf in &exit_query {
        let dx = (p_tf.translation.x - e_tf.translation.x).abs();
        let dy = (p_tf.translation.y - e_tf.translation.y).abs();
        if dx < 25.0 && dy < 35.0 {
            game_data.score += game_data.coins_collected * 50; // Bonus for coins
            next_state.set(GameState::LevelComplete);
        }
    }
}

// ── HUD ──

pub fn update_hud(
    game_data: Res<GameData>,
    mut score_query: Query<&mut Text, (With<HudScore>, Without<HudLives>, Without<HudCoins>)>,
    mut lives_query: Query<&mut Text, (With<HudLives>, Without<HudScore>, Without<HudCoins>)>,
    mut coins_query: Query<&mut Text, (With<HudCoins>, Without<HudScore>, Without<HudLives>)>,
) {
    for mut text in &mut score_query {
        **text = format!("Score: {}", game_data.score);
    }
    for mut text in &mut lives_query {
        **text = format!("Lives: {}", game_data.lives);
    }
    for mut text in &mut coins_query {
        **text = format!("Coins: {}/{}", game_data.coins_collected, game_data.coins_total);
    }
}

// ── Pause ──

pub fn pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

// ── Level Complete Screen ──

pub fn spawn_level_complete_screen(
    mut commands: Commands,
    game_data: Res<GameData>,
) {
    let is_final = game_data.current_level >= TOTAL_LEVELS - 1;

    commands.spawn((
        MenuTag,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(16.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
    )).with_children(|parent| {
        let title = if is_final { "You Win!" } else { "Level Complete!" };
        parent.spawn((
            Text::new(title),
            TextFont { font_size: 52.0, ..default() },
            TextColor(Color::srgb(0.2, 1.0, 0.4)),
        ));

        parent.spawn((
            Text::new(format!("Score: {}", game_data.score)),
            TextFont { font_size: 28.0, ..default() },
            TextColor(Color::WHITE),
        ));

        parent.spawn((
            Text::new(format!("Coins: {}/{}", game_data.coins_collected, game_data.coins_total)),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(1.0, 0.85, 0.0)),
        ));

        let prompt = if is_final {
            "Press ENTER to Play Again"
        } else {
            "Press ENTER for Next Level"
        };
        parent.spawn((
            Text::new(prompt),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(0.4, 1.0, 0.4)),
        ));
    });
}

pub fn level_complete_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        if game_data.current_level >= TOTAL_LEVELS - 1 {
            game_data.current_level = 0;
            game_data.score = 0;
            next_state.set(GameState::Menu);
        } else {
            game_data.current_level += 1;
            next_state.set(GameState::Playing);
        }
    }
}

// ── Game Over Screen ──

pub fn spawn_game_over_screen(
    mut commands: Commands,
    game_data: Res<GameData>,
) {
    commands.spawn((
        MenuTag,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(16.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Game Over"),
            TextFont { font_size: 56.0, ..default() },
            TextColor(Color::srgb(1.0, 0.2, 0.2)),
        ));

        parent.spawn((
            Text::new(format!("Final Score: {}", game_data.score)),
            TextFont { font_size: 28.0, ..default() },
            TextColor(Color::WHITE),
        ));

        parent.spawn((
            Text::new("Press ENTER to Try Again"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgb(0.4, 1.0, 0.4)),
        ));
    });
}

pub fn game_over_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        game_data.score = 0;
        game_data.current_level = 0;
        game_data.lives = 3;
        next_state.set(GameState::Playing);
    }
}
