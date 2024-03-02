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
    parent: Query<
        'w,
        's,
        (
            &'static Name,
            &'static Children,
            &'static mut MultiChoiceParent,
        ),
    >,
    children: Query<'w, 's, Q>,
}

struct ActiveMenu<'w, 's, Q: QueryData + 'static> {
    queries: MenuQueries<'w, 's, Q>,
    menu_entity: Entity,
}

impl<'w, 's, Q: QueryData + 'static> MenuQueries<'w, 's, Q> {
    fn get_active_menu(self) -> Option<ActiveMenu<'w, 's, Q>> {
        let parent_entity = self.menu_stack.get_current_menu()?;
        return Some(ActiveMenu {
            queries: self,
            menu_entity: parent_entity,
        });
    }
}

impl<'w, 's, Q: ReadOnlyQueryData + 'static> ActiveMenu<'w, 's, Q> {
    pub fn get_selected_child(&self) -> Q::Item<'_> {
        let (name, children_component, multi_choice_parent) =
            self.queries.parent.get(self.menu_entity).unwrap();

        let child_entity = *(children_component
            .get(multi_choice_parent.selected.index)
            .unwrap());

        let error_message = format!("Parent {name} has no child {}", child_entity.index());
        // The unwrap is safe if and only if multi_choice_parent was defined to handle the given children.
        self.queries
            .children
            .get(child_entity)
            .expect(&error_message)
    }
}

impl<'w, 's, Q: QueryData + 'static> ActiveMenu<'w, 's, Q> {
    pub fn get_selected_child_mut(&mut self) -> Q::Item<'_> {
        let (name, children_component, multi_choice_parent) =
            self.queries.parent.get(self.menu_entity).unwrap();
        print!("{name}");

        // The unwrap is safe if and only if multi_choice_parent was defined to handle the given children.

        self.queries
            .children
            .get_mut(
                *(children_component
                    .get(multi_choice_parent.selected.index)
                    .unwrap()),
            )
            .unwrap()
    }

    pub fn get_multi_choice_parent_mut(&mut self) -> Mut<MultiChoiceParent> {
        let (_, _, multi_choice_parent) = self
            .queries
            .parent
            .get_mut(self.menu_entity)
            .unwrap();
        multi_choice_parent
    }
}

pub struct MenuSystemPlugin;

impl Plugin for MenuSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuStack>()
            .add_systems(Update, (select_action, activate_action));
    }
}

#[derive(Resource, Default)]
pub struct MenuStack {
    menus: Vec<Entity>,
}

impl MenuStack {
    pub fn get_current_menu(&self) -> Option<Entity> {
        if self.menus.len() > 0 {
            Some(self.menus.get(self.menus.len() - 1).unwrap().clone())
        } else {
            None
        }
    }

    pub fn push_menu(&mut self, new_menu: Entity) {
        self.menus.push(new_menu);
    }

    pub fn pop_menu(&mut self, new_menu: Entity) {
        assert_eq!(self.menus.pop().unwrap(), new_menu);
    }
}

fn change_selection(
    mut commands: Commands,
    menu_queries: MenuQueries<(Entity, &MultiChoiceButton)>,
    change_amount: i8,
) {
    if let Some(mut active_menu_context) = menu_queries.get_active_menu() {
        let (entity, button) = active_menu_context.get_selected_child_mut();
        commands.run_system_with_input(button.deactivate, entity);

        active_menu_context
            .get_multi_choice_parent_mut()
            .selected
            .add(change_amount);

        let (entity, button) = active_menu_context.get_selected_child_mut();
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
            let (entity, button) = active_menu.get_selected_child();
            if let Some(on_selected) = button.on_selected {
                commands.run_system_with_input(on_selected, entity);
            }
        }
    }
}
