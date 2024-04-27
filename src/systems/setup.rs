use bevy::utils::HashMap;

use crate::prelude::*;

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    //commands.spawn(Camera2dBundle::default());
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            (MAP_SIZE_IN_TILES.0 * TILE_SIZE.0 / 2) as f32,
            (MAP_SIZE_IN_TILES.1 * TILE_SIZE.1 / 2) as f32,
            3.0,
        ),
        ..Default::default()
    });
    let map_atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        Vec2::new(32.0, 32.0),
        11,
        11,
        None,
        None,
    ));
    let enemy_tail_animation_atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        Vec2::new(19.0, 7.0),
        7,
        1,
        None,
        None,
    ));
    let player_animation_atlas_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        Vec2::new(32.0, 32.0),
        6,
        6,
        None,
        None,
    ));

    let window = windows_query.get_single().unwrap();
    let (win_w, win_h) = (window.resolution.width(), window.resolution.height());

    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    let game_texture = GameTextures {
        player: asset_server.load(EGG_SPRITE),
        enemy: asset_server.load(SPERM),
        enemy_tail: asset_server.load("tail_19_7.png"),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        coin: asset_server.load(COIN_SPRITE),
        skill: asset_server.load(SKILL_SPRITE),
        floor: asset_server.load(FLOOR_SPRITE),
        wall: asset_server.load(WALL_SPRITE),
        pixel: asset_server.load("pixel_debug.png"),
        map_texture: asset_server.load("map_edit.png"),
        enemy_tail_animation: asset_server.load("enemy_tail_animation.png"),
        player_animation: asset_server.load("player_animation.png"),
        egg_tentacles: asset_server.load("egg_tentacles_only.png"),
        egg_blue: asset_server.load("egg_blue_only.png"),
        egg_red: asset_server.load("egg_red_only.png"),
        egg_yellow: asset_server.load("egg_yellow_only.png"),
        egg_eye: asset_server.load("egg_eye_only.png"),
    };
    commands.insert_resource(game_texture);

    let game_atlases = GameAtlaseLayouts {
        map: map_atlas_layout,
        enemy_tail_animation: enemy_tail_animation_atlas_layout,
        player_animation: player_animation_atlas_layout,
    };
    commands.insert_resource(game_atlases);

    commands.insert_resource(PlayerState::default());
    commands.insert_resource(EggState::default());

    let game_state = GameState {
        //zoom: 0.65,
        zoom: 1.,
        coins: 0,
    };
    commands.insert_resource(game_state);

    commands.insert_resource(EnemyCount { alive: 0, dead: 0 });

    commands.insert_resource(PlayerSkill {
        timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
    });
    commands.insert_resource(AtomaticPlayerSkillList(Vec::new()));

    commands.insert_resource(WaveLevel(0));
    commands.insert_resource(LastMouse { pos: Vec2::ZERO });

    //let mut player_animations = HashMap::new();
    //player_animations.insert(
    //    AnimationState::MovingRight,
    //    Animation::new_from_millis(0, 3, 200),
    //);
    //pzayer_animations.insert(AnimationState::MovingRight, Animation::new_from_millis());
    //if velocity.x == 0. {
    //    if velocity.y < 0. {
    //        info!("vel y> 0");
    //        new_animation = Some(Animation::new_from_millis(0, 3, 200));
    //    } else if velocity.y > 0. {
    //        info!("vel y < 0");
    //        new_animation = Some(Animation::new_from_millis(6, 9, 200));
    //    } else {
    //        info!("vel y = 0");
    //        new_animation = Some(Animation::new(0, 1, 400));
    //    }
    //} else if velocity.x > 0. {
    //    info!("vel x > 0");
    //    let mut animation = Animation::new(12, 15, 200);
    //    animation.set_flip(true);
    //    new_animation = Some(animation);
    //} else {
    //    info!("vel x < 0");
    //    new_animation = Some(Animation::new(12, 15, 200));
    //};
    //
    //    commands.spawn((
    //        TextBundle::from_sections([
    //            TextSection::new(
    //                "Coins ",
    //                TextStyle {
    //                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                    font_size: 50.,
    //                    ..default()
    //                },
    //            ),
    //            TextSection::from_style(TextStyle {
    //                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                font_size: 50.,
    //                color: Color::GOLD,
    //                ..default()
    //            }),
    //        ])
    //        .with_text_alignment(TextAlignment::Center)
    //        .with_style(Style {
    //            position_type: PositionType::Absolute,
    //            bottom: Val::Px(5.0),
    //            right: Val::Px(5.0),
    //            ..default()
    //        }),
    //        CoinText,
    //    ));
    //
    let map_builder = MapBuilder::new();
    commands.insert_resource(map_builder.map);
}
