use bevy::prelude::*;

// TODO Make card panel stick to camera
// TODO Make card panel rotation and transformation calculated

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // box
    let mut box_transform = Transform::identity();
    box_transform.rotate(Quat::from_rotation_x(0.22));

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box { min_x: -5.0, max_x: 5.0, min_y: 3.0, max_y: 3.0, min_z: 2.5, max_z: 5.0 })),
        material: materials.add(Color::rgb(0.2, 0.7, 0.6).into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(0.50)),
        ..default()
    });
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
}
