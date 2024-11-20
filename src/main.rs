mod sand;

use bevy::{prelude::*};
use sand::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RectanglePlugin)
        .run();
}
