


use bevy::gltf::Gltf;
use bevy::log::tracing_subscriber::filter::FilterFn;
use bevy::pbr::wireframe::{Wireframe, WireframeColor};
use bevy::prelude::*;


use bevy::scene::SceneInstanceReady;
use bevy_editor_pls_core::Editor;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_mod_raycast::immediate::RaycastSettings;
use bevy_mod_raycast::CursorRay;

use bevy_mod_raycast::prelude::Raycast;

use super::doodad::LoadedGltfAssets;
use super::doodad_manifest::{DoodadManifest, DoodadManifestResource, RenderableType};
use super::DoodadToolState;



use anyhow::{Context, Result};


pub struct DoodadPlacementPlugin {}
impl Plugin for DoodadPlacementPlugin {
    fn build(&self, app: &mut App) {
        //put this inside of zone plugin ?
         app
            
            .add_systems(Startup, spawn_doodad_placement_component)
            .add_systems(Update, update_doodad_placement_preview_position)
             .add_systems(Update, update_doodad_placement_preview_state)
              .add_systems(Update, update_doodad_placement_preview_model)
              .add_systems(Update, apply_ghostly_material )
            // .add_systems(Update, doodad_placement_preview::update_doodad_placement_preview)


           
            ;
    }
}


 

#[derive(Component, Default)]
  struct WireframeMarker {}

#[derive(Component, Default)]
  struct GhostlyMaterialMarker {} 
 
#[derive(Component,Default)]
pub struct DoodadPlacementComponent {

	pub preview_doodad_name: Option<String> 

}


pub fn spawn_doodad_placement_component(
	mut commands: Commands
	){

	commands.spawn(SpatialBundle::default())
	.insert(DoodadPlacementComponent::default());

}


pub fn update_doodad_placement_preview_position (
  //  mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,

    

 //   doodad_tool_resource: Res<DoodadToolState>,

    mut contexts: EguiContexts,

    editor: Res<Editor>,


      doodad_placement_component_query: Query<Entity, With<DoodadPlacementComponent>>,
    mut doodad_placement_transform_query: Query<&mut Transform, With<DoodadPlacementComponent>>,

      parent_query: Query<&Parent >

) {
    //we can tell if we are clicking in viewport
    let egui_ctx = contexts.ctx_mut();

    let pointer_pos = egui_ctx.input(|input| input.pointer.interact_pos());
    let hovering_viewport = pointer_pos.map_or(false, |pos| editor.is_in_viewport(pos));

    if !hovering_viewport {
        return;
    }

    // ------- compute our rotation and scale from placement properties
    
   

 /*
 THIS raycast has to ignore doodads! 
 */		

 			//let raycast_filter = [];

 	let raycast_filter = |entity: Entity| {


         let mut current_entity = entity;
        loop {
            if doodad_placement_component_query.get(current_entity).is_ok() {
                return false;
            }
            match parent_query.get(current_entity).ok() {
                Some(parent) => current_entity = parent.get(),
                None => break,
            }
        }
        true
    };

    let raycast_settings = RaycastSettings {
        filter: &raycast_filter,
        ..default()
    };

    if let Some(cursor_ray) = **cursor_ray {
        if let Some((_intersection_entity, intersection_data)) =
            raycast.cast_ray(cursor_ray, &raycast_settings

            	
            	).first()
        {
            let hit_point = intersection_data.position();

            //offset this by the world psn offset of the entity !? would need to query its transform ?  for now assume 0 offset.
            let hit_coordinates = Vec3::new(hit_point.x, hit_point.y, hit_point.z);

            //use an event to pass the entity and hit coords to the terrain plugin so it can edit stuff there

            //   println!("place doodad 4 {:?}", doodad_definition);

           // let custom_props = None; 

         
         	if let Some( mut xform ) = doodad_placement_transform_query.get_single_mut().ok(){


         		xform.translation =  hit_coordinates.clone() ;

         	}
        }
    }


}



pub fn update_doodad_placement_preview_state (
  

    doodad_tool_resource: Res<DoodadToolState>,

    mut contexts: EguiContexts,

    editor: Res<Editor>,


    mut doodad_placement_component_query: Query<(Entity, &mut DoodadPlacementComponent), With<DoodadPlacementComponent>>

) {
    //we can tell if we are clicking in viewport
    let egui_ctx = contexts.ctx_mut();

    let pointer_pos = egui_ctx.input(|input| input.pointer.interact_pos());
    let hovering_viewport = pointer_pos.map_or(false, |pos| editor.is_in_viewport(pos));

    if !hovering_viewport {
        return;
    }

    // ------- compute our rotation and scale from placement properties
    
   

    let selected_doodad_definition = &doodad_tool_resource.selected;
 

  	

   	if let Some( (_placement_preview_entity, mut doodad_placement_comp) ) = doodad_placement_component_query.get_single_mut().ok(){

   		match selected_doodad_definition.clone() {
   			 Some(doodad_definition) => {


   			 	if doodad_placement_comp.preview_doodad_name != Some(doodad_definition.name.clone()) {
   			 		 doodad_placement_comp.preview_doodad_name  = Some(doodad_definition.name.clone());
   			 	}

   			 	
   			 }
   			 None => {

   			 	 doodad_placement_comp.preview_doodad_name  = None;
   			 }

   		}

   	 
         		//xform.translation =  hit_coordinates.clone() ;

        }

}



