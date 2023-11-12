use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{utils::data_structures::Index, AppState};

pub struct MenuUI;

impl Plugin for MenuUI {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuItemSelected>()
            .add_systems(Startup, spawn_menu_ui)
            .add_systems(
                Update,
                (select_action, activate_action, spawn_attack_menu)
                    .run_if(in_state(AppState::ActionMenu)),
            )
            .add_systems(OnEnter(AppState::ActionMenu), show_menu)
            .add_systems(OnExit(AppState::ActionMenu), hide_menu);
    }
}

type MenuCallback = fn(String);

#[derive(Component)]
pub struct MultiChoiceParent {
    pub selected: Index,
}

#[derive(Component)]
pub struct MultiChoiceButton {
    button: Buttons,
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

#[derive(Event)]
pub struct MenuItemSelected {
    button: Buttons,
}

const MENU_ITEMS: [Buttons; 4] = [
    (Buttons::ATTACK),
    (Buttons::ACT),
    (Buttons::ITEMS),
    (Buttons::RUN),
];

fn spawn_menu_ui(mut commands: Commands) {
    commands
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
                        MultiChoiceButton { button: *button },
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

fn deactivate(button: &mut BorderColor) {
    (*button).0 = Color::BLACK;
}

fn activate(button: &mut BorderColor) {
    (*button).0 = Color::GREEN;
}

fn get_selected_child(multi_choice_parent: &Mut<MultiChoiceParent>, children: &Children) -> Entity {
    // The unwrap is safe if and only if multi_choice_parent was defined to handle the given children.
    return children
        .get(multi_choice_parent.selected.index)
        .unwrap()
        .clone(); // TODO: Make sure cloning is safe; I'm pretty sure it is, as it also derives Copy
}

fn change_selection(
    multi_choice_parent: &mut Mut<MultiChoiceParent>,
    buttons: &mut Query<&mut BorderColor, With<MultiChoiceButton>>,
    children: &Children,
    change_amount: i8,
) {
    deactivate(
        &mut buttons
            .get_mut(get_selected_child(multi_choice_parent, children))
            .unwrap(),
    );

    multi_choice_parent.selected.add(change_amount);

    activate(
        &mut buttons
            .get_mut(get_selected_child(multi_choice_parent, children))
            .unwrap(),
    );
}

fn select_action(
    mut parent: Query<(Entity, &Children, &mut MultiChoiceParent)>,
    mut buttons_query: Query<&mut BorderColor, With<MultiChoiceButton>>,
    input: Res<Input<KeyCode>>,
) {
    let (_, children, mut multi_choice_parent) = parent.get_single_mut().unwrap();
    if input.just_pressed(KeyCode::Right) {
        change_selection(&mut multi_choice_parent, &mut buttons_query, &children, 1);
    } else if input.just_pressed(KeyCode::Left) {
        change_selection(&mut multi_choice_parent, &mut buttons_query, &children, -1);
    }
}

fn activate_action(
    mut parent: Query<(&Children, &mut MultiChoiceParent)>,
    buttons_query: Query<&MultiChoiceButton>,
    input: Res<Input<KeyCode>>,
    mut action_occurred_writer: EventWriter<MenuItemSelected>,
) {
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::Return) {
        let (children, multi_choice) = parent.single_mut();
        // TODO: This would have been simpler if I could just say multi_choice_parent.get_selected_child()
        // Maybe a helper function would do that? maybe a custom query?
        let selected = buttons_query
            .get(get_selected_child(&multi_choice, children))
            .unwrap();
        action_occurred_writer.send(MenuItemSelected {
            button: selected.button,
        });
    }
}

fn spawn_attack_menu(
    mut action_occurred_reader: EventReader<MenuItemSelected>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for event in action_occurred_reader.iter() {
        if event.button == Buttons::ATTACK {
            app_state.set(AppState::Defending);
        }
    }
}

fn hide_menu(mut menu_query: Query<&mut Visibility, With<MultiChoiceParent>>) {
    let mut menu_visibility = menu_query.single_mut();
    *menu_visibility = Visibility::Hidden;
}

fn show_menu(mut menu_query: Query<&mut Visibility, With<MultiChoiceParent>>) {
    let mut menu_visibility = menu_query.single_mut();
    *menu_visibility = Visibility::Visible;
}
