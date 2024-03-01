use bevy::{ecs::query::QueryData, prelude::*};

use crate::{
    utils::{
        data_structures::Index,
        menu_system::{MultiChoiceButton, MultiChoiceItem, MultiChoiceParent},
    },
    AppState,
};

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
            .add_systems(
                Update,
                select_action,
            )
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

fn spawn_menu_ui(mut world: &mut World) {
    let activate_id = world.register_system(activate);
    let deactivate_id = world.register_system(deactivate);
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
                        MultiChoiceButton::<Buttons> {
                            button: *button,
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

fn deactivate(In(entity): In<Entity>, mut border_query: Query<&mut BorderColor>){
    border_query.get_mut(entity).unwrap().0 = Color::BLACK;
}


fn activate(In(entity): In<Entity>, mut border_query: Query<&mut BorderColor>){
    border_query.get_mut(entity).unwrap().0 = Color::GREEN;
}

fn change_selection(
    mut commands: Commands,
    mut multi_choice_item: MultiChoiceItem<(Entity, &MultiChoiceButton<Buttons>)>,
    change_amount: i8,
) {
    let (entity, button) = multi_choice_item.get_selected_child_mut();
    commands.run_system_with_input(button.deactivate, entity);

    multi_choice_item
        .get_multi_choice_parent_mut()
        .selected
        .add(change_amount);

    let (entity, button) = multi_choice_item.get_selected_child_mut();
    commands.run_system_with_input(button.activate, entity);
}

fn select_action(
    mut commands: Commands,
    component_query: MultiChoiceItem<(
        // &mut BorderColor,
        Entity,
        &MultiChoiceButton<Buttons>,
    )>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::ArrowRight) {
        change_selection(commands, component_query, 1);
    } else if input.just_pressed(KeyCode::ArrowLeft) {
        change_selection(commands, component_query, -1);
    }
}

fn activate_action(
    multi_choice_item: MultiChoiceItem<&MultiChoiceButton<Buttons>>,
    input: Res<ButtonInput<KeyCode>>,
    mut action_occurred_writer: EventWriter<MenuItemSelected>,
) {
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::Enter) {
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
