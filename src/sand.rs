use bevy::{color::palettes::basic::PURPLE, prelude::*};
use bevy::window::{PrimaryWindow, WindowResolution};

#[derive(Component)]
struct PrimaryCamera;

pub fn draw_2d_sprite(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PrimaryCamera,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::from(PURPLE),
                custom_size: Some(Vec2::splat(128.0)), // Set the size of the sprite
                ..default()
            },
            transform: Transform::default(),
            ..default()
        },
        Moving, // Tag component to identify the moving entity
        Speed(50.0),
    ));
}

pub fn draw_2d_sprite_where_clicked(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>
)
{
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();

        if let Some(position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::from(PURPLE),
                        custom_size: Some(Vec2::splat(128.0)), // Set the size of the sprite
                        ..default()
                    },
                    transform: Transform::from_xyz(position.x, position.y, 0.0),
                        ..default()
                },
                Moving, // Tag component to identify the moving entity
                Speed(50.0),
            ));
        } else { return; };
    }
}

// Tag component for moving entities
#[derive(Component)]
struct Moving;

#[derive(Component)]
struct Speed(f32);

pub fn move_2d_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed, &Sprite), With<Moving>>,
    mut window: Query<&mut Window>
) {
    for (mut transform, speed, sprite) in query.iter_mut() {
        transform.translation.x += speed.0 * time.delta_seconds(); // Adjust speed as needed
        if (transform.translation.x - sprite.custom_size.unwrap().x / 2.0) > window.single().resolution.width() / 2.0 {
            transform.translation.x = -window.single().resolution.width() / 2.0 - sprite.custom_size.unwrap().x / 2.0;
        }
    }
}

fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Speed, With<Moving>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        for mut speed in &mut query {
            speed.0 *= 2.0;
        }
    }

    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
    }

    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down
    }

    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Middle]) {
        // Either the left or the middle (wheel) button was just pressed
    }
}

pub struct RectanglePlugin;

impl Plugin for RectanglePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_2d_sprite)
            .add_systems(Update, (move_2d_sprite, draw_2d_sprite_where_clicked)); // Add the movement system to the update stage
    }
}
