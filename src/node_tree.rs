

use bevy_ecs::prelude::*;
use gdnative::{
    api::Node,
    prelude::*,
};

use crate::{nodes::*, prelude::NoneOwned};

/// A node in Godot's class hierarchy.
pub trait NodeClass {
    type Parent: NodeClass;
    type GodotClass: GodotObject + SubClass<Node>;

    /// Adds components needed for this node class.
    /// This calls Parent's method as well, if it exists.
    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        Self::Parent::add_components(world_commands, entity, node);       
    }
}

/// An empty NodeClass.
/// Should be treated as a null value.
pub struct GDNullClass;

impl NodeClass for GDNullClass {
    type Parent = GDNullClass;
    // The Godot class here doesn't matter
    type GodotClass = Node;

    fn add_components<T: WorldCommands>(_world_commands: &mut T, _entity: Entity, _node: &gdnative::prelude::Node) {}
}

/// This component records an entity's true Godot node type.
#[derive(Default, Component)]
pub struct TrueNodeType {
    pub node: Option<Ref<Node>>,
    pub class_name: String,
}

/// An abstraction over World and Commands.
pub trait WorldCommands {
    fn insert<T: Component>(&mut self, entity: Entity, component: T);
    fn spawn(&mut self) -> Entity;
}

impl WorldCommands for World {
    fn insert<T: Component>(&mut self, entity: Entity, component: T) {
        self.entity_mut(entity).insert(component);
    }
    fn spawn(&mut self) -> Entity {
        self.spawn_empty().id()
    }
}

impl WorldCommands for Commands<'_, '_> {
    fn insert<T: Component>(&mut self, entity: Entity, component: T) {
        self.entity(entity).insert(component);
    }
    fn spawn(&mut self) -> Entity {
        self.spawn_empty().id()
    }
}

/// Spawns an entity with the correct components and return the ID.
pub fn spawn_entity<T: WorldCommands>(
    world_commands: &mut T,
    node: &Node,
) -> Entity {
    let entity = world_commands.spawn();
    setup_components(entity, world_commands, node);
    entity
}

/// Sets up an entity's components.
pub fn setup_components<T: WorldCommands>(
    entity: Entity,
    world_commands: &mut T,
    node: &Node,
) {
    // Add the true node type
    world_commands.insert(
        entity,
        TrueNodeType {
            node: Some(unsafe { node.assume_shared() }),
            class_name: node.get_class().to_string(),
        },
    );

    // Add all components
    add_components(world_commands, entity, node);

    // Add NoneOwned sync component
    world_commands.insert(entity, NoneOwned);
}
