//! Mirrors `net.h4bbo.lisbon.game.groups.GroupMemberRank`.

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
pub enum GroupMemberRank {
    Member,
    Administrator,
    Owner,
}

impl GroupMemberRank {
    /// Const array mirroring `values()`.
    pub const ALL: [GroupMemberRank; 3] = [
        GroupMemberRank::Member,
        GroupMemberRank::Administrator,
        GroupMemberRank::Owner,
    ];

    /// Mirrors `getByRankId(int)` (returns `None` on an unknown rank id;
    /// Java returns `null`).
    pub fn get_by_rank_id(rank_id: i32) -> Option<GroupMemberRank> {
        Self::ALL
            .iter()
            .copied()
            .find(|rank| rank.get_rank_id() == rank_id)
    }

    /// Mirrors `getRankId()`.
    pub fn get_rank_id(&self) -> i32 {
        match self {
            GroupMemberRank::Member => 1,
            GroupMemberRank::Administrator => 2,
            GroupMemberRank::Owner => 3,
        }
    }

    /// Mirrors `getClientRank()`.
    pub fn get_client_rank(&self) -> i32 {
        match self {
            GroupMemberRank::Member => 3,
            GroupMemberRank::Administrator => 2,
            GroupMemberRank::Owner => 1,
        }
    }
}
