//! Mirrors `net.h4bbo.lisbon.game.player.PlayerRank`.

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum PlayerRank {
    Rankless,
    Normal,
    CommunityManager,
    Guide,
    Hobba,
    SuperHobba,
    Moderator,
    Administrator,
}

impl PlayerRank {
    pub const ALL: [PlayerRank; 8] = [
        PlayerRank::Rankless,
        PlayerRank::Normal,
        PlayerRank::CommunityManager,
        PlayerRank::Guide,
        PlayerRank::Hobba,
        PlayerRank::SuperHobba,
        PlayerRank::Moderator,
        PlayerRank::Administrator,
    ];

    /// Mirrors `name()` (the Java enum constant name).
    pub fn name(&self) -> &'static str {
        match self {
            PlayerRank::Rankless => "RANKLESS",
            PlayerRank::Normal => "NORMAL",
            PlayerRank::CommunityManager => "COMMUNITY_MANAGER",
            PlayerRank::Guide => "GUIDE",
            PlayerRank::Hobba => "HOBBA",
            PlayerRank::SuperHobba => "SUPERHOBBA",
            PlayerRank::Moderator => "MODERATOR",
            PlayerRank::Administrator => "ADMINISTRATOR",
        }
    }

    /// Mirrors `getRankId`.
    pub fn rank_id(&self) -> i32 {
        match self {
            PlayerRank::Rankless => 0,
            PlayerRank::Normal => 1,
            PlayerRank::CommunityManager => 2,
            PlayerRank::Guide => 3,
            PlayerRank::Hobba => 4,
            PlayerRank::SuperHobba => 5,
            PlayerRank::Moderator => 6,
            PlayerRank::Administrator => 7,
        }
    }

    /// Mirrors `getRankForId`.
    pub fn get_rank_for_id(rank_id: i32) -> Option<PlayerRank> {
        Self::ALL.iter().copied().find(|rank| rank.rank_id() == rank_id)
    }
}
