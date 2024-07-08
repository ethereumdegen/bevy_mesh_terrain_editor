
use bevy::prelude::*;



 //this is causing stack overflow ? 


#[derive(Default)]
pub(crate) struct DoodadLoadPlugin;

impl Plugin for DoodadLoadPlugin {
    fn build(&self, app: &mut App) {
        app
           .add_systems(OnEnter(AssetLoadState::Complete), load_doodad_manifest)
           .add_systems(Update, build_doodad_data_from_manifest);

    }
}



  





fn load_doodad_manifest(
    asset_server: Res<AssetServer>,
    mut doodad_manifest_resource: ResMut<DoodadManifestResource>,
) {
    doodad_manifest_resource.manifest = Some(asset_server.load("doodad_manifest.doodadmanifest.ron"));

 
}

fn build_doodad_data_from_manifest(
    mut evt_asset: EventReader<AssetEvent<DoodadManifest>>,
    doodad_manifest_resource: Res<DoodadManifestResource>,

    mut doodad_tag_map_resource: ResMut<DoodadTagMapResource>, 
    doodad_manifest_assets: Res<Assets<DoodadManifest>>,

    mut loaded_gltf_resource: ResMut<LoadedGltfAssets>,

    asset_server: ResMut<AssetServer>,
) {
    let Some(doodad_manifest_handle) = &doodad_manifest_resource.manifest else {
        return;
    };

    for evt in evt_asset.read() {
        match evt {
            AssetEvent::LoadedWithDependencies { id } => {
                if id == &doodad_manifest_handle.id() {
                    let manifest: &DoodadManifest = doodad_manifest_assets
                        .get(doodad_manifest_handle.id())
                        .unwrap();

                    println!(" building doodad data  ");

                   /* for (doodad_name,doodad_definition) in &manifest.doodad_definitions {
                        let model_path_to_load = match &doodad_definition.model {
                            RenderableType::GltfModel(model_path) => Some(model_path),
                            _ => None, //other types dont need to have stuff preloaded
                        };

                        if let Some(model_path) = model_path_to_load {
                            let gltf_model_handle: Handle<Gltf> = asset_server.load(model_path);

                            loaded_gltf_resource
                                .gltf_models
                                .insert(model_path.clone(), gltf_model_handle);

                            println!("loaded gltf {:?}", model_path);
                        }
                    }*/

                    //now that our manifest is loaded, lets populate the doodad tag map resource 
                    for (doodad_name,doodad_definition) in &manifest.doodad_definitions {

                        for tag in &doodad_definition.tags.clone().unwrap_or(Vec::new()){
                            doodad_tag_map_resource.doodad_tag_map.entry(tag.clone()).or_default().push(doodad_name.to_string());
                        }


                        doodad_tag_map_resource.doodad_tag_map.entry("all_doodads".to_string()).or_default().push(doodad_name.to_string());

                    }

                     // Sort tags and doodad names
                  
                    let mut sorted_keys: Vec<_> = doodad_tag_map_resource.doodad_tag_map.keys().cloned().collect();
                    sorted_keys.sort();
                    doodad_tag_map_resource.doodad_tag_map = sorted_keys.into_iter().map(|k| (k.clone(), doodad_tag_map_resource.doodad_tag_map.remove(&k).unwrap())).collect();
                    
                      for doodads in doodad_tag_map_resource.doodad_tag_map.values_mut() {
                        doodads.sort();
                    }
 

             

                }
            }
            _ => {}
        }
    }
}