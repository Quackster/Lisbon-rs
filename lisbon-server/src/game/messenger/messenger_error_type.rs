//! Mirrors `net.h4bbo.lisbon.game.messenger.MessengerErrorType`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessengerErrorType {
    TargetFriendListFull,
    TargetDoesNotAccept,
    FriendRequestNotFound,
    BuddyRemoveError,
    FriendListFull,
    ConcurrencyError,
}

impl MessengerErrorType {
    /// Mirrors `getErrorCode`.
    pub fn get_error_code(&self) -> i32 {
        match self {
            Self::TargetFriendListFull => 2,
            Self::TargetDoesNotAccept => 3,
            Self::FriendRequestNotFound => 4,
            Self::BuddyRemoveError => 37,
            Self::FriendListFull => 39,
            Self::ConcurrencyError => 42,
        }
    }
}
