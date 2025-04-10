use bevy::{
    ecs::system::{Commands, Res},
    ui::{Node, PositionType, Val, widget::Text},
};

use crate::components::Score;

pub fn setup_score(mut commands: Commands, score_resource: Res<Score>) {
    let score = score_resource.into_inner();

    commands.spawn((
        Text::new(format!("Score: {}", score.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            right: Val::Px(12.),
            ..Default::default()
        },
    ));
}
