//! Mirrors `net.h4bbo.lisbon.game.ban.BanType`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize)]
pub enum BanType {
    UserId,
    MachineId,
    IpAddress,
}
