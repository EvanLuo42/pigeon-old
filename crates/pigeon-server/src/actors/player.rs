use actix::{Actor, Context};
use crate::actors::game::GameActor;

pub struct PlayerActor {
    game: GameActor
}

impl Actor for PlayerActor {
    type Context = Context<Self>;
}