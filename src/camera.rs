use bevy::prelude::*;
use bevy::camera::{Camera3d, Projection, PerspectiveProjection};
use std::f32::consts::FRAC_PI_4;

const CAMERA_DISTANCE: f32 = 800.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, spawn_camera);
  }
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn((
    Camera3d::default(),
    Camera::default(),
    Projection::Perspective(PerspectiveProjection {
      fov: FRAC_PI_4,
      near: 0.1,
      far: 1000.,
      ..default()
    }),
    Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0)
      .looking_at(Vec3::ZERO, Vec3::Y),
    GlobalTransform::default(),
  ));
}
