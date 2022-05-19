use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::PickableBundle;

#[derive(Reflect, Component)]
pub struct Cards;

#[derive(Inspectable, Component)]
pub struct Card {
    description: String,
}

#[derive(Bundle)]
struct CardBundle {
    card: Card,
    name: Name,
    #[bundle]
    pbr: PbrBundle,
}

pub fn create_cards_pack(mut commands: Commands) {
    commands.spawn().insert(Cards).insert(Name::new("Cards"));
}

// TODO Spawn cards only on panel // learning purposes

pub struct SpawnCardEvent;

/**
 * Spawn cards on event
 */
pub fn create_card(
    mut ev_spawn_card: EventReader<SpawnCardEvent>,
    mut commands: Commands,
    query: Query<(Entity, &Cards)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in ev_spawn_card.iter() {
        let (cards_entity, _cards) = query.get_single().unwrap();
        commands
            .spawn_bundle(CardBundle {
                card: Card {
                    description: String::from("ahoj"),
                },
                name: Name::new("Card"),
                pbr: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box {
                        min_x: 0.0,
                        max_x: 1.5,
                        min_y: 0.0,
                        max_y: 0.1,
                        min_z: 0.0,
                        max_z: 2.5,
                    })),
                    material: materials.add(Color::rgb(0.7, 0.7, 0.2).into()),
                    ..default()
                },
            })
            .insert(Parent(cards_entity))
            .insert_bundle(PickableBundle::default());
    }
}
