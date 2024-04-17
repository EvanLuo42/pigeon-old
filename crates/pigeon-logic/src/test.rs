use actix::{Actor, Context, Handler, Message};

pub struct TestActor;

impl Actor for TestActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Vec<u8>")]
pub struct Print(Vec<u8>);

impl Handler<Print> for TestActor {
    type Result = ();

    fn handle(&mut self, _msg: Print, _ctx: &mut Self::Context) -> Self::Result {
        println!("Print");
    }
}
