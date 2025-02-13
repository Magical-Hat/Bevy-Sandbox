use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use bevy::core::Name;

const GROUND_LEVEL: f32 = -100.0;
const PLAYER_X: f32 = -300.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movement {
    free_move: bool,
    destination: Vec2
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct Velocity(Vec2);


pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // Player
    commands.spawn((
        Name::new("Player".to_string()),
        Player,
        Movement {
            free_move: false,
            destination: Vec2::ZERO
        },
        Sprite {
            color: Color::srgb(1.0, 0.75, 0.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            anchor: Anchor::Center,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec2::ZERO)
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
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
    ));
}

pub fn handle_key_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
)
{
    if let Ok(mut velocity) = query.get_single_mut() {
        let mut x_movement_pressed = false;
        let mut y_movement_pressed = false;

        if keys.pressed(KeyCode::KeyD) {
            velocity.0.x = 250.0;
            x_movement_pressed = true;
        }
        if keys.pressed(KeyCode::KeyA) {
            velocity.0.x = -250.0;
            x_movement_pressed = true;
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity.0.y = -250.0;
            y_movement_pressed = true;
        }
        if keys.pressed(KeyCode::KeyW) {
            velocity.0.y = 250.0;
            y_movement_pressed = true;
        }
        if !x_movement_pressed {
            velocity.0.x = 0.0;
        }
        if !y_movement_pressed {
            velocity.0.y = 0.0;
        }
    }
}

pub fn handle_mouse_input(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut query: Query<(&mut Movement), With<Player>>,
)
{
    for (movement) in query.iter_mut() {
        if buttons.pressed(MouseButton::Left) {
            if let Some(mouse_position) = window.single().cursor_position() {
                // Get the camera information
                if let Ok((camera, camera_transform)) = cameras.get_single() {
                    // Convert window coordinates to world coordinates
                    if let Ok(world_position) = camera.viewport_to_world_2d(
                        camera_transform,
                        mouse_position
                    ) {
                        // A world position has been captured, free moving is disabled now while we're targeting a destination.
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
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>
)
{
    for (mut transform, mut velocity) in query.iter_mut() {
        let new_translation_x = transform.translation.x + velocity.0.x * time.delta_secs();
        let new_translation_y = transform.translation.y + velocity.0.y * time.delta_secs();

        transform.translation.x = transform.translation.x.lerp(new_translation_x, 0.95);
        transform.translation.y = transform.translation.y.lerp(new_translation_y, 0.95);
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