use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_time::*;
use gdnative::prelude::*;

use crate::{
    input::GDInput,
    node_tree::{spawn_entity},
    resource_loader::GDResLoader,
    scene_loader::{instance_scene, InstanceSceneEvent}, nodes::NodeComponentPlugin, node::{NodeMap, remove_nodes, insert_nodes},
};

/// Stages for interacting with Godot.
#[derive(Clone, StageLabel)]
pub enum GodotStages {
/// Label for node access stage.
/// During this stage, nodes can be accessed,
/// since it runs in a single thread.
Access,

/// Label for add stage.
/// During this stage, Godot node references are created.
Add,

/// Label for insert stage.
/// During this stage, Godot node references are inserted
/// into the scene tree.
Insert,

/// Label for synchronization stage.
/// During this stage, Bevy components are
/// synced with Godot nodes.
Sync,

/// Label for removal stage.
/// This is a special stage only for removing nodes.
Remove,
}

/// Holds a reference to the Runner node.
/// This should only be used from one thread at a time.
#[derive(Resource)]
pub struct RunnerNode {
    pub node: Ref<Node>,
}

/// The runner's ready method.
pub fn ready(_owner: &Node, app: &mut App) {
    // For each node in the hierarchy, spawn a matching entity
    let mut stack = vec![(None, _owner)];
    while !stack.is_empty() {
        let (p_id, node) = stack.pop().unwrap();

        // Spawn entity with components
        let n_id = spawn_entity(&mut app.world, node);

        // If this entity has a parent, add this entity to it
        if let Some(p_id) = p_id {
            app.world.entity_mut(p_id).push_children(&[n_id]);
        }

        // Push children
        for idx in 0..node.get_child_count() {
            let child = unsafe { node.get_child(idx).unwrap().assume_safe().as_ref() };
            stack.push((Some(n_id), child));
        }
    }
}

/// The runner's process method.
pub fn process(_owner: &Node, app: &mut App) {
    app.world.get_resource_mut::<Time>().unwrap().update();
    app.update();
}

/// Acts as Bevy's DefaultPlugins, but for Godot Bevy.
pub struct GodotPlugins;

impl Plugin for GodotPlugins {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            GodotStages::Access,
            SystemStage::single_threaded(),
        )
        .add_stage_after(GodotStages::Access, GodotStages::Add, SystemStage::single_threaded())
        .add_stage_after(GodotStages::Add, GodotStages::Insert, SystemStage::single_threaded())
        .add_stage_after(GodotStages::Insert, GodotStages::Sync, SystemStage::single_threaded())
        .add_stage_after(GodotStages::Sync, GodotStages::Remove, SystemStage::single_threaded())
        .insert_resource(Time::default())
        .add_plugin(NodeComponentPlugin)
        .insert_resource(GDInput(Input::godot_singleton()))
        .insert_resource(GDResLoader(ResourceLoader::godot_singleton()))
        .insert_resource(NodeMap {
            nodes: Default::default(),
        })
        .add_event::<InstanceSceneEvent>()
        .add_system(instance_scene)
        .add_system_to_stage(GodotStages::Insert, insert_nodes)
        .add_system_to_stage(GodotStages::Remove, remove_nodes);
    }
}
