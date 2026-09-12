//! Mirrors `org.alexdev.http.game.housekeeping.HousekeepingManager`.

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Serialize;

use lisbon_server::game::player::player_rank::PlayerRank;

pub struct HousekeepingManager {
    permissions: HashMap<String, PlayerRank>,
}

impl HousekeepingManager {
    /// Mirrors the `HousekeepingManager()` constructor.
    fn new() -> Self {
        let mut permissions = HashMap::new();
        permissions.insert("root/login".to_string(), PlayerRank::Moderator);
        permissions.insert("transaction/lookup".to_string(), PlayerRank::Moderator);
        permissions.insert("marketplace/log_check".to_string(), PlayerRank::Moderator);
        permissions.insert("marketplace/user_log".to_string(), PlayerRank::Moderator);
        permissions.insert("bans".to_string(), PlayerRank::Moderator);
        permissions.insert("user/search".to_string(), PlayerRank::Administrator);
        permissions.insert("user/edit".to_string(), PlayerRank::Administrator);
        permissions.insert("user/create".to_string(), PlayerRank::Administrator);
        permissions.insert("articles/create".to_string(), PlayerRank::Moderator);
        permissions.insert("articles/edit_any".to_string(), PlayerRank::Administrator);
        permissions.insert("articles/edit_own".to_string(), PlayerRank::Moderator);
        permissions.insert("articles/delete_any".to_string(), PlayerRank::Administrator);
        permissions.insert("articles/delete_own".to_string(), PlayerRank::Moderator);
        permissions.insert("room_ads".to_string(), PlayerRank::Administrator);
        permissions.insert("room_badges".to_string(), PlayerRank::CommunityManager);
        permissions.insert("configuration".to_string(), PlayerRank::Administrator);
        permissions.insert("infobus".to_string(), PlayerRank::CommunityManager);
        permissions.insert("infobus/delete_any".to_string(), PlayerRank::Administrator);
        permissions.insert("infobus/delete_own".to_string(), PlayerRank::CommunityManager);
        permissions.insert("catalogue/edit_frontpage".to_string(), PlayerRank::CommunityManager);
        permissions.insert("user/imitate".to_string(), PlayerRank::Administrator);
        permissions.insert("user/matches".to_string(), PlayerRank::Administrator);
        permissions.insert("badges".to_string(), PlayerRank::CommunityManager);

        Self { permissions }
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static HousekeepingManager {
        static INSTANCE: OnceLock<HousekeepingManager> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Mirrors `hasPermission(PlayerRank, String)`.
    pub fn has_permission(&self, rank: PlayerRank, permission: &str) -> bool {
        if let Some(permissible_rank) = self.permissions.get(permission) {
            return rank.rank_id() >= permissible_rank.rank_id();
        }

        false
    }
}

/// The template context materialises the singleton as its permission map
/// (the template cannot call `hasPermission` on a JSON value; the map is
/// the serialisable state the Java template could reach).
impl Serialize for HousekeepingManager {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.permissions.serialize(serializer)
    }
}
