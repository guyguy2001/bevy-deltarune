use bevy::{
    ecs::{
        query::{QueryData, ReadOnlyQueryData},
        system::{SystemId, SystemParam},
    },
    prelude::*,
};

use super::data_structures::Index;

#[derive(Component)]
pub struct MultiChoiceParent {
    pub selected: Index,
}

#[derive(Resource)]
pub struct MultiChoiceStack {
    pub menu_parents_stack: Vec<Entity>,
}

#[derive(Component)]
pub struct MultiChoiceButton {
    pub on_selected: Option<SystemId<Entity, ()>>,
    pub activate: SystemId<Entity, ()>,
    pub deactivate: SystemId<Entity, ()>,
}

#[derive(SystemParam)]
pub struct MenuQueries<'w, 's, Q: QueryData + 'static> {
    menu_stack: Res<'w, MenuStack>,
    q_menu_parent: Query<
        'w,
        's,
        (
            &'static Name,
            &'static Children,
            &'static mut MultiChoiceParent,
        ),
    >,
    q_menu_items: Query<'w, 's, Q>,
}

impl<'w, 's, Q: QueryData + 'static> MenuQueries<'w, 's, Q> {
    fn get_active_menu(&self) -> Option<Entity> {
        self.menu_stack.get_current_menu()
    }
}

impl<'w, 's, Q: ReadOnlyQueryData + 'static> MenuQueries<'w, 's, Q> {
    pub fn get_selected_child(&self, menu_parent_entity: Entity) -> Q::Item<'_> {
        let (name, children_component, multi_choice_parent) =
            self.q_menu_parent.get(menu_parent_entity).unwrap();

        let child_entity = *(children_component
            .get(multi_choice_parent.selected.index)
            .unwrap());

        let error_message = format!("Parent {name} has no child {}", child_entity.index());
        // The unwrap is safe if and only if multi_choice_parent was defined to handle the given children.
        self.q_menu_items.get(child_entity).expect(&error_message)
    }

    pub fn get_all_children(&self, menu_parent_entity: Entity) -> Vec<Q::Item<'_>> {
        let (_, children_component, _) = self.q_menu_parent.get(menu_parent_entity).unwrap();

        let child_entities = children_component.iter();

        // TODO: FilterMap?
        Iterator::map(child_entities, |entity| {
            self.q_menu_items.get(*entity).unwrap()
        })
        .collect()
    }
}

impl<'w, 's, Q: QueryData + 'static> MenuQueries<'w, 's, Q> {
    pub fn get_selected_child_mut(&mut self, menu_parent_entity: Entity) -> Q::Item<'_> {
        let (_, children_component, multi_choice_parent) =
            self.q_menu_parent.get(menu_parent_entity).unwrap();

        // The unwrap is safe if and only if multi_choice_parent was defined to handle the given children.

        self.q_menu_items
            .get_mut(
                *(children_component
                    .get(multi_choice_parent.selected.index)
                    .unwrap()),
            )
            .unwrap()
    }

    pub fn get_multi_choice_parent_mut(
        &mut self,
        menu_parent_entity: Entity,
    ) -> Mut<MultiChoiceParent> {
        let (_, _, multi_choice_parent) = self.q_menu_parent.get_mut(menu_parent_entity).unwrap();
        multi_choice_parent
    }
}

pub struct MenuSystemPlugin;

impl Plugin for MenuSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuStack>()
            .add_event::<SpawnedMenu>()
            .add_systems(Update, (select_action, activate_action, on_menu_spawned));
    }
}

#[derive(Resource, Default)]
pub struct MenuStack {
    menus: Vec<Entity>,
}

impl MenuStack {
    pub fn get_current_menu(&self) -> Option<Entity> {
        self.menus.last().copied()
    }

    pub fn push_menu(&mut self, new_menu: Entity) {
        self.menus.push(new_menu);
    }

    pub fn pop_menu(&mut self, new_menu: Entity) {
        assert_eq!(self.menus.pop().unwrap(), new_menu);
        self.menus.pop();
    }
}

fn change_selection(
    mut commands: Commands,
    mut menu_queries: MenuQueries<(Entity, &MultiChoiceButton)>,
    change_amount: i8,
) {
    if let Some(active_menu) = menu_queries.get_active_menu() {
        let (entity, button) = menu_queries.get_selected_child_mut(active_menu);
        commands.run_system_with_input(button.deactivate, entity);

        menu_queries
            .get_multi_choice_parent_mut(active_menu)
            .selected
            .add(change_amount);

        let (entity, button) = menu_queries.get_selected_child_mut(active_menu);
        commands.run_system_with_input(button.activate, entity);
    }
}

fn select_action(
    commands: Commands,
    component_query: MenuQueries<(Entity, &MultiChoiceButton)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::ArrowRight) {
        change_selection(commands, component_query, 1);
    } else if input.just_pressed(KeyCode::ArrowLeft) {
        change_selection(commands, component_query, -1);
    }
}

fn activate_action(
    menu_queries: MenuQueries<(Entity, &MultiChoiceButton)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::Enter) {
        if let Some(active_menu) = menu_queries.get_active_menu() {
            let (entity, button) = menu_queries.get_selected_child(active_menu);
            if let Some(on_selected) = button.on_selected {
                commands.run_system_with_input(on_selected, entity);
            }
        }
    }
}

#[derive(Event)]
pub struct SpawnedMenu(pub Entity);

fn on_menu_spawned(
    mut event: EventReader<SpawnedMenu>,
    component_query: MenuQueries<(Entity, &MultiChoiceButton)>,
    mut commands: Commands,
) {
    for SpawnedMenu(new_menu) in event.read() {
        let (selected_entity, _) = component_query.get_selected_child(*new_menu);
        for (button_entity, button) in component_query.get_all_children(*new_menu) {
            if button_entity == selected_entity {
                commands.run_system_with_input(button.activate, button_entity)
            } else {
                commands.run_system_with_input(button.deactivate, button_entity)
            }
        }
    }
}
