use bevy::prelude::*;

use crate::{
    utils::{
        data_structures::Index,
        menu_system::{MenuStack, MultiChoiceButton, MultiChoiceParent},
    },
    AppState,
};

pub struct MenuUI;

impl Plugin for MenuUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu_ui)
            .add_systems(OnEnter(AppState::ActionMenu), show_menu)
            .add_systems(OnExit(AppState::ActionMenu), hide_menu);
    }
}

#[derive(
    Component, Clone, Copy, Event, PartialEq, Eq, strum_macros::EnumIter, strum_macros::Display,
)]
enum Buttons {
    ATTACK,
    ACT,
    ITEMS,
    RUN,
}

const MENU_ITEMS: [Buttons; 4] = [Buttons::ATTACK, Buttons::ACT, Buttons::ITEMS, Buttons::RUN];

fn spawn_menu_ui(world: &mut World) {
    let activate_id = world.register_system(activate);
    let deactivate_id = world.register_system(deactivate);
    let attack_pressed_id = world.register_system(transition_to_defense);
    world
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(30.0),
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    border: UiRect::all(Val::Px(20.)),
                    justify_content: JustifyContent::SpaceAround,
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            MultiChoiceParent {
                selected: Index::new(MENU_ITEMS.len(), 0),
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            for button in MENU_ITEMS.iter() {
                commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                border: UiRect::all(Val::Px(5.)),
                                width: Val::Percent(20.),
                                height: Val::Auto,
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            border_color: if *button == Buttons::ATTACK {
                                Color::GREEN.into()
                            } else {
                                Color::BLACK.into()
                            },
                            ..default()
                        },
                        MultiChoiceButton {
                            on_selected: {
                                if *button == Buttons::ATTACK {
                                    Some(attack_pressed_id)
                                } else {
                                    None
                                }
                            },
                            activate: activate_id,
                            deactivate: deactivate_id,
                        },
                    ))
                    .with_children(|commands| {
                        commands.spawn((TextBundle {
                            text: Text::from_section(
                                button.to_string(),
                                TextStyle {
                                    font_size: 32.0,
                                    ..default()
                                },
                            ),
                            ..default()
                        },));
                    });
            }
        });
}
#[derive(Resource, Default)]
struct Counter(u8);

fn deactivate(In(entity): In<Entity>, mut border_query: Query<&mut BorderColor>) {
    border_query.get_mut(entity).unwrap().0 = Color::BLACK;
}

fn activate(In(entity): In<Entity>, mut border_query: Query<&mut BorderColor>) {
    border_query.get_mut(entity).unwrap().0 = Color::GREEN;
}

fn transition_to_defense(In(_entity): In<Entity>, mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::Defending);
}

fn hide_menu(
    mut menu_query: Query<(Entity, &mut Visibility), With<MultiChoiceParent>>,
    mut menu_stack: ResMut<MenuStack>,
) {
    let (entity, mut menu_visibility) = menu_query.single_mut();
    *menu_visibility = Visibility::Hidden;
    menu_stack.pop_menu(entity);
}

fn show_menu(
    mut menu_query: Query<(Entity, &mut Visibility), With<MultiChoiceParent>>,
    mut menu_stack: ResMut<MenuStack>,
) {
    let (entity, mut menu_visibility) = menu_query.single_mut();
    *menu_visibility = Visibility::Visible;
    menu_stack.push_menu(entity);
}
