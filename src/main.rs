use bevy::{
    app::{App, Startup},
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, hello_world)
        .run();
}

fn hello_world() {
    println!("Hello world!");
}
