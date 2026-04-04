use convert_case::{Case, Casing as _};
use indexmap::IndexMap;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde_json::Value;
use std::collections::HashMap;
use syn::Ident;

use crate::write_file;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    min_x: i32,
    min_y: i32,
    min_z: i32,
    max_x: i32,
    max_y: i32,
    max_z: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BlockAttributes {
    flags: u8,
    fluid_state: u32,
    hardness: u32,     // Store as bits for hashing
    friction: u32,     // Store as bits for hashing
    speed_factor: u32, // Store as bits for hashing
    jump_factor: u32,  // Store as bits for hashing
    light_emission: u8,
    clockwise_rotation_offset: i32,
    shape_id: u16,           // Index into SHAPES array
    collision_shape_id: u16, // Index into COLLISION_SHAPES array
}

fn parse_aabb_list(shape_str: &str) -> Vec<Shape> {
    if shape_str == "[]" || shape_str.is_empty() {
        return Vec::new();
    }

    let mut bounds = Vec::new();

    // Split by "AABB" and process each
    for part in shape_str.split("AABB").skip(1) {
        let part = part.trim().trim_start_matches('[').trim_end_matches(']');

        // Find the arrow separator
        if let Some(arrow_pos) = part.find("->") {
            let min_part = part[..arrow_pos]
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']');
            let max_part = part[arrow_pos + 2..]
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim_end_matches(',');

            let min_coords: Vec<f32> = min_part
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let max_coords: Vec<f32> = max_part
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            if min_coords.len() == 3 && max_coords.len() == 3 {
                bounds.push(Shape {
                    min_x: (min_coords[0] * 1000.0) as i32,
                    min_y: (min_coords[1] * 1000.0) as i32,
                    min_z: (min_coords[2] * 1000.0) as i32,
                    max_x: (max_coords[0] * 1000.0) as i32,
                    max_y: (max_coords[1] * 1000.0) as i32,
                    max_z: (max_coords[2] * 1000.0) as i32,
                });
            }
        }
    }

    bounds
}

// ===== Block Flags =====

const FLAG_OCCLUDES: u8 = 1 << 0;
const FLAG_BLOCKS_MOTION: u8 = 1 << 1;
const FLAG_REQUIRES_TOOL: u8 = 1 << 2;
const FLAG_FLAMMABLE: u8 = 1 << 3;
const FLAG_SOLID: u8 = 1 << 4;
const FLAG_SOLID_BLOCKING: u8 = 1 << 5;
const FLAG_REDSTONE_CONDUCTOR: u8 = 1 << 6;

fn generate_block_flags(block: &Value) -> u8 {
    let mut flags = 0;

    if block["occludes"].as_bool().unwrap_or(false) {
        flags |= FLAG_OCCLUDES;
    }
    if block["blocksMotion"].as_bool().unwrap_or(false) {
        flags |= FLAG_BLOCKS_MOTION;
    }
    if block["requiresTool"].as_bool().unwrap_or(false) {
        flags |= FLAG_REQUIRES_TOOL;
    }
    if block["flammable"].as_bool().unwrap_or(false) {
        flags |= FLAG_FLAMMABLE;
    }
    if block["solid"].as_bool().unwrap_or(false) {
        flags |= FLAG_SOLID;
    }
    if block["solidBlocking"].as_bool().unwrap_or(false) {
        flags |= FLAG_SOLID_BLOCKING;
    }
    if block["redstoneConductor"].as_bool().unwrap_or(false) {
        flags |= FLAG_REDSTONE_CONDUCTOR;
    }

    flags
}

