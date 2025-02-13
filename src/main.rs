mod player;

use bevy::{prelude::*, window::PresentMode};
use bevy::window::ExitCondition;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Playground".into(),
                        resolution: (1280., 720.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    exit_condition: ExitCondition::OnPrimaryClosed,
                    close_when_requested: true,
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.1)))
        .add_systems(Startup, player::setup)
        .add_systems(Update, (player::handle_input, player::update_movement, player::update_camera).chain())
        .run();
}