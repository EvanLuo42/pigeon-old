use bevy_ecs::system::Query;

use crate::ecs::components::{Position, Velocity};

pub fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}