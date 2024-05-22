#![allow(clippy::type_complexity)]
//! Default windows for the editor

use bevy::prelude::*;
 



 


pub mod add;
pub mod assets;
pub mod cameras;
pub mod debug_settings;
pub mod diagnostics;
pub mod gizmos;
pub mod hierarchy;
pub mod inspector;
pub mod renderer;
pub mod resources;
 pub mod scenes;
pub mod lighting;


pub struct StandardWindowsPlugin {}
impl Plugin for StandardWindowsPlugin {
    fn build(&self, app: &mut App) {
        //put this inside of zone plugin ?
         app;
    }
}


 