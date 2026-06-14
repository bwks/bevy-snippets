use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add camera system to application
        .add_systems(Startup, camera_system)
        .run();
}

//  Camera system
fn camera_system(mut commands: Commands) {
    // spawns 2d camera.
    commands.spawn(Camera2d);
}
