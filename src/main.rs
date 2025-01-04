use bevy::{
    app::{App, Startup, Update},
    asset::Assets,
    color::palettes::css::{BLUE, GREEN, RED},
    math::{
        bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
        Vec2,
    },
    prelude::{
        Bundle, Camera2d, Circle, Commands, Component, IntoSystemConfigs, Mesh, Mesh2d, Query,
        Rectangle, ResMut, Transform, With, Without,
    },
    sprite::{ColorMaterial, MeshMaterial2d},
    window::Window,
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_paddles, spawn_camera))
        .add_systems(
            Update,
            (move_ball, handle_collisions, project_positions).chain(),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2d::default());
}

#[derive(Component)]
struct Position(Vec2);
#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Shape(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    shape: Shape,
    velocity: Velocity,
    position: Position,
}

impl BallBundle {
    fn new(velocity: Vec2) -> Self {
        Self {
            ball: Ball,
            shape: Shape(Vec2 {
                x: BALL_SIZE,
                y: BALL_SIZE,
            }),
            velocity: Velocity(velocity),
            position: Position(Vec2::new(0., 0.)),
        }
    }
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

    commands.spawn((
        BallBundle::new(Vec2::new(1., 0.)),
        Mesh2d(mesh),
        MeshMaterial2d(material),
    ));
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.)
    }
}

const BALL_SPEED: f32 = 1.0;

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

#[derive(Component)]
struct Paddle;

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    shape: Shape,
    position: Position,
    velocity: Velocity,
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            shape: Shape(Vec2 {
                x: PADDLE_WIDTH,
                y: PADDLE_HEIGHT,
            }),
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2 { x: 0., y: 0. }),
        }
    }
}

const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning paddles...");
    if window.get_single().is_err() {
        return;
    }
    let window = window.get_single().unwrap();
    let window_width = window.resolution.width();
    let padding = 50.;
    let right_paddle_x = window_width / 2. - padding;
    let left_paddle_x = -window_width / 2. + padding;

    let mesh = meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    let player_color = materials.add(ColorMaterial::from_color(BLUE));
    let ai_color = materials.add(ColorMaterial::from_color(GREEN));

    commands.spawn((
        Player,
        PaddleBundle::new(right_paddle_x, -0.),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(player_color),
    ));

    commands.spawn((
        Ai,
        PaddleBundle::new(left_paddle_x, -0.),
        Mesh2d(mesh),
        MeshMaterial2d(ai_color),
    ));
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }
    let closest_point = wall.closest_point(ball.center);
    let offset = ball.center - closest_point;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };
    Some(side)
}

fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &other_things {
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(position.0, shape.0 / 2.),
            ) {
                match collision {
                    Collision::Left => ball_velocity.0.x *= -1.,
                    Collision::Right => ball_velocity.0.x *= -1.,
                    Collision::Top => ball_velocity.0.y *= -1.,
                    Collision::Bottom => ball_velocity.0.y *= -1.,
                }
            }
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;
