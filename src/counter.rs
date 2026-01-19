use actix::{Actor, Context, Handler, Message};

#[derive(Default)]
pub struct Counter {
    count: usize,
}

impl Actor for Counter {
    type Context = Context<Self>;
}

pub struct Increment;
pub struct Decrement;
pub struct Get;

impl Message for Increment {
    type Result = ();
}

impl Message for Decrement {
    type Result = ();
}

impl Message for Get {
    type Result = usize;
}

impl Handler<Increment> for Counter {
    type Result = ();

    fn handle(&mut self, _msg: Increment, _ctx: &mut Self::Context) {
        self.count += 1;
    }
}

impl Handler<Decrement> for Counter {
    type Result = ();

    fn handle(&mut self, _msg: Decrement, _ctx: &mut Self::Context) {
        self.count -= 1;
    }
}

impl Handler<Get> for Counter {
    type Result = usize;

    fn handle(&mut self, _msg: Get, _ctx: &mut Self::Context) -> Self::Result {
        self.count
    }
}
