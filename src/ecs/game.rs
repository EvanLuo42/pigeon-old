use std::time::Duration;

use async_trait::async_trait;
use bevy_ecs::prelude::{Schedule, World};
use xactor::{Actor, Addr, Context, Handler, message};

use crate::ecs::systems::movement;
use crate::network::server::TcpServer;

pub struct Game {
    pub world: World,
    pub schedule: Schedule,
    pub tcp_server: Option<Addr<TcpServer>>
}

impl Game {
    pub fn new() -> Self {
        let mut schedule = Schedule::default();
        schedule.add_systems(movement);
        Game {
            world: World::default(),
            schedule,
            tcp_server: None
        }
    }
}

#[async_trait]
impl Actor for Game {
    async fn started(&mut self, ctx: &mut Context<Self>) -> anyhow::Result<()> {
        ctx.send_interval(Start, Duration::from_secs_f32(0.1));
        Ok(())
    }
}

#[message]
#[derive(Clone)]
pub struct Start;

#[async_trait]
impl Handler<Start> for Game {
    async fn handle(&mut self, ctx: &mut Context<Self>, msg: Start) {
        self.schedule.run(&mut self.world);
    }
}

#[message]
pub struct InitializeGame {
    pub tcp_server: Addr<TcpServer>
}

#[async_trait]
impl Handler<InitializeGame> for Game {
    async fn handle(&mut self, ctx: &mut Context<Self>, msg: InitializeGame) {
        self.tcp_server = Some(msg.tcp_server)
    }
}