use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

mod card;
mod card_panel;
mod scene_setup;
use card::{Card, Cards};

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
      .register_inspectable::<Card>()
      .register_type::<Cards>()
      .add_startup_system(scene_setup::setup)
      .add_startup_system(card_panel::setup)
      .add_startup_system(card::create_cards_pack)
      .add_system(card::create_card)
      .run();
}
