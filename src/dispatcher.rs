use {
    crate::actor_messages::{Join, Reply},
    actix::prelude::*,
};

// DispatcherActor
// - receives a telegram join or telegram reply message and dispatches to interrogators
// - acts as a first-level filter to see only relevant TG messages and keep a map of currently active
//   interrogations
// i.e. for REPLY: if map.entry(reply.chat, reply.user) then dispatch to it, otherwise ignore
//      for JOIN: create actors in (chat, join.user) pairs and insert into map
// @todo also handle https://docs.rs/teloxide/0.5.2/teloxide/types/enum.UpdateKind.html#variant.ChatMember ?
#[derive(Default)]
struct Dispatcher {}

impl Actor for Dispatcher {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        todo!()
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        todo!()
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        todo!()
    }

    fn start(self) -> Addr<Self>
    where
        Self: Actor<Context = Context<Self>>,
    {
        todo!()
    }

    fn start_default() -> Addr<Self>
    where
        Self: Actor<Context = Context<Self>> + Default,
    {
        todo!()
    }

    fn start_in_arbiter<F>(wrk: &ArbiterHandle, f: F) -> Addr<Self>
    where
        Self: Actor<Context = Context<Self>>,
        F: FnOnce(&mut Context<Self>) -> Self + Send + 'static,
    {
        todo!()
    }

    fn create<F>(f: F) -> Addr<Self>
    where
        Self: Actor<Context = Context<Self>>,
        F: FnOnce(&mut Context<Self>) -> Self,
    {
        todo!()
    }
}

impl Handler<Reply> for Dispatcher {
    type Result = ();

    fn handle(&mut self, msg: Reply, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
impl Handler<Join> for Dispatcher {
    type Result = ();

    fn handle(&mut self, msg: Join, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
