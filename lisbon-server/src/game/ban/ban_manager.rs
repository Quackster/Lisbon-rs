//! Mirrors `net.h4bbo.lisbon.game.ban.BanManager`.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::game::ban::ban_type::BanType;
use crate::game::entity::entity::Entity;
use crate::game::player::player_manager::PlayerManager;
use crate::server::netty::netty_player_network::NettyPlayerNetwork;

pub struct BanManager;

impl BanManager {
    /// Mirrors `disconnectBanAccounts(Map<BanType, String>)`.
    pub fn disconnect_ban_accounts(&self, criteria: &HashMap<BanType, String>) {
        let players = PlayerManager::get_instance().get_active_players();

        for player_arc in &players {
            let player = player_arc.lock();

            if let Some(ban_id) = criteria.get(&BanType::UserId) {
                if let Ok(ban_id) = ban_id.parse::<i32>() {
                    if player.get_details().get_id() == ban_id {
                        player.get_network().disconnect();
                        return;
                    }
                }
            }

            if let Some(ip_address) = criteria.get(&BanType::IpAddress) {
                if NettyPlayerNetwork::get_ip_address(player.get_network()) == *ip_address {
                    player.get_network().disconnect();
                    return;
                }
            }
        }
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static BanManager {
        static INSTANCE: OnceLock<BanManager> = OnceLock::new();
        INSTANCE.get_or_init(|| BanManager)
    }
}
