use bevy::prelude::*;

use crate::{
    utils::{
        data_structures::Index,
        menu_system::{MultiChoiceButton, MultiChoiceParent},
    },
    AppState,
};

pub struct LevelTransitionMenuPlugin;

impl Plugin for LevelTransitionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::LevelTransition),
            spawn_level_transition_menu,
        );
    }
}

const MENU_ITEMS: [i32; 3] = [1, 2, 3];

fn spawn_level_transition_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                // fill the entire window
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            // spawn the key
            builder
                .spawn((
                    NodeBundle {
                        style: Style {
                            margin: UiRect::all(Val::Percent(5.)),
                            width: Val::Percent(100.),
                            height: Val::Auto,
                            flex_direction: FlexDirection::Row,
                            display: Display::Flex,
                            align_content: AlignContent::SpaceEvenly,

                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::BLACK),
                        ..Default::default()
                    },
                    // MultiChoiceParent {
                    //     selected: Index::new(MENU_ITEMS.len(), 0),
                    // },
                    Name::new("UpgradeSelectMenu"),
                ))
                .with_children(|builder| {
                    for i in [1, 2, 3] {
                        builder.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    margin: UiRect::all(Val::Percent(3.)),
                                    ..Default::default()
                                },
                                background_color: Color::GRAY.into(),
                                border_color: Color::WHITE.into(),
                                ..Default::default()
                            },
                            // MultiChoiceButton { button: i },
                            Name::new(format!("Upgrade {i}")),
                        ));
                    }
                });
        });
}
