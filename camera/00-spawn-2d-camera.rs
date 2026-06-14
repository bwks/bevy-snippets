use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add setup system to application
        .add_systems(Startup, setup)
        .run();
}

// Create setup system
fn setup(mut commands: Commands) {
    // spawns 2d camera.
    commands.spawn(Camera2d);
}
