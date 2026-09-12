//! Mirrors `net.h4bbo.lisbon.messages.outgoing.purse.VOUCHER_REDEEM_ERROR`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RedeemError {
    TechnicalError,
    Invalid,
    ProductDeliveryFailed,
    WebOnly,
}

impl RedeemError {
    /// Mirrors `getErrorCode()`.
    pub fn get_error_code(&self) -> i32 {
        match self {
            RedeemError::TechnicalError => 0,
            RedeemError::Invalid => 1,
            RedeemError::ProductDeliveryFailed => 2,
            RedeemError::WebOnly => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct VOUCHER_REDEEM_ERROR {
    error: RedeemError,
}

impl VOUCHER_REDEEM_ERROR {
    /// Mirrors the `VOUCHER_REDEEM_ERROR(RedeemError)` constructor.
    pub fn new(error: RedeemError) -> Self {
        Self { error }
    }
}

impl MessageComposer for VOUCHER_REDEEM_ERROR {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.error.get_error_code());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        213 // "CU"
    }
}
