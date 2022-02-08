mod view;

use bevy::prelude::*;

use bevy_prototype_debug_lines::{DebugLinesPlugin};
use crate::view::ViewMap;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "GTA San Andreas route viewer".to_string(),
            width: 600.,
            height: 600.,
            resizable: true,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::always_in_front())
        .add_plugin(ViewMap)
        .run();
}


