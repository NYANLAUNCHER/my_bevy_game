use bevy::prelude::*;
use crate::movement::Velocity;

pub struct SpaceshipPlugin;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 =  Vec3::new(0.0, 0.0, 1.0);

#[derive(Bundle)]
struct SpaceshipBundle {
  model: SceneRoot,
  velocity: Velocity,
  transform: Transform,
}

impl Plugin for SpaceshipPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_spaceship);
  }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(SpaceshipBundle{
    model: SceneRoot(asset_server.load("Spaceship.glb#Scene0")),
    velocity: Velocity {
      value: STARTING_VELOCITY,
    },
    transform: Transform::from_translation(STARTING_TRANSLATION),
  });
}
