use {
    crate::actor_messages::{Reply, Timeout},
    actix::prelude::*,
};

// "Reply to me with an answer within timeout"
//
// InterrogatorActor
// - init: remember user, take away user's permissions, print challenge text,
//         create a timer actor, and wait for either timeout or TG reply from DispatcherActor.
// - waiting: either wait timeout happens or we get a response from our user
// - timedout: timeout happened first; kickban user
// - answered: user answered, check if correct - then restore permissions, otherwise kickban
#[derive(Default)]
struct Interrogator {}

impl Actor for Interrogator {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        todo!()
        // start Timer actor - or do it in start()?
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        Running::Stop
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        todo!()
    }

    // fn start(self) -> Addr<Self>
    // where
    //     Self: Actor<Context = Context<Self>>,
    // {
    //     todo!()
    // }

    // fn create<F>(f: F) -> Addr<Self>
    // where
    //     Self: Actor<Context = Context<Self>>,
    //     F: FnOnce(&mut Context<Self>) -> Self,
    // {
    //     todo!()
    // }
}

impl Handler<Timeout> for Interrogator {
    type Result = ();

    fn handle(&mut self, msg: Timeout, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

impl Handler<Reply> for Interrogator {
    type Result = ();

    fn handle(&mut self, msg: Reply, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
