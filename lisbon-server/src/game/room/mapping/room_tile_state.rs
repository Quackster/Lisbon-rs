//! Mirrors `net.h4bbo.lisbon.game.room.mapping.RoomTileState`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoomTileState {
    Open,
    Closed,
}
