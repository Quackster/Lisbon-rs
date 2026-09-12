//! Mirrors `net.h4bbo.lisbon.game.catalogue.voucher.VoucherRedeemStatus`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VoucherRedeemStatus {
    Success,
    Failure,
    FailureNewAccount,
}
