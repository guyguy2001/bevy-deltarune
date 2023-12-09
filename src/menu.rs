use bevy::{ecs::system::SystemParam, prelude::*};

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

fn deactivate(mut button: Mut<BorderColor>) {
    button.0 = Color::BLACK;
}

fn activate(mut button: Mut<BorderColor>) {
    button.0 = Color::GREEN;
}

#[derive(SystemParam)]
struct MultiChoiceItem<'w, 's, T: Component> {
    parent: Query<'w, 's, (&'static Children, &'static mut MultiChoiceParent)>,
    children: Query<'w, 's, &'static mut T, With<MultiChoiceButton>>,
}

impl<'w, 's, T: Component> MultiChoiceItem<'w, 's, T> {
    fn get_selected_child(&self) -> &T {
        let (children_component, multi_choice_parent) = self.parent.get_single().unwrap();

        // The unwrap is safe if and only if multi_choice_parent was defined to handle the given children.
        return self
            .children
            .get(
                *(children_component
                    .get(multi_choice_parent.selected.index)
                    .unwrap()),
            )
            .unwrap();
    }

    fn get_selected_child_mut(&mut self) -> Mut<T> {
        let (children_component, multi_choice_parent) = self.parent.get_single().unwrap();

        // The unwrap is safe if and only if multi_choice_parent was defined to handle the given children.
        return self
            .children
            .get_mut(
                *(children_component
                    .get(multi_choice_parent.selected.index)
                    .unwrap()),
            )
            .unwrap();
    }

    fn get_multi_choice_parent_mut(&mut self) -> Mut<MultiChoiceParent> {
        let (_, multi_choice_parent) = self.parent.get_single_mut().unwrap();
        multi_choice_parent
    }
}

fn change_selection(mut multi_choice_item: MultiChoiceItem<BorderColor>, change_amount: i8) {
    deactivate(multi_choice_item.get_selected_child_mut());

    multi_choice_item
        .get_multi_choice_parent_mut()
        .selected
        .add(change_amount);

    activate(multi_choice_item.get_selected_child_mut());
}

fn select_action(component_query: MultiChoiceItem<BorderColor>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Right) {
        change_selection(component_query, 1);
    } else if input.just_pressed(KeyCode::Left) {
        change_selection(component_query, -1);
    }
}

fn activate_action(
    multi_choice_item: MultiChoiceItem<MultiChoiceButton>,
    input: Res<Input<KeyCode>>,
    mut action_occurred_writer: EventWriter<MenuItemSelected>,
) {
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::Return) {
        action_occurred_writer.send(MenuItemSelected {
            button: multi_choice_item.get_selected_child().button, // TODO: I want the MultiChoiceButton :(
        });
    }
}

fn spawn_attack_menu(
    mut action_occurred_reader: EventReader<MenuItemSelected>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for event in action_occurred_reader.read() {
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
