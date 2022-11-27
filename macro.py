from pathlib import Path
from json import load
import os
import shutil

NODE_FOLDER = "src/nodes/"
NODE_MOD_FILE = NODE_FOLDER + "mod.rs"
NAME_PASCAL = "NAME_PASCAL"
NAME_SNAKE = "NAME_SNAKE"
PARENT_NAME_PASCAL = "PARENT_NAME_PASCAL"
GDNODE_IMPORT = "GDNODE_IMPORT"
PARENT_COMPONENTS = "PARENT_COMPONENTS"
PARENT_COMPONENT_DEFAULTS = "PARENT_COMPONENT_DEFAULTS"
ADD_STAGE_SYSTEM_TEMPLATE = ".add_system_to_stage(GodotStages::Add, add_nodes::<NAME_PASCAL>)"
ADD_STAGE_SYSTEM = "ADD_STAGE_SYSTEM"
PROP_DECLS = "PROP_DECLS"
PROP_INITS = "PROP_INITS"
PROP_SETTERS = "PROP_SETTERS"
PROP_GETTERS = "PROP_GETTERS"
PROP_DEFAULTS = "PROP_DEFAULTS"
COMPONENT_SRC_TEMPLATE = """use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_app::prelude::*;
use gdnative::prelude::*;
use gdnative::api::*;
use super::*;

use crate::sync::{BevyOwned, GodotOwned};
use crate::{node_tree::{TrueNodeType, NodeClass}, runner::{GodotStages}};
use crate::node_tree::WorldCommands;
use crate::node_tree::GDNullClass;

GDNODE_IMPORT
use crate::node::add_nodes;

pub struct NAME_PASCALPlugin;

impl Plugin for NAME_PASCALPlugin {
    fn build(&self, app: &mut App) {
        app ADD_STAGE_SYSTEM
            .add_system_to_stage(GodotStages::Sync, sync_bevy_owned)
            .add_system_to_stage(GodotStages::Sync, sync_godot_owned);
    }
}

/// Returns true if the node can be casted to a NAME_SNAKE.
pub fn is_NAME_SNAKE(node: &gdnative::prelude::Node) -> bool {
    node.cast::<NAME_PASCAL>().is_some()
}

/// A bundle for NAME_PASCALs.
/// Contains all parent components as well.
#[derive(Bundle)]
pub struct GDNAME_PASCALBundle {
    pub NAME_SNAKE: GDNAME_PASCAL,
    PARENT_COMPONENTS
    pub true_type: TrueNodeType,
}

impl Default for GDNAME_PASCALBundle {
    fn default() -> Self {
        Self {
            NAME_SNAKE: Default::default(),
            PARENT_COMPONENT_DEFAULTS
            true_type: TrueNodeType {
                node: None,
                class_name: "NAME_PASCAL".to_string()
            }
        }
    }
}

/// Represents a NAME_PASCAL.
#[derive(Component)]
pub struct GDNAME_PASCAL {
    PROP_DECLS
}

impl Default for GDNAME_PASCAL {
    fn default() -> Self {
        Self {
            PROP_DEFAULTS
        }
    }
}

impl NodeClass for GDNAME_PASCAL {
    type Parent = GDPARENT_NAME_PASCAL;
    type GodotClass = NAME_PASCAL;

    fn add_components<T: WorldCommands>(world_commands: &mut T, entity: Entity, node: &gdnative::prelude::Node) {
        let component_ref = node.cast::<NAME_PASCAL>().unwrap();
        world_commands.insert(entity, GDNAME_PASCAL {
            PROP_INITS
        });
        Self::Parent::add_components(world_commands, entity, node);
    }
}

impl GDNAME_PASCAL {
    
}

fn sync_bevy_owned(query: Query<(&GDNAME_PASCAL, &TrueNodeType), With<BevyOwned>>) {
    for (component, true_type) in query.iter() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NAME_PASCAL>().unwrap();
        PROP_SETTERS
    }
}

fn sync_godot_owned(mut query: Query<(&mut GDNAME_PASCAL, &TrueNodeType), With<GodotOwned>>) {
    for (mut component, true_type) in query.iter_mut() {
        let component_ref = unsafe { true_type.node.unwrap().assume_safe() }.cast::<NAME_PASCAL>().unwrap();
        PROP_GETTERS
    }
}
""".strip()

ADD_COMPONENTS_BRANCHES = "ADD_COMPONENTS_BRANCHES"
NODE_COMPONENT_PLUGINS = "NODE_COMPONENT_PLUGINS"
ADD_COMPONENTS_BRANCH_TEMPLATE = """
"NAME_PASCAL" => GDNAME_PASCAL::add_components(world_commands, entity, node),
""".strip()
MOD_TEMPLATE = """
use crate::node_tree::{WorldCommands, NodeClass};
use bevy_app::prelude::*;
pub fn add_components<T: WorldCommands>(world_commands: &mut T, entity: bevy_ecs::prelude::Entity, node: &gdnative::prelude::Node) {
    match node.get_class().to_string().as_str() {
        ADD_COMPONENTS_BRANCHES
        _ => (),
    }
}

pub struct NodeComponentPlugin;

impl Plugin for NodeComponentPlugin {
    fn build(&self, app: &mut App) {
        app NODE_COMPONENT_PLUGINS;
    }
} 
""".strip()

