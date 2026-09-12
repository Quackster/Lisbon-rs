//! Mirrors `net.h4bbo.lisbon.messages.outgoing.tutorial.TUTORS_AVAILABLE`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct TUTORS_AVAILABLE {
    tutors: i32,
}

impl TUTORS_AVAILABLE {
    /// Mirrors the `TUTORS_AVAILABLE(int)` constructor.
    pub fn new(tutors: i32) -> Self {
        Self { tutors }
    }
}

impl MessageComposer for TUTORS_AVAILABLE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.tutors);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        356 // "Ed"
    }
}
