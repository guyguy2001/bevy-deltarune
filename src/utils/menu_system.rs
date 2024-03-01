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
pub struct MultiChoiceButton<Data: Sync + Send + 'static> {
    pub button: Data,
    pub activate: SystemId<Entity, ()>,
    pub deactivate: SystemId<Entity, ()>,
}

#[derive(SystemParam)]
pub struct MultiChoiceItem<'w, 's, Q: QueryData + 'static /* Data: Sync + Send + 'static*/> {
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
    // children: Query<'w, 's, &'static mut Q, With<MultiChoiceButton<Q>>>,
}

impl<'w, 's, Q: ReadOnlyQueryData + 'static> MultiChoiceItem<'w, 's, Q> {
    pub fn get_selected_child(&self) -> Q::Item<'_> {
        let (name, children_component, multi_choice_parent) = self.parent.get_single().unwrap();

        let child_entity = *(children_component
            .get(multi_choice_parent.selected.index)
            .unwrap());

        let error_message = format!("Parent {name} has no child {}", child_entity.index());
        // The unwrap is safe if and only if multi_choice_parent was defined to handle the given children.
        let x = self.children.get(child_entity).expect(&error_message);
        return x;
    }
}

impl<'w, 's, Q: QueryData + 'static> MultiChoiceItem<'w, 's, Q> {
    pub fn get_selected_child_mut(&mut self) -> Q::Item<'_> {
        let (name, children_component, multi_choice_parent) = self.parent.get_single().unwrap();
        print!("{name}");

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

    pub fn get_multi_choice_parent_mut(&mut self) -> Mut<MultiChoiceParent> {
        let (_, _, multi_choice_parent) = self.parent.get_single_mut().unwrap();
        multi_choice_parent
    }
}
