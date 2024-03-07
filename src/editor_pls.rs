use bevy::prelude::*;

use bevy_editor_pls::controls::EditorControls;
use bevy_editor_pls::custom_windows::CustomWindowsPlugin;
use bevy_editor_pls::default_windows::cameras::camera_3d_free;
use bevy_editor_pls::EditorPlugin;
use bevy_editor_pls::controls;
//use bevy_editor_pls_default_windows::hierarchy::picking::EditorRayCastSource;

use bevy_editor_pls::editor;


pub fn editor_ui_plugin(app: &mut App) {
    app

       .add_plugins(EditorPlugin{
            enable_camera_controls: true, 
          ..default()
       })
        .insert_resource(editor_controls())


          .add_plugins(CustomWindowsPlugin {} ) 

         // .add_sytems( Update, set_terrain_as_unpickable )
          ;

        //.add_systems(Startup, disable_cam3d_controls) //we handle camera controls on our own 
        ;
}


fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);


    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}
 



pub fn bevy_pls_editor_is_active(
       pls_editor_resource: Res<editor::Editor> ,
       ) -> bool {

         pls_editor_resource.active()  
}