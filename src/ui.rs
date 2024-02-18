use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_egui::EguiContexts;


use bevy_mesh_terrain::edit::TerrainCommandEvent;
 
use std::fmt::{self, Formatter, Display};


pub fn editor_ui_plugin(app: &mut App){
    app 
    .init_resource::<EditorToolsState>()
    .add_plugins(EguiPlugin)
    .add_systems(Update, editor_tools)
  
    ; 
}


#[derive(Default, Resource)]
pub struct LinearPixelColor {
    r:u8,
    g:u8,
    b:u8,
    a:u8


}



#[derive(Default, Resource)]
pub struct EditorToolsState {
    //name: String,
     tool_radius:u32,
     tool_mode: ToolMode,
     color: LinearPixelColor
}

#[derive(Eq,PartialEq,Debug,Default)]
pub enum ToolMode { 
    #[default]
    Height,
    Splat 
}
const TOOL_MODES: [ToolMode; 2] = [ToolMode::Height, ToolMode::Splat];

impl Display for ToolMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let label = match self {
            ToolMode::Height => "Height",
            ToolMode::Splat => "Splat",
        };

        write!(f, "{}", label)
    }
}

 
fn editor_tools(
    mut tools_state: ResMut<EditorToolsState>,

    mut command_event_writer: EventWriter<TerrainCommandEvent>,

    mut contexts: EguiContexts

) {
    egui::Window::new("Editor Tools").show(contexts.ctx_mut(), |ui| {
     
        if ui.button("Save All Chunks (Ctrl+S)").clicked() {
             command_event_writer.send(
                 TerrainCommandEvent::SaveAllChunks(true,true,true)
                 
             )
        }

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
                            .selectable_label(tools_state.tool_mode == tool_mode, tool_mode.to_string())
                            .clicked()
                        {
                            tools_state.tool_mode = tool_mode;
                        }
                    }
                });
        });
        ui.spacing();
        ui.separator();

        ui.add(egui::Slider::new(&mut tools_state.tool_radius, 0..=100).text("tool_radius"));

        match tools_state.tool_mode {

            ToolMode::Splat => {
                ui.add(egui::Slider::new(&mut tools_state.color.r, 0..=255).text("Texture A (R_Channel"));
                ui.add(egui::Slider::new(&mut tools_state.color.g, 0..=255).text("Texture B (G_Channel"));
                ui.add(egui::Slider::new(&mut tools_state.color.b, 0..=255).text("Layer Fade (B_Channel"));

            },
            ToolMode::Height => {
                ui.add(egui::Slider::new(&mut tools_state.color.r, 0..=255).text("Height (R_Channel)"));
                
            }


        }

    });
} 