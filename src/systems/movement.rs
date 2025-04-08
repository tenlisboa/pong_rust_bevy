use bevy::{ecs::system::ResMut, prelude::{ButtonInput, KeyCode, Query, Res, Time, Transform, Vec2, With}};
use crate::{components::Player, config::GameConfig};

pub fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_config: ResMut<GameConfig>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyJ) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyK) {
        direction.y += 1.;
    } 

    let move_delta = direction.normalize_or_zero() * game_config.player_speed * time.delta_secs();
    player.translation += move_delta.extend(0.);
}
