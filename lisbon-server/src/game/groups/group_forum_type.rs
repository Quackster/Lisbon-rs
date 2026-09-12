//! Mirrors `net.h4bbo.lisbon.game.groups.GroupForumType`.

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize)]
pub enum GroupForumType {
    Public,
    Private,
}

impl GroupForumType {
    /// Const array mirroring `values()`.
    pub const ALL: [GroupForumType; 2] = [
        GroupForumType::Public,
        GroupForumType::Private,
    ];

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        match self {
            GroupForumType::Public => 0,
            GroupForumType::Private => 1,
        }
    }

    /// Mirrors `getById(int)` (returns `None` on an unknown id; Java
    /// returns `null`).
    pub fn get_by_id(id: i32) -> Option<GroupForumType> {
        Self::ALL
            .iter()
            .copied()
            .find(|forum_type| forum_type.get_id() == id)
    }
}