pub fn generate() {
    let entries: IndexMap<String, serde_json::Value> =
        serde_json::from_str(include_str!("../data/blocks.json")).unwrap();

    let mut unique_attributes: Vec<BlockAttributes> = Vec::new();
    let mut attribute_to_index: HashMap<BlockAttributes, usize> = HashMap::new();

    // Separate arrays for shapes and collision shapes
    let mut unique_shapes: HashMap<Vec<Shape>, u16> = HashMap::new();
    let mut shapes_vec: Vec<Vec<Shape>> = Vec::new();

    let mut unique_collision_shapes: HashMap<Vec<Shape>, u16> = HashMap::new();
    let mut collision_shapes_vec: Vec<Vec<Shape>> = Vec::new();

    // Map: block_state_id -> attribute_index
    let mut state_to_attribute: Vec<Option<usize>> = Vec::new();
    let mut max_state_id = 0usize;

    // Store block metadata for generating Block structs
    let mut block_metadata: Vec<(u16, u16, Vec<usize>, Vec<String>)> = Vec::new(); // (id, default_state, attribute_indices, property_names)

    // First pass: find max state ID
    for (_block_name, block_data) in &entries {
        if let Some(default_state) = block_data["defaultStateId"].as_u64() {
            max_state_id = max_state_id.max(default_state as usize);
        }

        if let Some(states) = block_data["states"].as_object() {
            for (_state_name, state_data) in states {
                if let Some(state_id) = state_data["stateId"].as_u64() {
                    max_state_id = max_state_id.max(state_id as usize);
                }
            }
        }
    }

    // Initialize state_to_attribute vec
    state_to_attribute.resize(max_state_id + 1, None);

    // Helper function to get attribute value with state override
    let get_value = |state: &Value, block: &Value, key: &str| -> Value {
        if !state[key].is_null() {
            state[key].clone()
        } else {
            block[key].clone()
        }
    };

    let get_or_insert_shape = |shapes_map: &mut HashMap<Vec<Shape>, u16>,
                               shapes_vec: &mut Vec<Vec<Shape>>,
                               shape: Vec<Shape>|
     -> u16 {
        if let Some(&id) = shapes_map.get(&shape) {
            id
        } else {
            let id = shapes_vec.len() as u16;
            shapes_vec.push(shape.clone());
            shapes_map.insert(shape, id);
            id
        }
    };

    // Second pass: create unique BlockAttributes and map states
    for (block_idx, (_block_name, block_data)) in entries.iter().enumerate() {
        let mut block_attribute_indices: Vec<usize> = Vec::new();
        let default_state_id = block_data["defaultStateId"].as_u64().unwrap_or(0) as u16;

        // Extract property names
        let mut property_names: Vec<String> = Vec::new();
        if let Some(properties) = block_data["properties"].as_array() {
            for prop_name in properties {
                let prop_name = prop_name.as_str().unwrap();
                property_names.push(prop_name.to_case(Case::UpperSnake));
            }
        }

        // Map all states for this block (each state can override any attribute)
        if let Some(states) = block_data["states"].as_object() {
            for (_state_name, state_data) in states {
                if let Some(state_id) = state_data["stateId"].as_u64() {
                    // Get attributes with state overrides
                    let hardness = get_value(state_data, block_data, "hardness")
                        .as_f64()
                        .unwrap_or(0.0);
                    let friction = get_value(state_data, block_data, "friction")
                        .as_f64()
                        .unwrap_or(0.6);

                    let shape_str = if !state_data["shape"].is_null() {
                        state_data["shape"].as_str().unwrap_or("[]")
                    } else {
                        block_data["shape"].as_str().unwrap_or("[]")
                    };

                    let collision_shape_str = if !state_data["collisionShape"].is_null() {
                        state_data["collisionShape"].as_str().unwrap_or("[]")
                    } else {
                        block_data["collisionShape"].as_str().unwrap_or("[]")
                    };

                    let shape = parse_aabb_list(shape_str);
                    let collision_shape = parse_aabb_list(collision_shape_str);

                    let shape_id = get_or_insert_shape(&mut unique_shapes, &mut shapes_vec, shape);
                    let collision_shape_id = get_or_insert_shape(
                        &mut unique_collision_shapes,
                        &mut collision_shapes_vec,
                        collision_shape,
                    );

                    // Create state-specific block data for flag generation
                    let mut state_block_data = block_data.clone();
                    if !state_data["blocksMotion"].is_null() {
                        state_block_data["blocksMotion"] = state_data["blocksMotion"].clone();
                    }
                    if !state_data["solid"].is_null() {
                        state_block_data["solid"] = state_data["solid"].clone();
                    }
                    if !state_data["solidBlocking"].is_null() {
                        state_block_data["solidBlocking"] = state_data["solidBlocking"].clone();
                    }
                    if !state_data["mapColorId"].is_null() {
                        state_block_data["mapColorId"] = state_data["mapColorId"].clone();
                    }

                    let flags = generate_block_flags(&state_block_data);

                    let attr = BlockAttributes {
                        flags,
                        fluid_state: 0,
                        hardness: (hardness as f32).to_bits(),
                        friction: (friction as f32).to_bits(),
                        speed_factor: 1.0f32.to_bits(),
                        jump_factor: 1.0f32.to_bits(),
                        light_emission: 0,
                        clockwise_rotation_offset: 0,
                        shape_id,
                        collision_shape_id,
                    };

                    let attr_idx = if let Some(&idx) = attribute_to_index.get(&attr) {
                        idx
                    } else {
                        let idx = unique_attributes.len();
                        unique_attributes.push(attr.clone());
                        attribute_to_index.insert(attr, idx);
                        idx
                    };

                    state_to_attribute[state_id as usize] = Some(attr_idx);
                    block_attribute_indices.push(attr_idx);
                }
            }
        } else {
            // No states defined, use default state only
            if let Some(default_state) = block_data["defaultStateId"].as_u64() {
                let flags = generate_block_flags(&block_data);
                let hardness = block_data["hardness"].as_f64().unwrap_or(0.0);
                let friction = block_data["friction"].as_f64().unwrap_or(0.6);

                let shape = parse_aabb_list(block_data["shape"].as_str().unwrap_or("[]"));
                let collision_shape =
                    parse_aabb_list(block_data["collisionShape"].as_str().unwrap_or("[]"));

                let shape_id = get_or_insert_shape(&mut unique_shapes, &mut shapes_vec, shape);
                let collision_shape_id = get_or_insert_shape(
                    &mut unique_collision_shapes,
                    &mut collision_shapes_vec,
                    collision_shape,
                );

                let attr = BlockAttributes {
                    flags,
                    fluid_state: 0,
                    hardness: (hardness as f32).to_bits(),
                    friction: (friction as f32).to_bits(),
                    speed_factor: 1.0f32.to_bits(),
                    jump_factor: 1.0f32.to_bits(),
                    light_emission: 0,
                    clockwise_rotation_offset: 0,
                    shape_id,
                    collision_shape_id,
                };

                let attr_idx = if let Some(&idx) = attribute_to_index.get(&attr) {
                    idx
                } else {
                    let idx = unique_attributes.len();
                    unique_attributes.push(attr.clone());
                    attribute_to_index.insert(attr, idx);
                    idx
                };

                state_to_attribute[default_state as usize] = Some(attr_idx);
                block_attribute_indices.push(attr_idx);
            }
        }

        block_metadata.push((
            block_idx as u16,
            default_state_id,
            block_attribute_indices,
            property_names,
        ));
    }

 

    let enum_entries = entries
        .iter()
        .enumerate()
        .map(|(_, (key, _))| {
            let ident_enum = key
                .split_once(':')
                .map(|(_, path)| path)
                .unwrap()
                .to_case(Case::Pascal);
            let ident_enum = Ident::new(&ident_enum, Span::call_site());

            let ident_const = key
                .split_once(':')
                .map(|(_, path)| path)
                .unwrap()
                .to_case(Case::UpperSnake);
            let ident_const = Ident::new(&ident_const, Span::call_site());

            quote! {
                #ident_enum = #ident_const = #key
            }
        })
        .collect::<Vec<TokenStream>>();

    // Generate const definitions for each block
    let const_definitions = entries
        .iter()
        .enumerate()
        .map(|(block_idx, (key, _))| {
            let ident_const = key
                .split_once(':')
                .map(|(_, path)| path)
                .unwrap()
                .to_case(Case::UpperSnake);
            let ident_const = Ident::new(&ident_const, Span::call_site());

            let (id, default_state, _attr_indices, property_names) = &block_metadata[block_idx];

            // Generate property references
            let properties_tokens = if property_names.is_empty() {
                quote! { &[] }
            } else {
                let prop_idents: Vec<_> = property_names
                    .iter()
                    .map(|prop_name| {
                        let ident = Ident::new(prop_name, Span::call_site());
                        quote! { BlockProperty::#ident }
                    })
                    .collect();
                quote! { &[#(&#prop_idents),*] }
            };
            
            let ident_enum = key
                .split_once(':')
                .map(|(_, path)| path)
                .unwrap()
                .to_case(Case::Pascal);
            let ident_enum = Ident::new(&ident_enum, Span::call_site());

            quote! {
                register(&#ident_const, Block::#ident_enum, #key, __private::Block {
                    id: #id,
                    default_state: #default_state,
                    properties: #properties_tokens,
                    min_state_id: 0,
                });
            }
        })
        .collect::<Vec<TokenStream>>();


    let output = quote! {

        define_blocks! {
            pub enum Block {
                #(#enum_entries),*
            }
        }

        fn __init(registry: &mut BlockRegistry) {
            let mut register = |holder: &'static BlockHolder, block_enum: Block, key: &str, block: __private::Block| {
                registry.register(holder, block_enum, key, block);
            };
            
            #(#const_definitions)*
        }

    };
    write_file(&output, "blocks.rs");
}
