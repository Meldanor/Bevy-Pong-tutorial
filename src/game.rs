use bevy::{
    audio::Volume,
    color::palettes::css::{BLACK, BLUE, GREEN, RED},
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::Collider,
    render::RapierDebugRenderPlugin,
};

use crate::positions::Position;
pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<PongSounds>()
            .add_event::<Scored>()
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(
                Startup,
                (
                    spawn_ball,
                    spawn_paddles,
                    spawn_gutters,
                    spawn_scoreboards,
                    load_assets,
                ),
            )
            .add_systems(Update, (update_scoreboards, handle_player_input).chain())
            .add_systems(
                FixedUpdate,
                (
                    move_ball,
                    detect_scoring,
                    update_score,
                    reset_ball,
                    move_ai,
                    move_paddles,
                    // handle_collisions,
                )
                    .chain(),
            );
    }
}

#[derive(Resource, Default)]
struct PongSounds {
    on_hit: Handle<AudioSource>,
    on_score: Handle<AudioSource>,
}

fn load_assets(asset_server: Res<AssetServer>, mut pong_sounds: ResMut<PongSounds>) {
    pong_sounds.on_hit = asset_server.load("audio/sfx/8-Bit - Coin Drop 001.wav");
    pong_sounds.on_score = asset_server.load("audio/sfx/Items - Pickup - Collect 049.wav");
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    collider: Collider,
    velocity: Velocity,
    position: Position,
}

impl BallBundle {
    fn new(velocity: Vec2) -> Self {
        Self {
            ball: Ball,
            collider: Collider::ball(BALL_SIZE),
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
        BallBundle::new(Vec2::new(1., 1.)),
        Mesh2d(mesh),
        MeshMaterial2d(material),
    ));
}

const BALL_SPEED: f32 = 4.0;

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
    collider: Collider,
    position: Position,
    velocity: Velocity,
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            collider: Collider::cuboid(PADDLE_WIDTH / 2., PADDLE_HEIGHT / 2.),
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
    let window = match window.get_single() {
        Ok(window) => window,
        Err(_) => return,
    };
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

// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
// enum Collision {
//     Left,
//     Right,
//     Top,
//     Bottom,
// }

// fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
//     if !ball.intersects(&wall) {
//         return None;
//     }
//     let closest_point = wall.closest_point(ball.center);
//     let offset = ball.center - closest_point;

//     let side = if offset.x.abs() > offset.y.abs() {
//         if offset.x < 0. {
//             Collision::Left
//         } else {
//             Collision::Right
//         }
//     } else if offset.y > 0. {
//         Collision::Top
//     } else {
//         Collision::Bottom
//     };
//     Some(side)
// }

// fn handle_collisions(
//     mut commands: Commands,
//     mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
//     other_things: Query<(&Position, &Shape), Without<Ball>>,
//     pong_sounds: Res<PongSounds>,
// ) {
//     let (mut ball_velocity, ball_position, ball_shape) = match ball.get_single_mut() {
//         Ok(tuple) => tuple,
//         Err(_) => return,
//     };
//     for (position, shape) in &other_things {
//         if let Some(collision) = collide_with_side(
//             BoundingCircle::new(ball_position.0, ball_shape.0.x),
//             Aabb2d::new(position.0, shape.0 / 2.),
//         ) {
//             commands.spawn((
//                 AudioPlayer::new(pong_sounds.on_hit.clone_weak()),
//                 PlaybackSettings::ONCE.with_volume(Volume::new(0.5)),
//             ));
//             match collision {
//                 Collision::Left => ball_velocity.0.x *= -1.,
//                 Collision::Right => ball_velocity.0.x *= -1.,
//                 Collision::Top => ball_velocity.0.y *= -1.,
//                 Collision::Bottom => ball_velocity.0.y *= -1.,
//             }
//         }
//     }
// }

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

fn move_ai(
    mut ai: Query<(&mut Velocity, &Position), With<Ai>>,
    ball: Query<&Position, With<Ball>>,
) {
    if let Ok((mut velocity, position)) = ai.get_single_mut() {
        if let Ok(ball_position) = ball.get_single() {
            let a_to_b = ball_position.0 - position.0;
            velocity.0.y = a_to_b.y.signum();
        }
    }
}

const GUTTER_HEIGHT: f32 = 20.;

#[derive(Component)]
struct Gutter;

#[derive(Bundle)]
struct GutterBundle {
    gutter: Gutter,
    collider: Collider,
    position: Position,
}

impl GutterBundle {
    fn new(x: f32, y: f32, width: f32) -> Self {
        Self {
            gutter: Gutter,
            collider: Collider::cuboid(width / 2., GUTTER_HEIGHT / 2.),
            position: Position(Vec2::new(x, y)),
        }
    }
}

