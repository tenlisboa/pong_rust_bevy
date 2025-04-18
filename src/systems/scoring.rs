use bevy::{
    asset::AssetServer,
    audio::AudioPlayer,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Res, ResMut, Single},
    },
    hierarchy::BuildChildren,
    text::TextSpan,
    transform::components::Transform,
    ui::{
        Node, PositionType, Val,
        widget::{Text, TextUiWriter},
    },
};

use crate::{
    components::{Ball, Score, ScoreBoard},
    config::{BALL_DIAMETER, WINDOW_SIZE},
};

fn play_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioPlayer::new(asset_server.load("sounds/hit.ogg")));
}

pub fn setup_score(mut commands: Commands) {
    commands
        .spawn((
            Text::new("Score: "),
            ScoreBoard,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.),
                right: Val::Px(12.),
                ..Default::default()
            },
        ))
        .with_child(TextSpan::default());
}

pub fn update_scoreboard(
    score: Res<Score>,
    scoreboard: Single<Entity, (With<ScoreBoard>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*scoreboard, 1) = score.0.to_string();
}

pub fn check_for_score(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    ball: Single<&Transform, With<Ball>>,
) {
    let ball_transform = ball.into_inner();
    let right = WINDOW_SIZE.x / 2.;
    let x = ball_transform.translation.x;

    let ball_radius = BALL_DIAMETER / 2.;

    let has_hit_righ_wall = x + ball_radius > right;

    if has_hit_righ_wall {
        (**score) += 1;
        play_score(commands, asset_server);
    }
}
