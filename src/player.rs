use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use bevy::core::Name;
use bevy::math::bounding::{BoundingCircle, IntersectsVolume};

const GROUND_LEVEL: f32 = -100.0;
const PLAYER_X: f32 = -300.0;
const MOVEMENT_SPEED: f32 = 250.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movement {
    free_move: bool,
    destination: Vec2
}

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Velocity(Vec2);


pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // Player
    commands.spawn((
        Name::new("Player".to_string()),
        Player,
        Sprite {
            color: Color::srgb(1.0, 0.75, 0.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            anchor: Anchor::Center,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Movement {
            free_move: true,
            destination: Vec2::new(PLAYER_X, GROUND_LEVEL),
        },
        Velocity(Vec2::ZERO),
    ));

    // One tree?
    commands.spawn((
        Tree,
        Sprite {
            color: Color::srgb(0.0, 0.6, 0.1),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            anchor: Anchor::Center,
            ..default()
        },
        Transform::from_xyz(PLAYER_X + 100.0, GROUND_LEVEL, 0.0),
        Collider
    ));
}

pub fn handle_key_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Movement), With<Player>>,
)
{
    for (mut velocity, mut movement) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keys.pressed(KeyCode::KeyW) { direction.y += 1.0; }
        if keys.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keys.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
        if keys.pressed(KeyCode::KeyD) { direction.x += 1.0; }

        if direction != Vec2::ZERO {
            velocity.0 = direction.normalize() * MOVEMENT_SPEED;
            movement.free_move = true;
        } else {
            velocity.0 = Vec2::ZERO;
        }
    }
}

pub fn handle_mouse_input(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut query: Query<&mut Movement, With<Player>>,
)
{
    for mut movement in query.iter_mut() {
        if buttons.pressed(MouseButton::Left) {
            if let Some(mouse_position) = window.single().cursor_position() {
                // Get the camera information
                if let Ok((camera, camera_transform)) = cameras.get_single() {
                    // Convert window coordinates to world coordinates
                    if let Ok(world_position) = camera.viewport_to_world_2d(
                        camera_transform,
                        mouse_position
                    ) {
                        // A world position has been captured, free moving is disabled now while we're targeting a destination
                        // to allow for a click move to interrupt a key press move.
                        movement.destination = world_position;
                        movement.free_move = false;
                    }
                }
            }
        }
    }
}

pub fn update_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, &mut Movement), With<Player>>,
)
{
    for (mut transform, velocity, movement) in query.iter_mut() {
        let delta = time.delta_secs();
        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);

        let new_pos = if movement.free_move {
            current_pos + velocity.0 * delta
        } else {
            current_pos + (movement.destination - current_pos).normalize() * MOVEMENT_SPEED * delta
        };

        transform.translation.x = transform.translation.x.lerp(new_pos.x, 0.95);
        transform.translation.y = transform.translation.y.lerp(new_pos.y, 0.95);
    }
}

pub fn update_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<Camera>)>
)
{
    let player_transform = player_query.single().0;
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x.lerp(camera_transform.translation.x, 0.95);
    camera_transform.translation.y = player_transform.translation.y.lerp(camera_transform.translation.y, 0.95);
}

pub fn check_for_collisions(
    mut player_query: Query<(&Transform, &mut Velocity, &mut Sprite), With<Player>>,
    collider_query: Query<(&Transform), With<Collider>>,
)
{
    for collider_transform in collider_query.iter() {
        let mut player = player_query.single_mut();

        // Hardcode radius numbers and just use bounding circles for now.
        let collision = did_player_collide(
            BoundingCircle::new(player.0.translation.truncate(), 25.0),
            BoundingCircle::new(collider_transform.translation.truncate(), 25.0),
        );

        // Collision detection just changes color for now.
        if collision {
            player.2.color = Color::srgb(1.0, 0.0, 0.0);
        } else {
            player.2.color = Color::srgb(0.7, 0.7, 0.0);
        }
    }
}

fn did_player_collide (
    player: BoundingCircle,
    bounding_box: BoundingCircle,
) -> bool
{
    let mut collided = false;

    if player.intersects(&bounding_box) {
        collided = true;
    }

    collided
}