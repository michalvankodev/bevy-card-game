use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

// TODO Make card panel stick to camera
// TODO Make card panel rotation and transformation calculated

#[derive(Component)]
pub struct CardPanel;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // box
    let mut box_transform = Transform::identity();
    box_transform.rotate(Quat::from_rotation_x(0.22));

    let card_panel_mesh = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -5.0,
                max_x: 5.0,
                min_y: 3.0,
                max_y: 3.0,
                min_z: 2.5,
                max_z: 5.0,
            })),
            material: materials.add(Color::rgb(0.2, 0.7, 0.6).into()),
            transform: Transform::from_rotation(Quat::from_rotation_x(0.50)),
            ..default()
        })
        .insert(Name::new("Card panel"))
        .id();
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 5.0),
        ..default()
    });

    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.1, 0.2, 0.1),
            ..default()
        })
        .insert(Name::new("Card deck"))
        .insert(CardPanel)
        .insert_bundle(PickableBundle::default())
        .insert(Parent(card_panel_mesh));
}
