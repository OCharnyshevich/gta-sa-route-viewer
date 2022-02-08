mod view;

use bevy::prelude::*;

use bevy_prototype_debug_lines::{DebugLinesPlugin};
use crate::view::ViewMap;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::always_in_front())
        .add_plugin(ViewMap)
        .run();
}


