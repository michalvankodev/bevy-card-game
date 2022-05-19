use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

mod cards;
mod mouse_tracking;
mod scene_setup;
use bevy_mod_picking::DefaultPickingPlugins;
use cards::card::{Cards, Card};

fn main() {
   App::new()
      .insert_resource(WindowDescriptor {
         title: "bevy-card-game!".to_string(),
         width: 1194., // iPad Pro 11 resolution
         height: 834.,
         ..Default::default()
      })
      .add_plugins(DefaultPlugins)
      .add_plugin(WorldInspectorPlugin::new())
      .add_plugins(DefaultPickingPlugins) // <- Adds Picking, Interaction, and Highlighting plugins.
      .add_plugin(cards::card_plugin::CardPlugin)
      .register_inspectable::<Card>()
      .register_type::<Cards>()
      .add_startup_system(scene_setup::setup)
      //.add_system(mouse_tracking::cursor_events)
      .add_system(mouse_tracking::print_events)
      .run();
}
