use crate::{
    GameState,
    components::{CollisionEvent, Score},
    systems::{collision::*, movement::*, scoring::*, world::*},
};
use bevy::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.add_event::<CollisionEvent>()
        .insert_resource(Score(0))
        .add_systems(
            OnEnter(GameState::Game),
            (setup_game, setup_scene, setup_instructions, setup_score).chain(),
        )
        .add_systems(
            Update,
            (
                move_player,
                move_ball,
                move_enemy,
                check_collisions,
                check_wall_collision,
                check_for_score,
                update_scoreboard,
            )
                .run_if(in_state(GameState::Game)),
        );
}
