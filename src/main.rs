use bevy::prelude::*;
// Debugging
mod debug;
use debug::DebugPlugin;
// Movement
mod movement;
use movement::MovementPlugin;
// Camera
mod camera;
use camera::CameraPlugin;
// Spaceship
mod spaceship;
use spaceship::SpaceshipPlugin;

fn main() {
  App::new()
    // Bevy Builtins
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::linear_rgb(0.1, 0.0, 0.15)))
    .insert_resource(AmbientLight {
      color: Color::default(),
      brightness: 0.75,
      affects_lightmapped_meshes: true,
    })
    // Custom:
    .add_plugins(CameraPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(DebugPlugin)
    .run();
}
