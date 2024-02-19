use bevy::prelude::*;

use bevy::input::mouse::MouseMotion;
use bevy_mesh_terrain::{TerrainMeshPlugin, terrain::{  TerrainData, TerrainViewer}, edit::{EditTerrainEvent, TerrainCommandEvent}};
 use bevy_mesh_terrain::terrain_config::TerrainConfig;
use bevy_mesh_terrain::edit::EditingTool;


use bevy_mod_raycast::prelude::*;

pub fn update_camera_look(
    mut event_reader:   EventReader<MouseMotion>  ,
    mouse_input:  Res< Input<MouseButton> > ,
    mut query: Query<(&mut Transform, &Camera3d)>,
    
    
){
    const MOUSE_SENSITIVITY: f32 = 2.0;
     
    
    
      
      // Accumulate mouse delta
    let mut delta: Vec2 = Vec2::ZERO;
    for event in event_reader.iter() {
        delta += event.delta;
    }
    if !mouse_input.pressed(MouseButton::Right) {
        return;
    }

    // Apply to each camera with the CameraTag
    for (mut transform, _) in query.iter_mut() {
       // let rotation = transform.rotation;
      
        let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
       
        yaw -= delta.x / 180.0   * MOUSE_SENSITIVITY  ;
        pitch -= delta.y / 180.0   * MOUSE_SENSITIVITY;
        pitch = pitch .clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0) ;
   
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
       
    }
    
}


pub fn update_camera_move(
   
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera3d)>,
    
    
){
      const MOVE_SPEED: f32 = 10.0; // You can adjust this value as needed
     
     
     
  
    // Apply to each camera with the CameraTag
    for (mut transform, _) in query.iter_mut() {
       
      
           // Move the camera forward if W is pressed
        if keyboard_input.pressed(KeyCode::W) {
            let forward = transform.forward();
            transform.translation += forward * MOVE_SPEED;
        }
         
          if keyboard_input.pressed(KeyCode::S) {
            let forward = transform.forward() ;
            transform.translation -= forward * MOVE_SPEED;
        }
         
        
       
    }
    
}


