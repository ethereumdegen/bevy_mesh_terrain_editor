 
use crate::terrain::terrain_manifest::{TerrainManifestResource,TerrainManifest};
use bevy::prelude::*;

use bevy_egui::EguiContexts;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use bevy_mesh_terrain::edit::{BrushType as TerrainBrushType, TerrainCommandEvent};
use bevy_regions::edit::{BrushType as RegionsBrushType, RegionCommandEvent};
use bevy_foliage_paint::edit::{BrushType as FoliageBrushType, FoliageCommandEvent};

use std::fmt::{self, Display, Formatter};

use crate::editor_pls::bevy_pls_editor_is_active;

pub fn editor_ui_plugin(app: &mut App) {
    app.init_resource::<EditorToolsState>()
       // .add_plugins(EguiPlugin)  // only add this if it hasnt been added 
        .add_systems(Update, editor_tools.run_if(not(bevy_pls_editor_is_active)));
}

#[derive(Default, Resource, Clone)]
pub struct LinearPixelColor {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

#[derive(Default, Resource, Clone)]
pub struct EditorToolsState {
    pub tool_mode: ToolMode,
    pub brush_type: BrushType,
    pub brush_radius: u32,
    pub brush_hardness: u32,
    pub color: LinearPixelColor, //brush mode
}


#[derive(Clone ,PartialEq)]
pub enum BrushType {
 
    SetExact,
    Smooth,
    Noise,
    EyeDropper
}

impl BrushType{

    pub fn to_string(&self) -> String{

        match self {

            BrushType::SetExact  => "Set Exact".into(),
             BrushType::Smooth  => "Smooth".into(),
             BrushType::Noise  => "Noise".into(),
             BrushType::EyeDropper  => "Eyedropper".into(),

            

        }

    }
}

impl Default for BrushType {
    fn default() -> Self {
      BrushType::SetExact 
    }
} 

#[derive(Eq, PartialEq, Debug, Default, Clone)]
pub enum ToolMode {
    #[default]
    Height,
    Splat,
    Foliage,
    Regions
}
const TOOL_MODES: [ToolMode; 4] = [
ToolMode::Height, 
ToolMode::Splat, 
ToolMode::Foliage, 
ToolMode::Regions
];

const BRUSH_TYPES_HEIGHT: [ BrushType; 4] = [
BrushType::SetExact , 
BrushType::Smooth , 
BrushType::Noise , 
BrushType::EyeDropper
];
const BRUSH_TYPES_SPLAT: [ BrushType; 2] = [
BrushType::SetExact , 
 
 BrushType::EyeDropper
];
const BRUSH_TYPES_REGION: [BrushType; 2] = [
BrushType::SetExact ,   
BrushType::EyeDropper
];

//consider adding more stuff bc of bitmasking 
const BRUSH_TYPES_FOLIAGE: [BrushType; 2] = [
BrushType::SetExact ,   
BrushType::EyeDropper
];


impl Display for ToolMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let label = match self {
            ToolMode::Height => "Height",
            ToolMode::Splat => "Splat",
            ToolMode::Foliage => "Foliage",
            ToolMode::Regions => "Regions"
        };

        write!(f, "{}", label)
    }
}

