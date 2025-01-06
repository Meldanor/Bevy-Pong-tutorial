use bevy::prelude::*;

#[derive(Component)]
pub struct Position(pub Vec2);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Last, project_positions);
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.)
    }
}