fn spawn_gutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let window = match window.get_single() {
        Ok(window) => window,
        Err(_) => return,
    };
    let window_width = window.resolution.width();
    let window_heigth = window.resolution.height();

    let top_gutter_y = window_heigth / 2. - GUTTER_HEIGHT / 2.;
    let bottom_gutter_y = -window_heigth / 2. + GUTTER_HEIGHT / 2.;

    let top_gutter = GutterBundle::new(0., top_gutter_y, window_width);
    let bottom_gutter = GutterBundle::new(0., bottom_gutter_y, window_width);

    let mesh = meshes.add(Rectangle::new(window_width, GUTTER_HEIGHT));
    let color = materials.add(ColorMaterial::from_color(BLACK));

    commands.spawn((
        top_gutter,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(color.clone()),
    ));
    commands.spawn((
        bottom_gutter,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(color.clone()),
    ));
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player>>,
) {
    let mut paddle_velocity = match paddle.get_single_mut() {
        Ok(paddle) => paddle,
        Err(_) => return,
    };

    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        paddle_velocity.0.y = 1.;
    } else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        paddle_velocity.0.y = -1.;
    } else {
        paddle_velocity.0.y = 0.;
    }
}

const PADDLE_SPEED: f32 = 5.;
fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>,
) {
    let window = match window.get_single() {
        Ok(window) => window,
        Err(_) => return,
    };
    let window_heigth = window.resolution.height();
    let max_y = window_heigth / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.;
    for (mut position, velocity) in &mut paddle {
        let new_position = position.0 + velocity.0 * PADDLE_SPEED;
        if new_position.y.abs() < max_y {
            position.0 = new_position;
        }
    }
}

enum Scorer {
    Ai,
    Player,
}

#[derive(Event)]
struct Scored(Scorer);

#[derive(Resource, Default)]
struct Score {
    player: u32,
    ai: u32,
}

fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<Scored>,
) {
    let window = match window.get_single() {
        Ok(window) => window,
        Err(_) => return,
    };
    let ball = match ball.get_single_mut() {
        Ok(ball) => ball,
        Err(_) => return,
    };
    let window_width = window.resolution.width();
    if ball.0.x > window_width / 2. {
        events.send(Scored(Scorer::Ai));
    } else if ball.0.x < -window_width / 2. {
        events.send(Scored(Scorer::Player));
    }
}

fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    let (mut ball_position, mut ball_velocity) = match ball.get_single_mut() {
        Ok(ball) => ball,
        Err(_) => return,
    };
    for event in events.read() {
        ball_position.0 = Vec2::new(0., 0.);
        match event.0 {
            Scorer::Ai => {
                ball_velocity.0 = Vec2::new(-1., -1.);
            }
            Scorer::Player => {
                ball_velocity.0 = Vec2::new(1., 1.);
            }
        }
    }
}

fn update_score(
    mut score: ResMut<Score>,
    mut events: EventReader<Scored>,
    mut commands: Commands,
    pong_sounds: Res<PongSounds>,
) {
    for event in events.read() {
        commands.spawn((
            AudioPlayer::new(pong_sounds.on_score.clone_weak()),
            PlaybackSettings::ONCE.with_volume(Volume::new(0.3)),
        ));
        match event.0 {
            Scorer::Ai => score.ai += 1,
            Scorer::Player => score.player += 1,
        }
        println!("Score: {} - {}", score.player, score.ai);
    }
}

#[derive(Component)]
struct PlayerScore;

#[derive(Component)]
struct AiScore;

fn update_scoreboards(
    mut player_score: Query<&mut Text, With<PlayerScore>>,
    mut ai_score: Query<&mut Text, (With<AiScore>, Without<PlayerScore>)>,
    score: Res<Score>,
) {
    if !score.is_changed() {
        return;
    }

    if let Ok(mut player_score) = player_score.get_single_mut() {
        player_score.0 = score.player.to_string();
    }
    if let Ok(mut ai_score) = ai_score.get_single_mut() {
        ai_score.0 = score.ai.to_string();
    }
}

fn spawn_scoreboards(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(15.0),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text::new("0"),
                TextColor::WHITE,
                TextFont::from_font_size(72.),
                TextLayout::new_with_justify(JustifyText::Center),
                PlayerScore,
            ));
        });

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(15.0),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text::new("0"),
                TextColor::WHITE,
                TextFont::from_font_size(72.),
                TextLayout::new_with_justify(JustifyText::Center),
                AiScore,
            ));
        });
}
