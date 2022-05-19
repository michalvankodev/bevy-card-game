use crate::cards::{
    card::{create_card, Cards, SpawnCardEvent},
    card_panel::CardPanel,
};
use bevy::prelude::*;
use bevy_mod_picking::PickingEvent;

pub fn cursor_events(mut cursor_evr: EventReader<CursorMoved>) {
    for ev in cursor_evr.iter() {
        println!(
            "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
            ev.position.x, ev.position.y, ev.id
        );
    }
}

pub fn print_events(
    mut events: EventReader<PickingEvent>,
    mut ev_spawn_card: EventWriter<SpawnCardEvent>,
    card_deck: Query<(Entity, &CardPanel)>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => {
                info!("Gee Willikers, it's a click! {:?}", e);
                let card_deck_id = card_deck.get_single().unwrap();
                //let entity = commands.entity(e);
                match e {
                    card_deck_id => ev_spawn_card.send(SpawnCardEvent),
                }
            }
        }
    }
}
