//! Mirrors `net.h4bbo.lisbon.game.moderation.actions.ModeratorBanUserAction`.

use std::collections::HashMap;

use crate::dao::mysql::ban_dao::BanDao;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::ban::ban_manager::BanManager;
use crate::game::ban::ban_type::BanType;
use crate::game::commands::command_manager::CommandManager;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::game_scheduler::GameScheduler;
use crate::game::moderation::moderation_action::ModerationAction;
use crate::game::player::player::Player;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_manager::PlayerManager;
use crate::game::room::room::Room;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::moderation::user_banned::USER_BANNED;
use crate::server::netty::streams::NettyRequest;
use crate::util::date_util::DateUtil;

pub struct ModeratorBanUserAction;

impl ModerationAction for ModeratorBanUserAction {
    /// Mirrors `performAction(Player, Room, String, String, NettyRequest)`.
    fn perform_action(
        &self,
        player: &Player,
        _room: &Room,
        alert_message: &str,
        _notes: &str,
        reader: &mut NettyRequest,
    ) {
        if !player.has_fuse(&Fuseright::Ban) {
            return;
        }

        let name = reader.read_string();
        let mut ban_hours = reader.read_int();
        let _ban_machine_id = reader.read_boolean();
        let ban_ip = reader.read_boolean();

        if ban_hours > 100000 {
            ban_hours = 100000;
        }

        if ban_hours < 2 {
            ban_hours = 2;
        }

        let Some(player_details) = PlayerManager::get_instance().get_player_data_by_name(&name) else {
            player.send(&ALERT::new(&format!("Could not find user: {}", name)));
            return;
        };

        if player_details.is_banned().is_some() {
            player.send(&ALERT::new("User is already banned!"));
            return;
        }

        let ban_time = DateUtil::get_current_time_seconds() as i64 + (ban_hours as i64) * 3600;
        BanDao::add_ban(
            BanType::UserId,
            &player_details.get_id().to_string(),
            ban_time,
            alert_message,
            player.get_details().get_id(),
        );

        if ban_ip {
            let latest_ip = PlayerDao::get_latest_ip(player_details.get_id());
            BanDao::add_ban(
                BanType::IpAddress,
                &latest_ip,
                ban_time,
                alert_message,
                player.get_details().get_id(),
            );
        }

        if let Some(target_arc) = PlayerManager::get_instance().get_player_by_id(player_details.get_id()) {
            {
                let target = target_arc.lock();
                target.send(&USER_BANNED::new(alert_message.to_string()));
            }
            GameScheduler::get_instance().schedule(
                move || {
                    target_arc.lock().kick_from_server();
                },
                1000,
            );
        }

        player.send(&ALERT::new("User is already banned!"));
    }
}

impl ModeratorBanUserAction {
    /// Mirrors the static `ban(PlayerDetails, String, String, String, long,
    /// boolean, boolean)` (the Java IP-address branch is commented out in
    /// the source; the `getMachineId() != null` check maps to a non-empty
    /// check since Rust `&str` is never null).
    pub fn ban(
        banning_player_details: &PlayerDetails,
        alert_message: &str,
        _notes: &str,
        name: &str,
        ban_seconds: i64,
        ban_machine_id: bool,
        _ban_ip: bool,
    ) -> String {
        let mut criteria: HashMap<BanType, String> = HashMap::new();
        let Some(player_details) = PlayerManager::get_instance().get_player_data_by_name(name) else {
            return format!("Could not find user: {name}");
        };

        if player_details.get_id() == banning_player_details.get_id() {
            return "Cannot ban yourself".to_string();
        }

        if player_details.is_banned().is_some() {
            return "User is already banned!".to_string();
        }

        if CommandManager::get_instance().has_permission(&player_details, "ban") {
            return "Cannot ban a user who has permission to ban".to_string();
        }

        let ban_time = DateUtil::get_current_time_seconds() as i64 + ban_seconds;
        BanDao::add_ban(
            BanType::UserId,
            &player_details.get_id().to_string(),
            ban_time,
            alert_message,
            banning_player_details.get_id(),
        );
        criteria.insert(BanType::UserId, player_details.get_id().to_string());

        if ban_machine_id && !player_details.get_machine_id().is_empty() {
            BanDao::add_ban(
                BanType::MachineId,
                player_details.get_machine_id(),
                ban_time,
                alert_message,
                banning_player_details.get_id(),
            );
            criteria.insert(BanType::MachineId, player_details.get_machine_id().to_string());
        }

        if let Some(target_arc) = PlayerManager::get_instance().get_player_by_id(player_details.get_id()) {
            let target = target_arc.lock();
            target.get_network().disconnect();
        }

        BanManager::get_instance().disconnect_ban_accounts(&criteria);
        format!("The user {} has been banned.", player_details.get_name())
    }
}
