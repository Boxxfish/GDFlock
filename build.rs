use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

use serde::Deserialize;

const NAME_PASCAL: &str = "NAME_PASCAL";
const NAME_SNAKE: &str = "NAME_SNAKE";
const PARENT_NAME_PASCAL: &str = "PARENT_NAME_PASCAL";
const GDNODE_IMPORT: &str = "GDNODE_IMPORT";
const PARENT_COMPONENTS: &str = "PARENT_COMPONENTS";
const PARENT_COMPONENT_DEFAULTS: &str = "PARENT_COMPONENT_DEFAULTS";
const ADD_STAGE_SYSTEM_TEMPLATE: &str = ".add_system_to_stage(GodotStages::Add, add_nodes::<NAME_PASCAL>)";
const ADD_STAGE_SYSTEM: &str = "ADD_STAGE_SYSTEM";
const PROP_DECLS: &str = "PROP_DECLS";
const PROP_INITS: &str = "PROP_INITS";
const PROP_SETTERS: &str = "PROP_SETTERS";
const PROP_GETTERS: &str = "PROP_GETTERS";
const PROP_DEFAULTS: &str = "PROP_DEFAULTS";
const COMPONENT_SRC_TEMPLATE: &str = "use bevy_ecs::prelude::*;
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
                class_name: \"NAME_PASCAL\".to_string()
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
}";

const ADD_COMPONENTS_BRANCHES: &str = "ADD_COMPONENTS_BRANCHES";
const NODE_COMPONENT_PLUGINS: &str = "NODE_COMPONENT_PLUGINS";
const ADD_COMPONENTS_BRANCH_TEMPLATE: &str = "\"NAME_PASCAL\" => GDNAME_PASCAL::add_components(world_commands, entity, node),";
const MOD_TEMPLATE: &str = "
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
}";

struct GodotType {
    name: String,
    to_godot: Box<dyn Fn(&str) -> String>,
    to_bevy: Box<dyn Fn(&str) -> String>,
    default: Box<dyn Fn(&str) -> String>,
}

fn simple_type(type_name: &str, default: Option<Box<dyn Fn(&str) -> String>>) -> GodotType {
    GodotType {
        name: type_name.to_string(),
        to_godot: Box::new(|x| x.to_string()),
        to_bevy: Box::new(|x| x.to_string()),
        default: if let Some(func) = default {func} else {Box::new(|_| "Default::default()".to_string())},
    }
}

fn ref_type(type_name: &str, default: Option<Box<dyn Fn(&str) -> String>>) -> GodotType {
    GodotType {
        name: format!("Option<Ref<{type_name}>>"),
        to_godot: Box::new(|x| format!("{x}.as_ref().unwrap().clone()")),
        to_bevy: Box::new(|x| x.to_string()),
        default: if let Some(func) = default {func} else {Box::new(|_| "Default::default()".to_string())},
    }
}

fn pool_type(type_name: &str) -> GodotType {
    let type_name_str = type_name.to_string();
    GodotType {
        name: format!("Vec<{type_name}>"),
        to_godot: Box::new(move |x| format!("{type_name_str}Array::from_vec({x}.clone())")),
        to_bevy: Box::new(|x| format!("{x}.to_vec()")),
        default: Box::new(|_| "Default::default()".to_string()),
    }
}

fn get_types() -> HashMap<String, GodotType> {
    let mut map = HashMap::new();
    map.insert("Vector2".to_string(), simple_type("Vector2", None));
    map.insert("Vector3".to_string(), simple_type("Vector3", None));
    map.insert("AABB".to_string(), simple_type("Aabb", None));
    map.insert("int".to_string(), simple_type("i64", None));
    map.insert("float".to_string(), simple_type("f64", None));
    map.insert("bool".to_string(), simple_type("bool", None));
    map.insert("Basis".to_string(), simple_type("Basis", None));
    map.insert("Quat".to_string(), simple_type("Quat", None));
    map.insert("Color".to_string(), simple_type("Color", Some(Box::new(|x| format!("{x}::from_rgb(0.0, 0.0, 0.0)")))));
    map.insert("String".to_string(), GodotType {
        name: "String".to_string(),
        to_godot: Box::new(|x| format!("{x}.clone()")),
        to_bevy: Box::new(|x| format!("{x}.to_string()")),
        default: Box::new(|_| "Default::default()".to_string()),
    });
    map.insert("Transform".to_string(), simple_type("Transform", Some(Box::new(|x| format!("{x}::IDENTITY")))));
    map.insert("Transform2D".to_string(), simple_type("Transform2D", Some(Box::new(|x| format!("{x}::IDENTITY")))));
    map.insert("Rect2".to_string(), simple_type("Rect2", Some(Box::new(|x| format!("{x}::from_components(0.0, 0.0, 0.0, 0.0)")))));
    map.insert("PoolVector3Array".to_string(), pool_type("Vector3"));
    map.insert("PoolVector2Array".to_string(), pool_type("Vector2"));
    map.insert("PoolByteArray".to_string(), pool_type("Byte"));
    map.insert("PoolFloatArray".to_string(), pool_type("Float"));
    map.insert("PoolIntArray".to_string(), pool_type("Int32"));
    map.insert("PoolStringArray".to_string(), GodotType{
        name: "Vec<String>".to_string(),
        to_godot: Box::new(|x| format!("StringArray::from_vec({x}.iter().map(GodotString::from_str).collect())")),
        to_bevy: Box::new(|x| format!("{x}.to_vec().iter().map(|x| x.to_string()).collect()")),
        default: Box::new(|x| format!("Default::default()")),
    });
    map.insert("PoolColorArray".to_string(), pool_type("Color"));
    map.insert("NodePath".to_string(), GodotType {
        name: "NodePath".to_string(),
        to_godot: Box::new(|x| format!("{x}.to_godot_string()")),
        to_bevy: Box::new(|x| x.to_string()),
        default: Box::new(|_| "Default::default()".to_string()),
    });
    map.insert("ShaderMaterial,SpatialMaterial".to_string(), ref_type("Material", None));
    map.insert("ShaderMaterial,CanvasItemMaterial".to_string(), ref_type("Material", None));
    map.insert("ShaderMaterial,ParticlesMaterial".to_string(), ref_type("Material", None));
    map.insert("SpatialMaterial,ShaderMaterial".to_string(), ref_type("Material", None));
    map.insert("Texture".to_string(), ref_type("Texture", None));
    map.insert("Texture2D".to_string(), ref_type("Texture2D", None));
    map.insert("Mesh".to_string(), ref_type("Mesh", None));
    map.insert("MultiMesh".to_string(), ref_type("MultiMesh", None));
    map
}

