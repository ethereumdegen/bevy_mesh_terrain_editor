use bevy::prelude::*;

use crate::editor_pls::bevy_pls_editor_is_active;
use crate::ui::{EditorToolsState, ToolMode};
use bevy::input::mouse::MouseMotion;
use bevy_mesh_terrain::edit::{EditingTool, TerrainBrushEvent};
use bevy_mesh_terrain::terrain_config::TerrainConfig;
use bevy_mesh_terrain::{
    edit::{BrushType, EditTerrainEvent, TerrainCommandEvent},
    terrain::{TerrainData, TerrainViewer},
    tool_preview::{ToolPreviewResource},
    TerrainMeshPlugin,
};

use bevy_egui::EguiContexts;

use bevy_mod_raycast::prelude::*;

pub fn brush_tools_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_brush_paint,
            handle_brush_events_from_terrain,



            ).chain().run_if(not(bevy_pls_editor_is_active)),
    );
}

struct EditingToolData {
    editing_tool: EditingTool,
    brush_type: BrushType,
    brush_radius: f32,
    brush_hardness: f32,
}

impl From<EditorToolsState> for EditingToolData {
    fn from(state: EditorToolsState) -> Self {
        let editing_tool = EditingTool::from(state.clone());

        Self {
            editing_tool,
            brush_radius: state.brush_radius as f32,
            brush_type: state.brush_type,
            brush_hardness: (state.brush_hardness as f32) / 100.0,
        }
    }
}

impl From<EditorToolsState> for EditingTool {
    fn from(state: EditorToolsState) -> Self {
        match state.tool_mode {
            ToolMode::Height => EditingTool::SetHeightMap {
                height: state.color.r,
            },
            ToolMode::Splat => EditingTool::SetSplatMap {
                r: state.color.r as u8,
                g: state.color.g as u8,
                b: state.color.b as u8,
            },
        }
    }
}

fn update_brush_paint(
    mouse_input: Res<ButtonInput<MouseButton>>, //detect mouse click

    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,

    mut edit_event_writer: EventWriter<EditTerrainEvent>,
    // command_event_writer: EventWriter<TerrainCommandEvent>,
    editor_tools_state: Res<EditorToolsState>,

    mut tool_preview_state: ResMut<ToolPreviewResource>,

    mut contexts: EguiContexts,
) {
     

    let egui_ctx = contexts.ctx_mut();
    if egui_ctx.is_pointer_over_area() {
        return;
    }

    //if tool is paintbrush ... (conditional check)

    //make me dynamic or whatever
    // let tool = EditingTool::SetHeightMap(125,25.0, false);

    let tool_data: EditingToolData = (*editor_tools_state).clone().into();

    let radius = tool_data.brush_radius;
    let brush_hardness = tool_data.brush_hardness;
    let brush_type = tool_data.brush_type;

    // let tool = EditingTool::SetSplatMap(5,1,0,25.0,false);

    if let Some(cursor_ray) = **cursor_ray {
        if let Some((intersection_entity, intersection_data)) =
            raycast.cast_ray(cursor_ray, &default()).first()
        {
            let hit_point = intersection_data.position();

            //offset this by the world psn offset of the entity !? would need to query its transform ?  for now assume 0 offset.
            let hit_coordinates = Vec2::new(hit_point.x, hit_point.z);

            //use an event to pass the entity and hit coords to the terrain plugin so it can edit stuff there


            tool_preview_state.tool_coordinates = hit_coordinates.clone();
            tool_preview_state.tool_radius = radius.clone();
            tool_preview_state.tool_color = (0.6,0.6,0.9).into() ;



             if !mouse_input.pressed(MouseButton::Left) {
                return;
            }


            edit_event_writer.send(EditTerrainEvent {
                entity: intersection_entity.clone(),
                tool: tool_data.editing_tool,
                brush_type,
                brush_hardness,
                coordinates: hit_coordinates,
                radius,
            });
        }
    }
}



fn handle_brush_events_from_terrain(
    mut evt_reader: EventReader<TerrainBrushEvent>,
    mut editor_tools_state: ResMut<EditorToolsState>,
){


    for evt in evt_reader.read(){

        info!("learned of evt {:?}", evt );
        match evt{
            TerrainBrushEvent::EyeDropTerrainHeight { height } => {

                editor_tools_state.color.r = * height ; 

            },
            TerrainBrushEvent::EyeDropSplatMap { r, g, b } => {

                editor_tools_state.color.r = * r as u16 ;
                editor_tools_state.color.g = * g as u16 ;
                editor_tools_state.color.b = * b as u16 ; 

            }
        }


    }
}