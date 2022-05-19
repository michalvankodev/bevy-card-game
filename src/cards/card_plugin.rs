use super::card;
use super::card::SpawnCardEvent;
use super::card_panel;
use bevy::prelude::*;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(card_panel::setup)
            .add_startup_system(card::create_cards_pack)
            .add_event::<SpawnCardEvent>()
            .add_system(card::create_card);
    }
}