def simple_type(type_name: str, default = None) -> dict:
    return {
        "name": type_name,
        "to_godot": lambda x: x,
        "to_bevy": lambda x: x,
        "default": lambda x: default(x) if default else "Default::default()",
    }

def ref_type(type_name: str, default = None) -> dict:
    return {
        "name": f"Option<Ref<{type_name}>>",
        "to_godot": lambda x: f"{x}.as_ref().unwrap().clone()",
        "to_bevy": lambda x: f"{x}",
        "default": lambda x: default(x) if default else "Default::default()",
    }

def pool_type(type_name: str) -> dict:
    return {
        "name": f"Vec<{type_name}>",
        "to_godot": lambda x: f"{type_name}Array::from_vec({x}.clone())",
        "to_bevy": lambda x: f"{x}.to_vec()",
        "default": lambda _: "Default::default()",
    }

TYPES = {
    "Vector2": simple_type("Vector2"),
    "Vector3": simple_type("Vector3"),
    "AABB": simple_type("Aabb"),
    "int": simple_type("i64"),
    "float": simple_type("f64"),
    "bool": simple_type("bool"),
    "Basis": simple_type("Basis"),
    "Quat": simple_type("Quat"),
    "Color": simple_type("Color", lambda x: f"{x}::from_rgb(0.0, 0.0, 0.0)"),
    "String": {
        "name": "String",
        "to_godot": lambda x: f"{x}.clone()",
        "to_bevy": lambda x: f"{x}.to_string()",
        "default": lambda _: "Default::default()",
    },
    "Transform": simple_type("Transform", lambda x: f"{x}::IDENTITY"),
    "Transform2D": simple_type("Transform2D", lambda x: f"{x}::IDENTITY"),
    "Rect2": simple_type("Rect2", lambda x: f"{x}::from_components(0.0, 0.0, 0.0, 0.0)"),
    "PoolVector3Array": pool_type("Vector3"),
    "PoolVector2Array": pool_type("Vector2"),
    "PoolByteArray": pool_type("Byte"),
    "PoolFloatArray": pool_type("Float"),
    "PoolIntArray": pool_type("Int32"),
    "PoolStringArray": {
        "name": f"Vec<String>",
        "to_godot": lambda x: f"StringArray::from_vec({x}.iter().map(GodotString::from_str).collect())",
        "to_bevy": lambda x: f"{x}.to_vec().iter().map(|x| x.to_string()).collect()",
        "default": lambda _: "Default::default()",
    },
    "PoolColorArray": pool_type("Color"),
    "NodePath": {
        "name": "NodePath",
        "to_godot": lambda x: f"{x}.to_godot_string()",
        "to_bevy": lambda x: f"{x}",
        "default": lambda _: "Default::default()",
    },
    
    "ShaderMaterial,SpatialMaterial": ref_type("Material"),
    "ShaderMaterial,CanvasItemMaterial": ref_type("Material"),
    "ShaderMaterial,ParticlesMaterial": ref_type("Material"),
    "SpatialMaterial,ShaderMaterial": ref_type("Material"),
    "Texture": ref_type("Texture"),
    "Texture2D": ref_type("Texture2D"),
    "Mesh": ref_type("Mesh"),
    "MultiMesh": ref_type("MultiMesh"),
}

# Converts Pascal Case to snake_case.
def pascal_to_snake(input_str: str) -> str:
    output = ""
    for c in input_str:
        if not c.islower():
            output += "_" + c.lower()
        else:
            output += c
    return output[1:]

# Generates a node tree.
def gen_node_tree(api_dict: list, node_name: str) -> list:
    child_nodes = [item for item in api_dict if item["base_class"] == node_name]
    child_tree = {}
    for child_node in child_nodes:
        child_name = child_node["name"]
        child_tree[child_name] = gen_node_tree(api_dict, child_name)
    return child_tree

# Generates a node list.
def gen_node_list(api_dict: list, node_name: str) -> list:
    child_nodes = [item for item in api_dict if item["base_class"] == node_name]
    result_list = [node_name]
    for child_node in child_nodes:
        child_name = child_node["name"]
        result_list += gen_node_list(api_dict, child_name)
    return result_list

