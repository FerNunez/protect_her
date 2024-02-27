use crate::prelude::*;

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());

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
    };
    commands.insert_resource(game_texture);

    commands.insert_resource(PlayerState::default());

    let game_state = GameState { zoom: 1., coins: 0 };
    commands.insert_resource(game_state);

    commands.insert_resource(EnemyCount { alive: 0, dead: 0 });

    commands.insert_resource(PlayerSkill {
        timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
    });
    commands.insert_resource(AtomaticPlayerSkillList(Vec::new()));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Coins ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 50.,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 50.,
                color: Color::GOLD,
                ..default()
            }),
        ])
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        CoinText,
    ));
}
