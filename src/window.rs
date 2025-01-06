use bevy::prelude::*;
pub struct PongWindowPlugin;

impl Plugin for PongWindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pong in Bevy".into(),
                    resizable: false,
                    resolution: (800., 600.).into(),
                    desired_maximum_frame_latency: core::num::NonZero::new(1u32),

                    ..default()
                }),

                ..default()
            }));
    }
}
