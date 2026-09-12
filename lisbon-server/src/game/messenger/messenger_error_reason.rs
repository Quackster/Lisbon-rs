//! Mirrors `net.h4bbo.lisbon.game.messenger.MessengerErrorReason`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessengerErrorReason {
    FriendListFullPendingFriend,
    SenderFriendListFull,
    /// Requests refresh user console in client.
    Concurrency,
}

impl MessengerErrorReason {
    /// Mirrors `getReasonCode`.
    pub fn get_reason_code(&self) -> i32 {
        match self {
            Self::FriendListFullPendingFriend => 1,
            Self::SenderFriendListFull => 2,
            Self::Concurrency => 42,
        }
    }
}
