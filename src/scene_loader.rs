use bevy_ecs::prelude::*;
use bevy_hierarchy::BuildChildren;
use gdnative::{api::PackedScene, prelude::{Ref, GodotObject}};

use crate::{
    node_tree::{setup_components}, node::{NodeMap, ShouldInsert},
};

/// Sent when a packed scene should be instantiated.
/// The scene will be instantiated onto the given entity.
pub struct InstanceSceneEvent {
    pub scene: Ref<PackedScene>,
    pub root: Entity,
}

/// Instantiates a packed scene.
pub fn instance_scene(
    mut ev_instance_scene: EventReader<InstanceSceneEvent>,
    mut commands: Commands,
    mut node_map: Option<ResMut<NodeMap>>,
) {
    for ev in ev_instance_scene.iter() {
        let instance_ref = unsafe { ev.scene.assume_safe() }.as_ref().instance(0).unwrap();
        let instance = unsafe { instance_ref.assume_safe() };

        // For each node in the hierarchy, spawn a matching entity
        let mut stack = vec![(None, instance.as_ref())];
        while !stack.is_empty() {
            let (p_id, node) = stack.pop().unwrap();

            // Add components to entity
            let n_id = if p_id.is_none() {
                ev.root
            }
            else {
                commands.spawn_empty().id()
            };
            setup_components(n_id, &mut commands, node);

            // If this entity has a parent, add this entity to it
            if let Some(p_id) = p_id {
                commands.entity(p_id).insert_children(0, &[n_id]);
            }

            // Update map
            node_map.as_mut().unwrap().nodes.insert(n_id, unsafe { node.assume_shared() });

            // Push children
            for idx in 0..node.get_child_count() {
                let child = unsafe { node.get_child(idx).unwrap().assume_safe().as_ref() };
                stack.push((Some(n_id), child));
            }
        }

        // Mark root for insertion
        commands.entity(ev.root).insert(ShouldInsert);
    }
}
