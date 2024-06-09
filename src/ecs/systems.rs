use bevy_ecs::system::{Query, Res, Resource};
use xactor::Addr;

use crate::ecs::components::{Position, Velocity};
use crate::network::server::TcpServer;

#[derive(Resource)]
pub struct Server(Addr<TcpServer>);

pub fn movement(tcp_server: Res<Server>, mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}