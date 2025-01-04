use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    color::palettes::css::RED,
    math::Vec2,
    prelude::{
        Bundle, Camera2d, Circle, Commands, Component, IntoSystemConfigs, Mesh, Mesh2d, Query,
        ResMut, Transform, With,
    },
    sprite::{ColorMaterial, MeshMaterial2d},
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera))
        .add_systems(Update, (move_ball, project_positions).chain())
        .run();
}

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    position: Position,
}

impl BallBundle {
    fn new() -> Self {
        Self {
            ball: Ball,
            position: Position(Vec2::new(0., 0.)),
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2d::default());
}

const BALL_SIZE: f32 = 5.;

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");

    let mesh = meshes.add(Circle::new(BALL_SIZE));
    let material = materials.add(ColorMaterial::from_color(RED));

    commands.spawn((BallBundle::new(), Mesh2d(mesh), MeshMaterial2d(material)));
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.)
    }
}

const BALL_SPEED: f32 = 1.0;

fn move_ball(mut ball: Query<&mut Position, With<Ball>>) {
    if let Ok(mut position) = ball.get_single_mut() {
        position.0.x += BALL_SPEED;
    }
}
