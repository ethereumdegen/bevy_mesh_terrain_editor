
use bevy::prelude::*;




pub mod doodads;
pub mod placement;
pub mod zones;


use doodads::DoodadPlugin;
use zones::{zone_file::{CustomProp, CustomPropsComponent},  ZoneEvent, ZoneResource};


pub struct ExtWindowsPlugin {}
impl Plugin for ExtWindowsPlugin {
    fn build(&self, app: &mut App) {


    	app
            .add_event::<placement::PlacementEvent>()
            .add_event::<ZoneEvent>()
            .add_event::<doodads::PlaceDoodadEvent>()
            .register_type::<CustomPropsComponent>() //reflect
              .register_type::<CustomProp>() //reflect
            .add_event::<doodads::picking::SelectDoodadEvent>()
            .init_resource::<ZoneResource>()
            .init_resource::<placement::PlacementResource>()
            .add_systems(Update, zones::handle_zone_events)


            .add_plugins(DoodadPlugin {})
          
            .add_systems(Update, placement::update_placement_tool_inputs)
            .add_systems(Update, placement::handle_placement_tool_events)



            ;

            
    }
}