# Processes the node, adding it to the mod file and generating a source file.
def process_node(parents: list, node_name: str, node_tree: dict, api_dict: list):
    # Generate component source file
    api_node = [item for item in api_dict if item["name"] == node_name][0]
    node_name_snake = pascal_to_snake(node_name)
    parent_name = parents[-1] if len(parents) > 0 else "NullClass"
    parent_components = []
    parent_component_defaults = []
    for parent_name in parents:
        parent_name_snake = pascal_to_snake(parent_name)
        parent_components.append(f"pub {parent_name_snake}: GD{parent_name},")
        parent_component_defaults.append(f"{parent_name_snake}: Default::default(),")
    prop_decls = []
    prop_inits = []
    prop_setters = []
    prop_getters = []
    prop_defaults = []
    for prop in api_node["properties"]:
        prop_name = prop["name"]
        
        # Don't cache any data that has no setters or getters
        getters = [method for method in api_node["methods"] if method["name"] == prop["getter"]]
        no_getter = len(getters) == 0 or len(getters[0]["arguments"]) > 0 or getters[0]["name"][0] == "_"
        setters = [method for method in api_node["methods"] if method["name"] == prop["setter"]]
        no_setter = len(setters) == 0 or len(setters[0]["arguments"]) > 1 or setters[0]["name"][0] == "_"
        if no_getter or no_setter:
            continue
        prop_getter = getters[0]
        prop_setter = setters[0]

        prop_type = prop_getter["return_type"]
        
        if prop_name == "loop":
            prop_name = "_loop"

        if "/" in prop_name or ":" in prop_type or prop_type not in TYPES:
            continue

        getter = prop_getter["name"]
        if getter[:4] == "get_":
            getter = getter[4:]
        getter = TYPES[prop_type]["to_bevy"](f"component_ref.{getter}()")
        
        setter = prop_setter['name']
        setter = f"{setter}({TYPES[prop_type]['to_godot'](f'component.{prop_name}')})"

        prop_default = TYPES[prop_type]["default"](prop_type)

        type_name = TYPES[prop_type]["name"]
        prop_decls.append(f"pub {prop_name}: {type_name},")
        prop_inits.append(f"{prop_name}: {getter},")
        prop_setters.append(f"component_ref.{setter};")
        prop_getters.append(f"component.{prop_name} = {getter};")
        prop_defaults.append(f"{prop_name}: {prop_default},")
    component_src = (
        COMPONENT_SRC_TEMPLATE
            .replace(ADD_STAGE_SYSTEM, ADD_STAGE_SYSTEM_TEMPLATE if api_node["instanciable"] else "")
            .replace(PARENT_NAME_PASCAL, parent_name)
            .replace(NAME_PASCAL, node_name)
            .replace(NAME_SNAKE, node_name_snake)
            .replace(PROP_DECLS, "\n".join(prop_decls))
            .replace(PROP_INITS, "\n".join(prop_inits))
            .replace(PROP_SETTERS, "\n".join(prop_setters))
            .replace(PROP_GETTERS, "\n".join(prop_getters))
            .replace(PROP_DEFAULTS, "\n".join(prop_defaults))
            .replace(GDNODE_IMPORT, "use super::GDNode;"if node_name != "Node" else "")
            .replace(PARENT_COMPONENTS, "\n".join(parent_components))
            .replace(PARENT_COMPONENT_DEFAULTS, "\n".join(parent_component_defaults))
    )
    with open(NODE_FOLDER + node_name_snake + ".rs", "w") as f:
        f.write(component_src)

    # Add to mod.rs
    with open(NODE_MOD_FILE, "a") as f:
        f.write(f"pub mod {node_name_snake};\n")
        f.write(f"pub use {node_name_snake}::*;\n")

    for child_key, child_value in node_tree.items():
        process_node(parents + [node_name], child_key, child_value, api_dict)

with open("src/api.json") as file:
    api_dict: list = load(file)

    # Find all descendants of Node and generate a tree and list
    node_tree = {"Node": gen_node_tree(api_dict, "Node")}
    node_list = gen_node_list(api_dict, "Node")

    # Process each node of the tree
    shutil.rmtree(NODE_FOLDER)
    os.mkdir(NODE_FOLDER)
    with open(NODE_MOD_FILE, "w") as f:
        f.write("")
    process_node([], "Node", node_tree["Node"], api_dict)

    # Add add components function
    add_components_branches = []
    node_component_plugins = []
    for node_name in node_list:
        add_components_branches.append(ADD_COMPONENTS_BRANCH_TEMPLATE.replace(NAME_PASCAL, node_name))
        node_component_plugins.append(f".add_plugin({node_name}Plugin)")
    add_components_fn = (MOD_TEMPLATE
        .replace(ADD_COMPONENTS_BRANCHES, "\n".join(add_components_branches))
        .replace(NODE_COMPONENT_PLUGINS, "\n".join(node_component_plugins)))
    with open(NODE_MOD_FILE, "a") as f:
        f.write(add_components_fn)

