//! Mirrors `net.h4bbo.lisbon.game.navigator.NavigatorCategory`.
use crate::game::player::player_rank::PlayerRank;
use crate::game::room::room_manager::RoomManager;

#[derive(Clone, Debug)]
pub struct NavigatorCategory {
    id: i32,
    parent_id: i32,
    name: String,
    public_spaces: bool,
    allow_trading: bool,
    minimum_role_access: PlayerRank,
    minimum_role_set_flat: PlayerRank,
    is_node: bool,
}

impl NavigatorCategory {
    pub fn new(
        id: i32,
        parent_id: i32,
        name: String,
        public_spaces: bool,
        allow_trading: bool,
        minimum_role_access: PlayerRank,
        minimum_role_set_flat: PlayerRank,
        is_node: bool,
    ) -> Self {
        Self {
            id,
            parent_id,
            name,
            public_spaces,
            allow_trading,
            minimum_role_access,
            minimum_role_set_flat,
            is_node,
        }
    }

    /// Mirrors `getCurrentVisitors()`.
    pub fn get_current_visitors(&self) -> i32 {
        let mut current_visitors = 0;

        for room in RoomManager::get_instance().get_rooms() {
            let room = room.lock();

            if room.get_data().get_category_id() == self.id {
                current_visitors += room.get_data().get_visitors_now();
            }
        }

        current_visitors
    }

    /// Mirrors `getMaxVisitors()`.
    pub fn get_max_visitors(&self) -> i32 {
        let mut max_visitors = 0;

        for room in RoomManager::get_instance().get_rooms() {
            let room = room.lock();

            if room.get_data().get_category_id() == self.id {
                max_visitors += room.get_data().get_visitors_max();
            }
        }

        max_visitors
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getParentId()`.
    pub fn get_parent_id(&self) -> i32 {
        self.parent_id
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `isPublicSpaces()`.
    pub fn is_public_spaces(&self) -> bool {
        self.public_spaces
    }

    /// Mirrors `hasAllowTrading()`.
    pub fn has_allow_trading(&self) -> bool {
        self.allow_trading
    }

    /// Mirrors `getMinimumRoleAccess()`.
    pub fn get_minimum_role_access(&self) -> PlayerRank {
        self.minimum_role_access
    }

    /// Mirrors `getMinimumRoleSetFlat()`.
    pub fn get_minimum_role_set_flat(&self) -> PlayerRank {
        self.minimum_role_set_flat
    }

    /// Mirrors `isNode()`.
    pub fn is_node(&self) -> bool {
        self.is_node
    }
}
