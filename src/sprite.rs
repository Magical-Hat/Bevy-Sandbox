use bevy::prelude::*;
use bevy::sprite::Anchor;

const GROUND_LEVEL: f32 = -100.0;
const PLAYER_X: f32 = -300.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Velocity(Vec2);


pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // Player
    commands.spawn((
        Player,
        Sprite {
            color: Color::srgb(0.5, 1.0, 0.5),
            custom_size: Some(Vec2::new(30.0, 50.0)),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec2::ZERO),
    ));

    // Ground
    commands.spawn((
        Ground,
        Sprite {
           color: Color::srgb(0.5, 0.5, 0.5),
           custom_size: Some(Vec2::new(800.0, 10.0)),
           anchor: Anchor::TopLeft,
           ..default()
        },
        Transform::from_xyz(-400.0, GROUND_LEVEL, 0.0),
    ));
}

pub fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity), With<Player>>,
)
{
    if let Ok((mut velocity)) = query.get_single_mut() {
        if keys.pressed(KeyCode::KeyD) {
            velocity.0.x = 150.0;
        }
        else if keys.pressed(KeyCode::KeyA) {
            velocity.0.x = -150.0;
        }
        else {
            velocity.0.x = 0.0;
        }
    }
}

pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>
)
{
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_secs();
    }
}