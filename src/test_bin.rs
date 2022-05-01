use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            canvas: Some("#bevy".to_string()),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
