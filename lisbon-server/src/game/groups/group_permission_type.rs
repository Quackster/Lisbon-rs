//! Mirrors `net.h4bbo.lisbon.game.groups.GroupPermissionType`.

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize)]
pub enum GroupPermissionType {
    AdminOnly,
    MemberOnly,
    Everyone,
}

impl GroupPermissionType {
    /// Const array mirroring `values()`.
    pub const ALL: [GroupPermissionType; 3] = [
        GroupPermissionType::AdminOnly,
        GroupPermissionType::MemberOnly,
        GroupPermissionType::Everyone,
    ];

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        match self {
            GroupPermissionType::AdminOnly => 2,
            GroupPermissionType::MemberOnly => 1,
            GroupPermissionType::Everyone => 0,
        }
    }

    /// Mirrors `getById(int)` (returns `None` on an unknown id; Java
    /// returns `null`).
    pub fn get_by_id(id: i32) -> Option<GroupPermissionType> {
        Self::ALL
            .iter()
            .copied()
            .find(|permission_type| permission_type.get_id() == id)
    }
}
