use bevy::prelude::*;

use crate::{
    metagame::StartGameEvent,
    utils::{
        data_structures::Index,
        menu_system::{MenuStack, MultiChoiceButton, MultiChoiceParent},
    },
    AppState,
};

use super::palette;

pub struct MenuUI;

impl Plugin for MenuUI {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::ActionMenu),
            (spawn_menu_ui, show_menu).chain(),
        )
        .add_systems(OnExit(AppState::ActionMenu), hide_menu);
    }
}

#[derive(Clone, Copy, Event, PartialEq, Eq, strum_macros::EnumIter, strum_macros::Display)]
enum Buttons {
    Start,
}

const MENU_ITEMS: [Buttons; 1] = [Buttons::Start];

fn spawn_menu_ui(world: &mut World) {
    let activate_id = world.register_system(activate);
    let deactivate_id = world.register_system(deactivate);
    let attack_pressed_id = world.register_system(start_game);
    world
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                left: Val::Px(0.),
                bottom: Val::Px(0.),
                border: UiRect::all(Val::Px(20.)),
                justify_content: JustifyContent::SpaceAround,
                position_type: PositionType::Absolute,
                display: Display::Flex,
                ..default()
            },
            BackgroundColor(palette::DARK_GRAY.into()),
            Visibility::Hidden,
            MultiChoiceParent {
                selected: Index::new(MENU_ITEMS.len(), 0),
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            for button in MENU_ITEMS.iter() {
                commands
                    .spawn((
                        Node {
                            border: UiRect::all(Val::Px(5.)),
                            width: Val::Percent(20.),
                            height: Val::Auto,
                            align_items: AlignItems::Start,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BorderColor(if *button == Buttons::Start {
                            palette::GREEN.into()
                        } else {
                            palette::BLACK.into()
                        }),
                        MultiChoiceButton {
                            on_selected: {
                                if *button == Buttons::Start {
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
                        commands.spawn((
                            Text(button.to_string()),
                            TextFont {
                                font_size: 32.0,
                                ..default()
                            },
                        ));
                    });
            }
        });
}

fn deactivate(In(entity): In<Entity>, mut border_query: Query<&mut BorderColor>) {
    border_query.get_mut(entity).unwrap().0 = palette::BLACK.into();
}

fn activate(In(entity): In<Entity>, mut border_query: Query<&mut BorderColor>) {
    border_query.get_mut(entity).unwrap().0 = palette::GREEN.into();
}

fn start_game(In(_entity): In<Entity>, mut start_game_event: EventWriter<StartGameEvent>) {
    start_game_event.send(StartGameEvent);
}

fn hide_menu(
    mut menu_query: Query<(Entity, &mut Visibility), With<MultiChoiceParent>>,
    mut menu_stack: ResMut<MenuStack>,
) -> Result {
    let (entity, mut menu_visibility) = menu_query.single_mut()?;
    *menu_visibility = Visibility::Hidden;
    menu_stack.pop_menu(entity);

    Ok(())
}

fn show_menu(
    mut menu_query: Query<(Entity, &mut Visibility), With<MultiChoiceParent>>,
    mut menu_stack: ResMut<MenuStack>,
) -> Result {
    let (entity, mut menu_visibility) = menu_query.single_mut()?;
    *menu_visibility = Visibility::Visible;
    menu_stack.push_menu(entity);

    Ok(())
}
