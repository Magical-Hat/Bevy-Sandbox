use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const GRID_SIZE: f32 = 10.0; // Size of each sand particle
const FALL_SPEED: f32 = 240.0; // Pixels per second

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.1)))
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_sand, update_sand))
        .run();
}

#[derive(Component)]
struct Sand;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn world_to_grid(pos: Vec2) -> (i32, i32) {
    (
        (pos.x / GRID_SIZE).floor() as i32,
        (pos.y / GRID_SIZE).floor() as i32
    )
}

fn grid_to_world(grid_x: i32, grid_y: i32) -> Vec2 {
    Vec2::new(
        grid_x as f32 * GRID_SIZE,
        grid_y as f32 * GRID_SIZE
    )
}

fn spawn_sand(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    sand_query: Query<&Transform, With<Sand>>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window_query.single();

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(
                camera_transform,
                cursor_position,
            ) {
                let (grid_x, grid_y) = world_to_grid(world_pos);
                let grid_world_pos = grid_to_world(grid_x, grid_y);

                let is_occupied = sand_query.iter().any(|transform| {
                    let (other_x, other_y) = world_to_grid(transform.translation.truncate());
                    other_x == grid_x && other_y == grid_y
                });

                if !is_occupied {
                    commands.spawn((
                        Sprite {
                            color: Color::rgb(0.8, 0.7, 0.2),
                            custom_size: Some(Vec2::splat(GRID_SIZE)),
                            ..default()
                        },
                        Transform::from_xyz(grid_world_pos.x, grid_world_pos.y, 0.0),
                        Sand,
                    ));
                }
            }
        }
    }
}

fn update_sand(
    mut sand_query: Query<&mut Transform, With<Sand>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    // Calculate bottom boundary aligned to grid
    let bottom = ((-window.height() / 2.0).ceil() / GRID_SIZE.ceil() * GRID_SIZE) + (GRID_SIZE / 2.0).ceil();

    // Collect positions
    let positions: Vec<Vec2> = sand_query
        .iter()
        .map(|transform| transform.translation.truncate())
        .collect();

    for mut transform in sand_query.iter_mut() {
        let current_pos = transform.translation.truncate();
        let next_pos = Vec2::new(
            current_pos.x,
            current_pos.y - FALL_SPEED * time.delta().as_secs_f32()
        );

        // Check for bottom boundary
        if next_pos.y <= bottom {
            transform.translation.y = bottom;
            continue;
        }

        // Check for collision with other sand particles
        let would_collide = positions.iter().any(|&pos| {
            pos.y != current_pos.y && // Don't collide with self
                pos.x == current_pos.x && // Same column
                pos.y < current_pos.y && // Below current
                pos.y >= next_pos.y - GRID_SIZE // Within one grid cell of next position
        });

        if would_collide {
            // Find the highest occupied position below current position
            let highest_below = positions.iter()
                .filter(|&&pos| pos.x == current_pos.x && pos.y < current_pos.y)
                .map(|&pos| pos.y)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(bottom);

            // Set position to one grid above the highest below
            transform.translation.y = highest_below + GRID_SIZE;
        } else {
            transform.translation.y = next_pos.y;
        }
    }
}