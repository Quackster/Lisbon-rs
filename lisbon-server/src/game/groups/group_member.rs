//! Mirrors `net.h4bbo.lisbon.game.groups.GroupMember`.

use crate::game::groups::group_member_rank::GroupMemberRank;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_manager::PlayerManager;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GroupMember {
    user_id: i32,
    group_id: i32,
    is_pending: bool,
    member_rank: Option<GroupMemberRank>,
    // The Java field is a direct `PlayerDetails`; the `Box` breaks the
    // `PlayerDetails.groupMember` <-> `GroupMember.user` size cycle.
    user: Option<Box<PlayerDetails>>,
}

impl GroupMember {
    /// Mirrors the `GroupMember(int, int, boolean, int)` constructor.
    pub fn new(user_id: i32, group_id: i32, is_pending: bool, member_rank: i32) -> Self {
        Self {
            user_id,
            group_id,
            is_pending,
            member_rank: GroupMemberRank::get_by_rank_id(member_rank),
            user: PlayerManager::get_instance()
                .get_player_data_by_id(user_id)
                .map(Box::new),
        }
    }

    /// Mirrors `getUserId()`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `getUser()`.
    pub fn get_user(&self) -> Option<&PlayerDetails> {
        self.user.as_deref()
    }

    /// Mirrors `getGroupId()`.
    pub fn get_group_id(&self) -> i32 {
        self.group_id
    }

    /// Mirrors `isPending()`.
    pub fn is_pending(&self) -> bool {
        self.is_pending
    }

    /// Mirrors `isFavourite(int)`.
    pub fn is_favourite(&self, group_id: i32) -> bool {
        self.user
            .as_ref()
            .is_some_and(|user| user.get_favourite_group_id() == group_id)
    }

    /// Mirrors `getMemberRank()`.
    pub fn get_member_rank(&self) -> Option<GroupMemberRank> {
        self.member_rank
    }
}
