use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

mod cards;
mod scene_setup;
use cards::card::{Card, Cards};

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
      .add_plugin(cards::card_plugin::CardPlugin)
      .register_inspectable::<Card>()
      .register_type::<Cards>()
      .add_startup_system(scene_setup::setup)
      .run();
}
