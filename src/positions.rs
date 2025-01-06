use bevy::prelude::*;

pub struct PongPositionsPlugin;

impl Plugin for PongPositionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, project_positions);
    }
}

#[derive(Component)]
pub struct Position(pub Vec2);

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.)
    }
}
