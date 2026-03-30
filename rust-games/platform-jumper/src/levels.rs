use bevy::prelude::*;
use crate::components::*;

pub struct LevelData {
    pub platforms: Vec<PlatformDef>,
    pub moving_platforms: Vec<MovingPlatformDef>,
    pub coins: Vec<Vec2>,
    pub enemies: Vec<EnemyDef>,
    pub player_start: Vec2,
    pub exit_pos: Vec2,
}

pub struct PlatformDef {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
}

pub struct MovingPlatformDef {
    pub start: Vec2,
    pub end: Vec2,
    pub width: f32,
    pub speed: f32,
}

pub struct EnemyDef {
    pub pos: Vec2,
    pub patrol_left: f32,
    pub patrol_right: f32,
    pub speed: f32,
}

pub const TOTAL_LEVELS: usize = 3;

pub fn get_level(index: usize) -> LevelData {
    match index {
        0 => level_1(),
        1 => level_2(),
        2 => level_3(),
        _ => level_1(),
    }
}

fn level_1() -> LevelData {
    LevelData {
        platforms: vec![
            // Ground
            PlatformDef { pos: Vec2::new(0.0, -200.0), width: 600.0, height: 32.0 },
            PlatformDef { pos: Vec2::new(700.0, -200.0), width: 400.0, height: 32.0 },
            PlatformDef { pos: Vec2::new(1300.0, -200.0), width: 500.0, height: 32.0 },
            // Stepping platforms
            PlatformDef { pos: Vec2::new(200.0, -100.0), width: 120.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(450.0, -20.0), width: 120.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(700.0, 40.0), width: 150.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(950.0, -60.0), width: 100.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(1150.0, 20.0), width: 120.0, height: 20.0 },
            // Upper platforms
            PlatformDef { pos: Vec2::new(350.0, 100.0), width: 100.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(600.0, 160.0), width: 100.0, height: 20.0 },
        ],
        moving_platforms: vec![
            MovingPlatformDef {
                start: Vec2::new(500.0, -200.0),
                end: Vec2::new(500.0, -80.0),
                width: 100.0,
                speed: 0.8,
            },
        ],
        coins: vec![
            Vec2::new(200.0, -60.0),
            Vec2::new(450.0, 30.0),
            Vec2::new(700.0, 90.0),
            Vec2::new(950.0, -10.0),
            Vec2::new(350.0, 150.0),
            Vec2::new(600.0, 210.0),
            Vec2::new(1150.0, 70.0),
            Vec2::new(1400.0, -150.0),
        ],
        enemies: vec![
            EnemyDef { pos: Vec2::new(700.0, -168.0), patrol_left: 500.0, patrol_right: 900.0, speed: 80.0 },
            EnemyDef { pos: Vec2::new(1300.0, -168.0), patrol_left: 1100.0, patrol_right: 1500.0, speed: 100.0 },
        ],
        player_start: Vec2::new(-200.0, -140.0),
        exit_pos: Vec2::new(1500.0, -155.0),
    }
}

fn level_2() -> LevelData {
    LevelData {
        platforms: vec![
            // Ground segments with gaps
            PlatformDef { pos: Vec2::new(-100.0, -200.0), width: 300.0, height: 32.0 },
            PlatformDef { pos: Vec2::new(400.0, -200.0), width: 200.0, height: 32.0 },
            PlatformDef { pos: Vec2::new(800.0, -200.0), width: 200.0, height: 32.0 },
            PlatformDef { pos: Vec2::new(1200.0, -200.0), width: 300.0, height: 32.0 },
            PlatformDef { pos: Vec2::new(1700.0, -200.0), width: 400.0, height: 32.0 },
            // Floating platforms
            PlatformDef { pos: Vec2::new(250.0, -100.0), width: 80.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(550.0, -50.0), width: 80.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(650.0, 50.0), width: 80.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(900.0, -80.0), width: 100.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(1050.0, 0.0), width: 100.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(1350.0, -80.0), width: 100.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(1500.0, 30.0), width: 80.0, height: 20.0 },
            // High platforms
            PlatformDef { pos: Vec2::new(400.0, 120.0), width: 100.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(700.0, 180.0), width: 120.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(1100.0, 150.0), width: 100.0, height: 20.0 },
        ],
        moving_platforms: vec![
            MovingPlatformDef {
                start: Vec2::new(200.0, -120.0),
                end: Vec2::new(400.0, -120.0),
                width: 80.0,
                speed: 1.0,
            },
            MovingPlatformDef {
                start: Vec2::new(1000.0, -200.0),
                end: Vec2::new(1000.0, -50.0),
                width: 90.0,
                speed: 0.6,
            },
            MovingPlatformDef {
                start: Vec2::new(1500.0, -150.0),
                end: Vec2::new(1700.0, -150.0),
                width: 80.0,
                speed: 1.2,
            },
        ],
        coins: vec![
            Vec2::new(250.0, -50.0),
            Vec2::new(550.0, 0.0),
            Vec2::new(650.0, 100.0),
            Vec2::new(900.0, -30.0),
            Vec2::new(1050.0, 50.0),
            Vec2::new(400.0, 170.0),
            Vec2::new(700.0, 230.0),
            Vec2::new(1100.0, 200.0),
            Vec2::new(1500.0, 80.0),
            Vec2::new(1800.0, -150.0),
        ],
        enemies: vec![
            EnemyDef { pos: Vec2::new(400.0, -168.0), patrol_left: 300.0, patrol_right: 500.0, speed: 90.0 },
            EnemyDef { pos: Vec2::new(800.0, -168.0), patrol_left: 700.0, patrol_right: 900.0, speed: 110.0 },
            EnemyDef { pos: Vec2::new(1300.0, -168.0), patrol_left: 1100.0, patrol_right: 1400.0, speed: 100.0 },
            EnemyDef { pos: Vec2::new(1700.0, -168.0), patrol_left: 1550.0, patrol_right: 1850.0, speed: 120.0 },
        ],
        player_start: Vec2::new(-100.0, -140.0),
        exit_pos: Vec2::new(1850.0, -155.0),
    }
}