// Converts Pascal Case to snake_case.
fn pascal_to_snake(input_str: &str) -> String {
    let mut output = String::new();
    for c in input_str.chars() {
        if !c.is_lowercase() {
            output.push('_');
            output.push_str(&c.to_lowercase().to_string());
        }
        else {
            output.push(c);
        }
    }
    output.remove(0);
    output
}

#[derive(Deserialize, Clone)]
struct Property {
    name: String,
    getter: String,
    setter: String,
}

#[derive(Deserialize, Clone)]
struct Argument {
    name: String,
    #[serde(rename = "type")]
    type_name: String,
    has_default_value: bool,
    default_value: String,
}

#[derive(Deserialize, Clone)]
struct Method {
    name: String,
    arguments: Vec<Argument>,
    return_type: String,
}

#[derive(Deserialize, Clone)]
struct ApiItem {
    name: String,
    base_class: String,
    properties: Vec<Property>,
    methods: Vec<Method>,
    instanciable: bool,
}

struct NodeTree {
    name: String,
    children: HashMap<String, NodeTree>,
}

// Generates a node tree.
fn gen_node_tree(api_dict: &Vec<ApiItem>, node_name: &str) -> NodeTree {
    let child_nodes: Vec<_> = api_dict.iter().filter(|x| x.base_class == node_name).collect();
    let mut child_tree = NodeTree {
        name: node_name.to_string(),
        children: HashMap::new(),
    };
    for child_node in child_nodes {
        let child_name = &child_node.name;
        child_tree.children.insert(child_name.to_string(), gen_node_tree(api_dict, child_name));
    }
    child_tree
}

// Generates a node list.
fn gen_node_list(api_dict: &Vec<ApiItem>, node_name: &str) -> Vec<String> {
    let child_nodes: Vec<_> = api_dict.iter().filter(|x| x.base_class == node_name).collect();
    let mut result_list = vec![node_name.to_string()];
    for child_node in child_nodes {
        let child_name = &child_node.name;
        result_list.extend(gen_node_list(api_dict, child_name));
    }
    result_list
}

