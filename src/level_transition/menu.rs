use bevy::prelude::*;

use crate::{
    utils::{
        data_structures::Index,
        menu_system::{MenuStack, MultiChoiceButton, MultiChoiceParent},
    },
    AppState,
};

pub struct LevelTransitionMenuPlugin;

impl Plugin for LevelTransitionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::LevelTransition),
            (spawn_level_transition_menu, register_menu).chain(),
        );
    }
}

#[derive(Component)]
struct UpgradeSelectMenu;

const MENU_ITEMS: [i32; 3] = [1, 2, 3];

fn spawn_level_transition_menu(world: &mut World) {
    let activate_id = world.register_system(activate);
    let deactivate_id = world.register_system(deactivate);
    world
        .spawn((
            NodeBundle {
                style: Style {
                    // fill the entire window
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Name::new("UpgradeSelectMenu"),
        ))
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
                    MultiChoiceParent {
                        selected: Index::new(MENU_ITEMS.len(), 0),
                    },
                    UpgradeSelectMenu,
                    Name::new("UpgradeSelectMenu"),
                ))
                .with_children(|builder| {
                    for i in [1, 2, 3] {
                        builder.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    margin: UiRect::all(Val::Percent(3.)),
                                    border: UiRect::all(Val::Percent(1.)),
                                    ..Default::default()
                                },
                                background_color: Color::GRAY.into(),
                                border_color: Color::WHITE.into(),
                                ..Default::default()
                            },
                            MultiChoiceButton {
                                on_selected: None,
                                activate: activate_id,
                                deactivate: deactivate_id,
                            },
                            Name::new(format!("Upgrade {i}")),
                        ));
                    }
                });
        });
}

fn register_menu(
    menu_query: Query<Entity, (With<MultiChoiceParent>, With<UpgradeSelectMenu>)>,
    mut menu_stack: ResMut<MenuStack>,
) {
    menu_stack.push_menu(menu_query.get_single().unwrap());
}

pub fn deactivate(In(entity): In<Entity>, mut border_query: Query<&mut BorderColor>) {
    border_query.get_mut(entity).unwrap().0 = Color::DARK_GRAY;
}

pub fn activate(In(entity): In<Entity>, mut border_query: Query<&mut BorderColor>) {
    border_query.get_mut(entity).unwrap().0 = Color::WHITE;
}