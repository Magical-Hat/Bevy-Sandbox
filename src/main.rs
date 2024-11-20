mod sand;

use bevy::{color::palettes::basic::PURPLE, prelude::*, sprite::MaterialMesh2dBundle};
use sand::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RectanglePlugin)
        .run();
}
