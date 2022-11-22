use std::collections::HashMap;

use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use gdnative::prelude::*;

use crate::nodes::node::GDNode;
use crate::node_tree::TrueNodeType;
use crate::runner::RunnerNode;

/// Maps Godot nodes to entities.
/// This should only be used for removing entities.
#[derive(Resource)]
pub struct NodeMap {
    pub nodes: HashMap<Entity, Ref<gdnative::api::Node>>,
}

/// Only exists if the entity's Godot components have been
/// created but not added to the scene tree.
#[derive(Component)]
pub struct ShouldInsert;

/// This is a generic function that works with all Godot nodes.
pub fn add_nodes<T>(
    mut node_map: ResMut<NodeMap>,
    mut node_query: Query<(Entity, &GDNode, &mut TrueNodeType), Added<GDNode>>,
    mut commands: Commands,
) where
    T: GodotObject<Memory=ManuallyManaged> + Instanciable + SubClass<gdnative::prelude::Node>,
{
    for (node_e, node, mut true_type) in node_query.iter_mut() {
        // If the node is None and a node is requested, add into scene tree
        if true_type.node.is_none() && true_type.class_name == T::class_name() {
            let new_node = T::new();
            let node_ref = new_node.as_ref().upcast::<gdnative::prelude::Node>();
            node_ref.set_name(node.name.clone());
            true_type.node = Some(unsafe { node_ref.assume_shared() });

            // Update map
            node_map
                .nodes
                .insert(node_e, true_type.node.unwrap());

            // Note that this node must be inserted
            commands.entity(node_e).insert(ShouldInsert);
        }
    }
}

pub fn insert_nodes(
    runner_node: Res<RunnerNode>,
    parent_query: Query<(Entity, Option<&Parent>), With<ShouldInsert>>,
    mut true_type_query: Query<&mut TrueNodeType>,
    mut commands: Commands,
) {
    for (node_e, parent) in parent_query.iter() {
        let true_type = true_type_query.get_mut(node_e).unwrap();
        let node_ref = true_type.node.unwrap();
        let runner_ref = unsafe { runner_node.node.assume_safe() };

        // Insert under correct parent
        if let Some(parent) = parent {
            let parent_ref = unsafe {
                true_type_query
                    .get_mut(parent.get())
                    .unwrap()
                    .node
                    .unwrap()
                    .assume_safe()
            };
            parent_ref.add_child(node_ref, true);
        } else {
            runner_ref.add_child(node_ref, true);
        }

        // Remove insertion component
        commands.entity(node_e).remove::<ShouldInsert>();
    }
}

pub fn remove_nodes(mut node_map: ResMut<NodeMap>, removed_nodes: RemovedComponents<GDNode>) {
    for entity in removed_nodes.iter() {
        // Remove node from scene
        if let Some(node) = node_map.nodes.get
        (&entity) {
            unsafe { node.assume_safe() }.queue_free();

            // Remove node reference from map
            node_map.nodes.remove(&entity);
        }
    }
}