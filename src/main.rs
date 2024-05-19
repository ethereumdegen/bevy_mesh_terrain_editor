use bevy_foliage_paint::foliage_config::FoliageConfig;
use bevy_foliage_paint::foliage::FoliageData;
use bevy_foliage_paint::BevyFoliagePaintPlugin;
use bevy_regions::regions::RegionsData;
use bevy_regions::regions_config::RegionsConfig;
use bevy_regions::BevyRegionsPlugin;
use asset_loading::asset_loading_plugin;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::input::mouse::MouseMotion;
use bevy::pbr::wireframe::WireframePlugin;
use bevy_magic_fx::MagicFxPlugin;
use ui::EditorToolsState;

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy_mesh_terrain::edit::EditingTool;
use bevy_mesh_terrain::terrain_config::TerrainConfig;
use bevy_mesh_terrain::{
    edit::{EditTerrainEvent, TerrainCommandEvent},
    terrain::{TerrainData, TerrainViewer},
    TerrainMeshPlugin,
};


use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::pbr::ShadowFilteringMethod;

use bevy_mod_raycast::prelude::*;

mod camera;
mod commands;
mod editor_pls;
mod tools;
mod ui;
mod asset_loading;
mod water;

mod doodads;
mod terrain;
mod foliage; 

use crate::camera::camera_plugin;
use crate::water::water_plugin;

use crate::tools::brush_tools_plugin;

use crate::commands::update_commands;
use crate::ui::editor_ui_plugin;

use seldom_fn_plugin::FnPluginExt;



use bevy::winit::WinitWindows;
use winit::window::Icon;

fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/images/favicon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}



fn main() {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings.features |= WgpuFeatures::POLYGON_MODE_LINE;

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoNoVsync, //improves latency

                        title: "Mesh Terrain Editor".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(wgpu_settings),
                    ..default()
                }),


        )   
        
        .add_plugins(DefaultRaycastingPlugin)
 

        .add_plugins(TerrainMeshPlugin::default())

        .add_plugins(BevyRegionsPlugin::default())
        .add_plugins(BevyFoliagePaintPlugin::default() )

        .add_plugins(doodads::doodad::DoodadPlugin)
        .add_plugins(terrain::terrain_manifest::TerrainManifestPlugin)
        .add_plugins(foliage::FoliagePlugin  )

        .add_plugins(bevy_obj::ObjPlugin)
        .add_plugins( MagicFxPlugin )
        .add_plugins(asset_loading_plugin)

 
        .fn_plugin(water_plugin)
        .fn_plugin(brush_tools_plugin)
        .fn_plugin(editor_ui_plugin)
        .fn_plugin(camera_plugin)
          .add_systems(Startup, set_window_icon)
        .add_systems(Startup, setup)
        //move to brushes and tools lib
        .add_systems(Update, update_commands)
         .add_systems(Update, update_regions_plane_visibility)
        .add_systems(Update, update_directional_light_position)
        //move to camera lib
        .add_plugins(editor_pls::editor_ui_plugin)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, // asset_server: Res<AssetServer>
) {
    commands
        .spawn(SpatialBundle::default())
        .insert(
            TerrainConfig::load_from_file("assets/terrain/default_terrain/terrain_config.ron")
                .unwrap(),
        )
        .insert(TerrainData::new());




              
     commands
        .spawn(SpatialBundle {
           transform: Transform::from_xyz(0.0, 40.0, 0.0) , 
            ..default()
        } )
        .insert(FoliageConfig::load_from_file("assets/foliage/foliage_config.ron").unwrap())
        .insert(FoliageData::new()) 
        //.insert(Visibility::Hidden)  // only in editor 
        ;


        //spawn regions painting plane 
     commands
        .spawn(SpatialBundle {
           transform: Transform::from_xyz(0.0, 40.0, 0.0) , 
            ..default()
        } )
        .insert(RegionsConfig::load_from_file("assets/regions/regions_config.ron").unwrap())
        .insert(RegionsData::new()) 
        .insert(Visibility::Hidden)  // only in editor 
        ;

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
           // shadow_depth_bias: 0.5,
           // shadow_normal_bias: 0.5,

            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,

            color: Color::WHITE,
            ..default()
        },

        transform: Transform {
            translation: Vec3::new(7.0, 20.0, 2.0),
            
            ..default()
        },

        ..default()
    }); 





    // camera

    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                 hdr: true, // 1. HDR must be enabled on the camera
                ..default()
            },

            transform: Transform::from_xyz(20.0, 162.5, 20.0)
                .looking_at(Vec3::new(900.0, 0.0, 900.0), Vec3::Y),
            ..default()
        })
        .insert( BloomSettings::default())
        .insert(TerrainViewer::default())
       // .insert(ShadowFilteringMethod::Jimenez14)
       ;
}

 


fn update_directional_light_position(
    mut query: Query<&mut Transform, With<DirectionalLight>>,
   
    time: Res<Time>,
) {

    let current_time = time.elapsed();


 //   let delta_time = time.delta_seconds();
    
    let SECONDS_IN_A_CYCLE = 80.0;

    let angle = (current_time.as_millis() as f32 / (SECONDS_IN_A_CYCLE* 1000.0) ) * std::f32::consts::PI * 2.0; // Convert time to radians

    let radius = 20.0; // Adjust the radius of the sun's orbit
    let x = angle.cos() * radius;
    let y = angle.sin() * radius + 10.0; // Adjust the height of the sun
    let z = 0.0;

    for mut transform in query.iter_mut() {

        transform.translation = Vec3::new(x, y, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}


fn update_regions_plane_visibility (

   mut region_plane_query: Query<&mut Visibility, With<RegionsData>>,
   editor_tools_state: Res<EditorToolsState>



  ){

    let Some( mut region_plane_vis ) = region_plane_query.get_single_mut().ok() else {return};

    *region_plane_vis = match &editor_tools_state.tool_mode {
        
        ui::ToolMode::Regions => Visibility::Visible, 
        _ => Visibility::Hidden
    }

}