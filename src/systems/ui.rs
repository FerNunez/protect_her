use crate::prelude::*;

pub fn text_update_system(game_state: Res<GameState>, mut query: Query<&mut Text, With<CoinText>>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}", game_state.coins)
    }
}