// Processes the node, adding it to the mod file and generating a source file.
fn process_node(parents: Vec<&str>, node_name: &str, node_tree: &NodeTree, api_dict: &Vec<ApiItem>, node_dir: &str, node_mod_file: &str) {
    let types = get_types();
    
    // Generate component source file
    let api_node = api_dict.iter().find(|x| x.name == node_name).unwrap();
    let node_name_snake = pascal_to_snake(node_name);
    let parent_name = parents.last().unwrap_or(&"NullClass");
    let mut parent_components = Vec::new();
    let mut parent_component_defaults = Vec::new();
    for parent_name in &parents {
        let parent_name_snake = pascal_to_snake(parent_name);
        parent_components.push(format!("pub {parent_name_snake}: GD{parent_name},"));
        parent_component_defaults.push(format!("{parent_name_snake}: Default::default(),"));
    }
    let mut prop_decls = Vec::new();
    let mut prop_inits = Vec::new();
    let mut prop_setters = Vec::new();
    let mut prop_getters = Vec::new();
    let mut prop_defaults = Vec::new();
    for prop in &api_node.properties {
        let mut prop_name = prop.name.clone();
        
        // Don't cache any data that has no setters or getters
        let getters: Vec<Method> = api_node.methods.iter().filter(|x| x.name == prop.getter).map(Method::clone).collect();
        let no_getter = getters.is_empty() || !getters[0].arguments.is_empty() || getters[0].name.starts_with('_');
        let setters: Vec<Method> = api_node.methods.iter().filter(|x| x.name == prop.setter).map(Method::clone).collect();
        let no_setter = setters.is_empty() || setters[0].arguments.len() > 1 || setters[0].name.starts_with('_');
        if no_getter || no_setter {
            continue;
        }
        let prop_getter = &getters[0];
        let prop_setter = &setters[0];

        let prop_type = &prop_getter.return_type;
        
        if prop_name == "loop" {
            prop_name = "_loop".to_string();
        }

        if prop_name.contains('/') || prop_type.contains(':') || !types.contains_key(prop_type) {
            continue;
        }

        let mut getter = prop_getter.name.clone();
        if getter.starts_with("get_") {
            getter.drain(0..4);
        }
        getter = (types.get(prop_type).unwrap().to_bevy)(&format!("component_ref.{getter}()"));
        
        let setter = &prop_setter.name;
        let setter = format!("{setter}({})", (types.get(prop_type).unwrap().to_godot)(&format!("component.{prop_name}")));

        let prop_default = (types.get(prop_type).unwrap().default)(prop_type);

        let type_name = &types.get(prop_type).unwrap().name;
        prop_decls.push(format!("pub {prop_name}: {type_name},"));
        prop_inits.push(format!("{prop_name}: {getter},"));
        prop_setters.push(format!("component_ref.{setter};"));
        prop_getters.push(format!("component.{prop_name} = {getter};"));
        prop_defaults.push(format!("{prop_name}: {prop_default},"));
    }
    let component_src = COMPONENT_SRC_TEMPLATE
            .replace(ADD_STAGE_SYSTEM, if api_node.instanciable {ADD_STAGE_SYSTEM_TEMPLATE} else {""})
            .replace(PARENT_NAME_PASCAL, parent_name)
            .replace(NAME_PASCAL, node_name)
            .replace(NAME_SNAKE, &node_name_snake)
            .replace(PROP_DECLS, prop_decls.join("\n").as_str())
            .replace(PROP_INITS, prop_inits.join("\n").as_str())
            .replace(PROP_SETTERS,prop_setters.join("\n").as_str())
            .replace(PROP_GETTERS, prop_getters.join("\n").as_str())
            .replace(PROP_DEFAULTS, prop_defaults.join("\n").as_str())
            .replace(GDNODE_IMPORT, if node_name != "Node" {"use super::GDNode;"} else {""})
            .replace(PARENT_COMPONENTS, parent_components.join("\n").as_str())
            .replace(PARENT_COMPONENT_DEFAULTS, parent_component_defaults.join("\n").as_str());
    fs::write(format!("{node_dir}/{node_name_snake}.rs"), component_src).expect("Couldn't write to node file");

    // Add to mod.rs
    let mut file = OpenOptions::new().append(true).open(node_mod_file).unwrap();
    file.write_all(format!("pub mod {node_name_snake};\n").as_bytes()).expect("Couldn't append to mod.rs");
    file.write_all(format!("pub use {node_name_snake}::*;\n").as_bytes()).expect("Couldn't append to mod.rs");

    for (child_key, child_value) in &node_tree.children {
        let mut parents = parents.clone();
        parents.push(node_name);
        process_node(parents, child_key, child_value, api_dict, node_dir, node_mod_file);
    }
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let node_dir = format!("{}/nodes", out_dir.to_str().unwrap());
    let node_mod_file = format!("{}/mod.rs", node_dir);

    let reader = File::open("api.json").expect("Couldn't open api.json");
    let api_dict: Vec<ApiItem> = serde_json::from_reader(reader).expect("Couldn't parse api.json");

    // Find all descendants of Node and generate a tree and list
    let node_tree = gen_node_tree(&api_dict, "Node");
    let node_list = gen_node_list(&api_dict, "Node");

    // Process each node of the tree
    std::fs::remove_dir_all(&node_dir);
    std::fs::create_dir(&node_dir).expect("Couldn't create directory for nodes");
    std::fs::write(&node_mod_file, "").expect("Couldn't create mod.rs");
    process_node(Vec::new(), "Node", &node_tree, &api_dict, &node_dir, &node_mod_file);

    // Add add components function
    let add_components_branches: Vec<String> = node_list.iter().map(|x| ADD_COMPONENTS_BRANCH_TEMPLATE.replace(NAME_PASCAL, x)).collect();
    let node_component_plugins: Vec<String> = node_list.iter().map(|x| format!(".add_plugin({x}Plugin)")).collect();
    let add_components_fn = MOD_TEMPLATE
        .replace(ADD_COMPONENTS_BRANCHES, &add_components_branches.join("\n"))
        .replace(NODE_COMPONENT_PLUGINS, &node_component_plugins.join("\n"));
    let mut f = OpenOptions::new().append(true).open(node_mod_file).expect("Couldn't open mod.rs.");
    f.write_all(add_components_fn.as_bytes()).expect("Couldn't append to mod.rs.");

    println!("cargo:rerun-if-changed=build.rs,api.json");
}