fn level_3() -> LevelData {
    LevelData {
        platforms: vec![
            // Sparse ground
            PlatformDef { pos: Vec2::new(-100.0, -200.0), width: 200.0, height: 32.0 },
            PlatformDef { pos: Vec2::new(600.0, -200.0), width: 150.0, height: 32.0 },
            PlatformDef { pos: Vec2::new(1400.0, -200.0), width: 150.0, height: 32.0 },
            PlatformDef { pos: Vec2::new(2000.0, -200.0), width: 300.0, height: 32.0 },
            // Staircase up
            PlatformDef { pos: Vec2::new(100.0, -120.0), width: 80.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(250.0, -40.0), width: 80.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(400.0, 40.0), width: 80.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(550.0, 120.0), width: 80.0, height: 20.0 },
            // Mid-air gauntlet
            PlatformDef { pos: Vec2::new(750.0, 80.0), width: 60.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(900.0, 40.0), width: 60.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(1050.0, 100.0), width: 60.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(1200.0, 60.0), width: 60.0, height: 20.0 },
            // Descent
            PlatformDef { pos: Vec2::new(1350.0, 0.0), width: 80.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(1500.0, -60.0), width: 80.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(1650.0, -120.0), width: 80.0, height: 20.0 },
            // Final stretch
            PlatformDef { pos: Vec2::new(1800.0, -60.0), width: 100.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(1950.0, 20.0), width: 80.0, height: 20.0 },
            // High bonus area
            PlatformDef { pos: Vec2::new(300.0, 200.0), width: 100.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(500.0, 260.0), width: 80.0, height: 20.0 },
            PlatformDef { pos: Vec2::new(700.0, 220.0), width: 80.0, height: 20.0 },
        ],
        moving_platforms: vec![
            MovingPlatformDef {
                start: Vec2::new(150.0, -200.0),
                end: Vec2::new(350.0, -200.0),
                width: 70.0,
                speed: 1.5,
            },
            MovingPlatformDef {
                start: Vec2::new(850.0, -100.0),
                end: Vec2::new(850.0, 60.0),
                width: 70.0,
                speed: 0.8,
            },
            MovingPlatformDef {
                start: Vec2::new(1100.0, -200.0),
                end: Vec2::new(1300.0, -200.0),
                width: 70.0,
                speed: 1.3,
            },
            MovingPlatformDef {
                start: Vec2::new(1700.0, -180.0),
                end: Vec2::new(1700.0, -40.0),
                width: 70.0,
                speed: 1.0,
            },
        ],
        coins: vec![
            Vec2::new(100.0, -70.0),
            Vec2::new(250.0, 10.0),
            Vec2::new(400.0, 90.0),
            Vec2::new(550.0, 170.0),
            Vec2::new(750.0, 130.0),
            Vec2::new(900.0, 90.0),
            Vec2::new(1050.0, 150.0),
            Vec2::new(1200.0, 110.0),
            Vec2::new(300.0, 250.0),
            Vec2::new(500.0, 310.0),
            Vec2::new(700.0, 270.0),
            Vec2::new(2100.0, -150.0),
        ],
        enemies: vec![
            EnemyDef { pos: Vec2::new(600.0, -168.0), patrol_left: 525.0, patrol_right: 675.0, speed: 100.0 },
            EnemyDef { pos: Vec2::new(1400.0, -168.0), patrol_left: 1325.0, patrol_right: 1475.0, speed: 120.0 },
            EnemyDef { pos: Vec2::new(2000.0, -168.0), patrol_left: 1870.0, patrol_right: 2130.0, speed: 130.0 },
            EnemyDef { pos: Vec2::new(2100.0, -168.0), patrol_left: 1950.0, patrol_right: 2150.0, speed: 90.0 },
        ],
        player_start: Vec2::new(-100.0, -140.0),
        exit_pos: Vec2::new(2150.0, -155.0),
    }
}
