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


    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 11, 11, None, None);
    let texutre_atlas_layout = texture_atlases.add(layout.clone());


    let window = windows_query.get_single().unwrap();
    let (win_w, win_h) = (window.resolution.width(), window.resolution.height());

    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    let game_texture = GameTextures {
        player: asset_server.load(EGG_SPRITE),
        enemy: asset_server.load(SPERM),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        coin: asset_server.load(COIN_SPRITE),
        skill: asset_server.load(SKILL_SPRITE),
        floor: asset_server.load(FLOOR_SPRITE),
        wall: asset_server.load(WALL_SPRITE),
        pixel: asset_server.load("pixel_debug.png"),
        map_texture: asset_server.load("map_edit.png"),
    };
    commands.insert_resource(game_texture);

    let game_atlases = GameAtlases{
        map_texture: texutre_atlas_layout,
    };
    commands.insert_resource(game_atlases);


    commands.insert_resource(PlayerState::default());

    let game_state = GameState { zoom: 1., coins: 0 };
    commands.insert_resource(game_state);

    commands.insert_resource(EnemyCount { alive: 0, dead: 0 });

    commands.insert_resource(PlayerSkill {
        timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
    });
    commands.insert_resource(AtomaticPlayerSkillList(Vec::new()));

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
