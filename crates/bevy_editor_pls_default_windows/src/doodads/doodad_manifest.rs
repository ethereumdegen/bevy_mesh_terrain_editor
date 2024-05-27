use bevy::utils::HashMap;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use crate::zones::zone_file::CustomPropsMap;

#[derive(Resource, Default)]
pub struct DoodadManifestResource {
    pub manifest: Option<Handle<DoodadManifest>>,

    pub doodad_tag_map: HashMap< String, Vec<DoodadDefinition>  >
}

#[derive(Asset, TypePath, Clone, Debug, Serialize, Deserialize)]
pub struct DoodadManifest {
    pub doodad_tags: Vec<String>,
    pub doodad_definitions: HashMap<String,DoodadDefinition>,
}

impl DoodadManifest {
    pub fn get_doodad_definition_by_name(&self, name: &str) -> Option<DoodadDefinition> {
     

        return self.doodad_definitions.get(name).cloned();

        
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RenderableType {
    GltfModel(String), //the path
    CubeShape(CubeShapeDefinition),
    MagicFx(String),
    LiquidPlane(String)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CubeShapeDefinition {
    pub color: Color,
    pub wireframe: bool 
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct DoodadDefinition {
   // pub name: String,
    pub model: RenderableType,
    pub initial_custom_props: Option<CustomPropsMap>,
    pub tags: Option<Vec<String>> ,
    pub snap_dimensions: Option<Vec2>, 
}

impl DoodadManifest {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = get_doodad_manifest_file_path();
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data: Self = ron::de::from_str(&contents)?;
        Ok(data)
    }
}

fn get_doodad_manifest_file_path() -> String {
    format!("assets/doodad_manifest.ron")
}
