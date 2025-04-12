use bevy::{
    app::{App, Update},
    color::{Color, palettes::css::WHITE},
    ecs::{
        component::Component,
        query::{Changed, With},
        schedule::IntoSystemConfigs,
        system::{Commands, Query, ResMut},
    },
    hierarchy::{BuildChildren, ChildBuild},
    state::{
        app::AppExtStates,
        condition::in_state,
        state::{NextState, OnEnter, OnExit, States},
    },
    text::{TextColor, TextFont},
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Node, UiRect, Val,
        widget::{Button, Text},
    },
};

use crate::{GameState, despawn_screen, utils::color::from_rgb};

#[derive(States, Clone, Debug, Hash, Eq, PartialEq, Default)]
enum MenuState {
    Main,
    #[default]
    Disabled,
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
}

#[derive(Component)]
struct OnMainMenuScreen;

const BUTTON_COLOR: Color = Color::srgb(0.43, 0.40, 0.37);
const HOVER_BUTTON_COLOR: Color = Color::srgb(0.34, 0.33, 0.37);
const BACKGROUND_COLOR: Color = from_rgb(163., 149., 148.);

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(OnEnter(MenuState::Main), setup_main_menu)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(
            Update,
            (button_behavior, menu_actions).run_if(in_state(GameState::Menu)),
        );
}

fn setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn setup_main_menu(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(350.),
        height: Val::Px(95.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_text_font = TextFont {
        font_size: 32.0,
        ..Default::default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(BACKGROUND_COLOR),
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Ping & Pong"),
                        TextFont {
                            font_size: 70.,
                            ..Default::default()
                        },
                        TextColor(WHITE.into()),
                        Node {
                            margin: UiRect::bottom(Val::Px(100.)),
                            ..Default::default()
                        },
                    ));

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(BUTTON_COLOR),
                            MenuButtonAction::Play,
                        ))
                        .with_child((
                            Text::new("Play"),
                            TextColor(WHITE.into()),
                            button_text_font.clone(),
                        ));
                });
        });
}

fn button_behavior(
    mut interactions: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interactions {
        *background_color = match *interaction {
            Interaction::Pressed => HOVER_BUTTON_COLOR.into(),
            Interaction::Hovered => HOVER_BUTTON_COLOR.into(),
            Interaction::None => BUTTON_COLOR.into(),
        }
    }
}

fn menu_actions(
    interactions: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interactions {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
            }
        }
    }
}
