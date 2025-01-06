use bevy::app::App;
use bevy::prelude::*;

use camera::PongCameraPlugin;
use game::PongPlugin;
use positions::PongPositionsPlugin;
use window::PongWindowPlugin;

mod camera;
mod game;
mod positions;
mod window;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PongCameraPlugin,
            PongWindowPlugin,
            PongPositionsPlugin,
            PongPlugin,
        ));
    }
}