pub fn update_doodad_placement_preview_model (
  

    mut commands: Commands,

   // doodad_tool_resource: Res<DoodadToolState>,
 


    doodad_manifest_resource: Res<DoodadManifestResource>,
    doodad_manifest_assets: Res<Assets<DoodadManifest>>,

    gltf_assets: Res<LoadedGltfAssets>,
     models:  Res< Assets<bevy::gltf::Gltf>>,


     //this is happening too often !! 
      doodad_placement_component_query: Query<(Entity,&DoodadPlacementComponent), Changed<DoodadPlacementComponent>>

) {
    //we can tell if we are clicking in viewport
  

   
    // ------- compute our rotation and scale from placement properties
    
   

   // let selected_doodad_definition = &doodad_tool_resource.selected;
 

  	let Some((placement_preview_entity, doodad_placement_comp)) = doodad_placement_component_query.get_single().ok() else {return};
  	
  		 commands.entity(placement_preview_entity).despawn_descendants() ;

		  	  let Some(doodad_name) =  &doodad_placement_comp.preview_doodad_name else {return};


		  		  let Some(manifest_handle) = &doodad_manifest_resource.manifest else {
				        println!("WARN: no doodad manifest file found");
				        return;
				    };


					 let Some(manifest) = doodad_manifest_assets.get(manifest_handle) else {
				        println!("WARN: no doodad manifest file found");
				        return;
				    };


		  		   let Some(doodad_definition) = manifest.get_doodad_definition_by_name(doodad_name) else {
			            println!("WARN: Could not spawn doodad {:?}", doodad_name);
			            return;
			        };
		  	   
		  	
		    


		   match (&doodad_definition.model).clone() {
            RenderableType::GltfModel(model_name) => {

               match get_loaded_model_from_name(model_name, &gltf_assets, &models){

                        Ok(loaded_model)=> {

                        	info!("spawn preview placement model ");

                             let gltf_scene = commands.spawn(SpatialBundle::default())
                             .insert(  loaded_model.named_scenes["Scene"].clone() )
                             .insert(GhostlyMaterialMarker {})
                             .id();


                            commands 
                              .entity(placement_preview_entity)
                               .add_child(
                                gltf_scene
                                 )
                           //    .insert( Wireframe )
                              
                           
                                  ; 


                         }
                        ,
                       Err(_err) =>  {
                       
                        


                       }

                 };
                

              
            },
 

           _ =>  {

           	warn!("no preview for this model type");
           }
        }
}




fn get_loaded_model_from_name<'a>(
    model_name:String,

   
    gltf_assets: &Res<LoadedGltfAssets>,
     models: &'a Res<'_, Assets<bevy::gltf::Gltf>>,

     ) -> Result< &'a Gltf >{

    let model_handle = gltf_assets
                    .gltf_models
                    .get(model_name.as_str())
                    .context(format!(" no doodad model registered at "))?;

      let loaded_model = models
                    .get(model_handle)
                    .context(format!("Could not load model handle for {}", model_name))?;


         Ok(loaded_model)
}



fn add_wireframe_to_children( 
        mut commands: Commands ,

       doodad_query: Query<   (Entity,  &WireframeMarker) >,
         children_query: Query<&Children>,           
  
      mut  scene_instance_evt_reader: EventReader<SceneInstanceReady>

    ) {


 for evt in scene_instance_evt_reader.read(){
        let parent = evt.parent;
        
        if let Some((new_doodad_entity,wireframe_marker)) = doodad_query.get(parent).ok() {
         
          for child_entity in DescendantIter::new(&children_query, new_doodad_entity) { 
 
           
                commands.entity( child_entity ) 
                        .try_insert(Wireframe)
                        .try_insert(WireframeColor { color: Color::WHITE  } )
                        ;

                    }
             
        }

    }

       



}

fn apply_ghostly_material( 
        mut commands: Commands ,

       doodad_query: Query<   (Entity,  &GhostlyMaterialMarker), With<GhostlyMaterialMarker> >,
       children_query: Query<&Children>,           
  
      mut  scene_instance_evt_reader: EventReader<SceneInstanceReady>,


      	standard_material_query: Query<&Handle<StandardMaterial>>,

     mut   standard_material_assets : ResMut<Assets<StandardMaterial>>,


    ) {


 for evt in scene_instance_evt_reader.read(){
        let parent = evt.parent;
        
        if let Some((new_doodad_entity,_marker )) = doodad_query.get(parent).ok() {
         
          for child_entity in DescendantIter::new(&children_query, new_doodad_entity) { 
 
           	let Some(material_handle) = standard_material_query.get(child_entity).ok() else {continue};
               let Some(  mat) = standard_material_assets.get(material_handle) else {continue}; 

               let mut  new_mat = mat.clone();

               new_mat.alpha_mode = AlphaMode::Blend;
               new_mat.base_color = mat.base_color.clone().with_a( 0.25 );

                let new_material_handle = standard_material_assets.add(new_mat);
               commands.entity(child_entity).insert( new_material_handle   );

                   
           	}
             
        }

    }

       



}