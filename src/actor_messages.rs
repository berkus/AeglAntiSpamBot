use actix::Message;

pub(crate) struct Join;

pub(crate) struct Reply;

pub(crate) struct Timeout;

impl Message for Join {
    type Result = ();
}
impl Message for Reply {
    type Result = ();
}
impl Message for Timeout {
    type Result = ();
}