fn editor_tools(
    mut tools_state: ResMut<EditorToolsState>,

    mut command_event_writer: EventWriter<TerrainCommandEvent>,
    mut foliage_command_event_writer: EventWriter<FoliageCommandEvent>,
    mut region_command_event_writer: EventWriter<RegionCommandEvent>,

    mut contexts: EguiContexts,

    terrain_manifest_res: Res<TerrainManifestResource>,
    terrain_manifest_asset: Res<Assets<TerrainManifest>>
) {
    egui::Window::new("Editor Tools").show(contexts.ctx_mut(), |ui| {
        if ui.button("Save All   (Ctrl+S)").clicked() {
            command_event_writer.send(TerrainCommandEvent::SaveAllChunks(true, true, true));
            region_command_event_writer.send(RegionCommandEvent::SaveAll );
            foliage_command_event_writer.send(FoliageCommandEvent::SaveAll );
        }

       // if ui.button("Save Splat and Height").clicked() {
      //      command_event_writer.send(TerrainCommandEvent::SaveAllChunks(true, true, false));
      //  }

        ui.spacing();
        ui.separator();

        /*ui.horizontal(|ui| {
            let name_label = ui.label("Your name: ");
            ui.text_edit_singleline(&mut tools_state.name)
                .labelled_by(name_label.id);
        });*/

        ui.heading("Tool Mode");
        ui.horizontal(|ui| {
            ui.label("Mode:");
            ui.spacing();
            egui::ComboBox::new("tool_mode", "")
                .selected_text(tools_state.tool_mode.to_string())
                .show_ui(ui, |ui| {
                    for tool_mode in TOOL_MODES.into_iter() {
                        if ui
                            .selectable_label(
                                tools_state.tool_mode == tool_mode,
                                tool_mode.to_string(),
                            )
                            .clicked()
                        {
                            tools_state.tool_mode = tool_mode;
                        }
                    }
                });
        });
        ui.spacing();
        ui.separator();

        ui.add(egui::Slider::new(&mut tools_state.brush_radius, 0..=100).text("Brush Radius"));
        ui.add(egui::Slider::new(&mut tools_state.brush_hardness, 0..=100).text("Brush Hardness"));

        match tools_state.tool_mode {
            ToolMode::Splat => {

                 egui::ComboBox::new("brush_type", "")
                    .selected_text(tools_state.brush_type.to_string())
                    .show_ui(ui, |ui| {
                        for brush_type in BRUSH_TYPES_SPLAT.into_iter() {
                            if ui
                                .selectable_label(
                                    tools_state.brush_type == brush_type,
                                    brush_type.to_string(),
                                )
                                .clicked()
                            {
                                tools_state.brush_type = brush_type;
                            }
                        }
                    });


                let terrain_index_A = tools_state.color.r.clone();
                let terrain_index_B = tools_state.color.g.clone();

                let terrain_manifest:Option<&TerrainManifest> =  terrain_manifest_res.manifest.as_ref().map(|m| terrain_manifest_asset.get( m )).flatten();

                ui.add(
                    egui::Slider::new(&mut tools_state.color.r, 0..=255)
                        .text("Texture A (R_Channel"),
                );

                if let Some(terrain_def) = terrain_manifest.map(|m| m.get_terrain_type(terrain_index_A) ).flatten() {
                     ui.label( terrain_def.name.clone() );
                }
               
                ui.add(
                    egui::Slider::new(&mut tools_state.color.g, 0..=255)
                        .text("Texture B (G_Channel"),
                );
                 
                if let Some(terrain_def) = terrain_manifest.map(|m| m.get_terrain_type(terrain_index_B) ).flatten() {
                     ui.label( terrain_def.name.clone() );
                }
                ui.add(
                    egui::Slider::new(&mut tools_state.color.b, 0..=255)
                        .text("Layer Fade (B_Channel"),
                );
            }
            ToolMode::Height => {
                egui::ComboBox::new("brush_type", "")
                    .selected_text(tools_state.brush_type.to_string())
                    .show_ui(ui, |ui| {
                        for brush_type in BRUSH_TYPES_HEIGHT.into_iter() {
                            if ui
                                .selectable_label(
                                    tools_state.brush_type == brush_type,
                                    brush_type.to_string(),
                                )
                                .clicked()
                            {
                                tools_state.brush_type = brush_type;
                            }
                        }
                    });

                ui.add(
                    egui::Slider::new(&mut tools_state.color.r, 0..=65535)
                        .text("Height (R_Channel)"),
                );
            },
            ToolMode::Foliage => {


                 egui::ComboBox::new("brush_type", "")
                    .selected_text(tools_state.brush_type.to_string())
                    .show_ui(ui, |ui| {
                        for brush_type in BRUSH_TYPES_FOLIAGE.into_iter() {
                            if ui
                                .selectable_label(
                                    tools_state.brush_type == brush_type,
                                    brush_type.to_string(),
                                )
                                .clicked()
                            {
                                tools_state.brush_type = brush_type;
                            }
                        }
                    });

                ui.add(
                    egui::Slider::new(&mut tools_state.color.r, 0..=256)
                        .text("Foliage Index (R_Channel)"),
                );


            }
            ToolMode::Regions => {


                 egui::ComboBox::new("brush_type", "")
                    .selected_text(tools_state.brush_type.to_string())
                    .show_ui(ui, |ui| {
                        for brush_type in BRUSH_TYPES_REGION.into_iter() {
                            if ui
                                .selectable_label(
                                    tools_state.brush_type == brush_type,
                                    brush_type.to_string(),
                                )
                                .clicked()
                            {
                                tools_state.brush_type = brush_type;
                            }
                        }
                    });

                ui.add(
                    egui::Slider::new(&mut tools_state.color.r, 0..=64)
                        .text("Region Index (R_Channel)"),
                );


            }
        }
    });
}
