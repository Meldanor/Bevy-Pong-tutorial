use bevy::prelude::*;

pub struct PongCameraPlugin;

impl Plugin for PongCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
