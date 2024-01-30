use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

#[derive(Component)]
struct SandParticle;

#[derive(Component)]
struct Moving;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sand_physics_system)
        .add_system(spawn_sand_particles_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_sand_particles_system(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    let mut rng = rand::thread_rng();
    let window = windows.get_primary().unwrap();
    if let Some(cursor_position) = window.cursor_position() {
        if mouse_button_input.pressed(MouseButton::Left) {
            let color = Color::rgb(
                rng.gen_range(0.7..0.9),
                rng.gen_range(0.6..0.8),
                rng.gen_range(0.1..0.3),
            );

            // Random offset for spawning adjacent particles
            let offset_x = if rng.gen_bool(0.5) {
                rng.gen_range(-20..=20) as f32
            } else {
                0.0
            };
            let offset_y = if rng.gen_bool(0.5) {
                rng.gen_range(-20..=20) as f32
            } else {
                0.0
            };

            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(5.0, 5.0)),
                        color,
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        cursor_position.x - window.width() / 2.0 + offset_x,
                        cursor_position.y - window.height() / 2.0 + offset_y,
                        0.0,
                    ),
                    ..default()
                })
                .insert(SandParticle)
                .insert(Moving);
        }
    }
}

fn sand_physics_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, With<SandParticle>, With<Moving>)>,
) {
    let ground_level = -300.0; // Define the ground level
    let mut occupied_positions: HashMap<(i32, i32), Entity> = HashMap::new();
    let collapse_height = 10; // Define the height at which columns should collapse

    // Step 1: Update positions and track heights
    for (entity, mut transform, _, _) in query.iter_mut() {
        let grid_x = (transform.translation.x / 5.0).round() as i32;
        let grid_y = (transform.translation.y / 5.0).round() as i32;

        // Check for ground collision or if the position below is occupied
        if transform.translation.y <= ground_level
            || occupied_positions.contains_key(&(grid_x, grid_y - 1))
        {
            occupied_positions.insert((grid_x, grid_y), entity);
        } else {
            // Particle falls
            transform.translation.y -= 6.0;
        }
    }

    // Step 2: Collapse unsupported columns
    let mut to_collapse = Vec::new();
    for (&(x, y), &entity) in &occupied_positions {
        let mut column_height = 1;
        let mut current_y = y - 1;
        while occupied_positions.contains_key(&(x, current_y)) {
            column_height += 1;
            current_y -= 1;
        }

        let has_left_support = occupied_positions.contains_key(&(x - 1, y));
        let has_right_support = occupied_positions.contains_key(&(x + 1, y));

        if column_height >= collapse_height && !has_left_support && !has_right_support {
            to_collapse.push(entity);
        }
    }

    for entity in to_collapse {
        if let Ok((_, mut transform, _, _)) = query.get_mut(entity) {
            transform.translation.y -= 2.0; // Move the particle down to simulate collapse
            commands.entity(entity).insert(Moving);
        }
    }
}
