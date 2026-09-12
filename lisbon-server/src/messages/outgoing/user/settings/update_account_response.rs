//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.settings.UPDATE_ACCOUNT_RESPONSE`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResponseType {
    Success,
    IncorrectPassword,
    IncorrectBirthday,
}

impl ResponseType {
    /// Mirrors `getStatusId()`.
    pub fn get_status_id(&self) -> i32 {
        match self {
            ResponseType::Success => 0,
            ResponseType::IncorrectPassword => 1,
            ResponseType::IncorrectBirthday => 2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct UPDATE_ACCOUNT_RESPONSE {
    response_type: ResponseType,
}

impl UPDATE_ACCOUNT_RESPONSE {
    /// Mirrors the `UPDATE_ACCOUNT_RESPONSE(ResponseType)` constructor.
    pub fn new(response_type: ResponseType) -> Self {
        Self { response_type }
    }
}

impl MessageComposer for UPDATE_ACCOUNT_RESPONSE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.response_type.get_status_id());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        169
    }
}
