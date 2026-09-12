//! Mirrors `net.h4bbo.lisbon.game.player.Player`.

use std::any::Any;
use std::collections::HashSet;
use std::sync::Arc;

use rand::Rng;

use crate::crypto::habbo_cipher::HabboCipher;
use crate::dao::mysql::item_dao::ItemDao;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use crate::dao::mysql::settings_dao::SettingsDao;
use crate::game::achievements::user::user_achievement_manager::UserAchievementManager;
use crate::game::badges::badge_manager::BadgeManager;
use crate::game::club::club_subscription::ClubSubscription;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::fuserights::fuserights_manager::FuserightsManager;
use crate::game::game_scheduler::GameScheduler;
use crate::dao::mysql::group_dao::GroupDao;
use crate::game::groups::group::Group;
use crate::game::guides::guide_manager::GuideManager;
use crate::game::inventory::inventory::Inventory;
use crate::game::messenger::messenger::Messenger;
use crate::game::player::guides::player_guide_manager::PlayerGuideManager;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_manager::PlayerManager;
use crate::game::player::player_rank::PlayerRank;
use crate::game::player::statistics::player_statistic::PlayerStatistic;
use crate::game::player::statistics::player_statistic_manager::PlayerStatisticManager;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::room::entities::room_player::RoomPlayer;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::alert::hotel_logout::{HOTEL_LOGOUT, LogoutReason};
use crate::messages::outgoing::club::club_gift::CLUB_GIFT;
use crate::messages::outgoing::handshake::available_sets::AVAILABLE_SETS;
use crate::messages::outgoing::handshake::login::LOGIN;
use crate::messages::outgoing::handshake::rights::RIGHTS;
use crate::messages::outgoing::moderation::user_banned::USER_BANNED;
use crate::messages::outgoing::openinghours::info_hotel_closing::INFO_HOTEL_CLOSING;
use crate::messages::outgoing::user::settings::help_items::HELP_ITEMS;
use crate::messages::types::MessageComposer;
use crate::server::netty::netty_player_network::NettyPlayerNetwork;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;

/// Mirrors the nested `Player.CryptoMode` enum.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CryptoMode {
    None,
    Init,
}

/// Mirrors `Player`.
///
/// `PLAYER_KEY` (a Netty `AttributeKey<Player>`) has no tokio equivalent: the
/// player is owned by the connection task and passed around as
/// `Arc<Mutex<Player>>` / `&mut Player`, which plays the same role.
pub struct Player {
    network: Arc<NettyPlayerNetwork>,
    details: PlayerDetails,
    // The Java field is `RoomPlayer roomEntity`; the `Entity` trait demands
    // `RoomEntity` and Rust has no subtyping, so the `RoomPlayer` is composed.
    room_player: RoomPlayer,
    ignored_list: HashSet<String>,
    log: tracing::Span,
    messenger: Option<Messenger>,
    inventory: Option<Inventory>,
    badge_manager: BadgeManager,
    achievement_manager: UserAchievementManager,
    statistic_manager: PlayerStatisticManager,
    joined_groups: Option<Vec<Group>>,
    logged_in: bool,
    disconnected: bool,
    ping_ok: bool,
    possible_achievements_sent: bool,
    inbound_encrypted: bool,
    outbound_encrypted: bool,
    inbound_cipher: Option<HabboCipher>,
    outbound_cipher: Option<HabboCipher>,
    crypto_mode: CryptoMode,
    time_connected: i32,
    last_gift: Option<String>,
    guide_manager: PlayerGuideManager,
}

impl Player {
    /// Mirrors the `Player(NettyPlayerNetwork)` constructor.
    pub fn new(network: NettyPlayerNetwork) -> Self {
        let connection_id = network.get_connection_id();
        let log = tracing::info_span!("Connection {}", connection_id);
        Self {
            network: Arc::new(network),
            details: PlayerDetails::new(),
            room_player: RoomPlayer::new(),
            ignored_list: HashSet::new(),
            log,
            messenger: None,
            inventory: None,
            badge_manager: BadgeManager::default(),
            achievement_manager: UserAchievementManager::new(),
            statistic_manager: PlayerStatisticManager::new(-1, Default::default()),
            joined_groups: None,
            logged_in: false,
            disconnected: false,
            ping_ok: true,
            possible_achievements_sent: false,
            inbound_encrypted: false,
            outbound_encrypted: false,
            inbound_cipher: None,
            outbound_cipher: None,
            crypto_mode: CryptoMode::None,
            time_connected: 0,
            last_gift: None,
            guide_manager: PlayerGuideManager::new(),
        }
    }

    /// Mirrors `UUID.randomUUID().toString()` (the `java.util.UUID` is a
    /// `rand`-generated string).
    fn random_uuid() -> String {
        let mut rng = rand::thread_rng();
        let bytes: [u8; 16] = std::array::from_fn(|_| rng.gen());

        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
            bytes[8],
            bytes[9],
            bytes[10],
            bytes[11],
            bytes[12],
            bytes[13],
            bytes[14],
            bytes[15],
        )
    }

    /// Login handler for player.
    pub fn login(&mut self) {
        let name = self.details.get_name().to_string(); // Update logger to show name
        self.log = tracing::info_span!("Player {}", name);
        self.logged_in = true;
        self.ping_ok = true;

        self.time_connected = DateUtil::get_current_time_seconds();

        let player_manager = PlayerManager::get_instance();
        player_manager.disconnect_session(self.details.get_id()); // Kill other sessions with same id
        // Port note: `add_player` takes an `Arc<Mutex<Player>>`, which is
        // not available from `&mut Player` (the callers hold the `Arc`).

        if self.details.get_name() != "Abigail.Ryan" {
            PlayerDao::save_last_online(self.details.get_id(), self.details.get_last_online(), true);
        }

        if GameConfiguration::get_instance().get_bool("reset.sso.after.login") {
            PlayerDao::reset_sso_ticket(self.details.get_id()); // Protect against replay attacks
        }

        SettingsDao::update_setting(
            "players.online",
            &player_manager.get_players().len().to_string(),
        );

        self.messenger = Some(Messenger::from_details(&self.details));
        // The Java `new Inventory(this)` constructor runs the `reload`
        // with the (still roomless) `RoomPlayer`; the `&Player` reload
        // step lands here.
        let mut inventory = Inventory::default();
        inventory.reload(self.details.get_id(), self.get_room_user());
        self.inventory = Some(inventory);

        // Bye bye!
        if let Some((ban_reason, _)) = self.details.is_banned() {
            self.send(&USER_BANNED::new(ban_reason));
            // The Java `this::kickFromServer` bound method is resolved
            // back to the `Arc` handle via the player id.
            let user_id = self.details.get_id();
            GameScheduler::get_instance().schedule(move || {
                PlayerManager::get_instance()
                    .get_player_by_id(user_id)
                    .map(|player| player.lock().kick_from_server());
            }, 1000);
            return;
        }

        // Update user IP address
        let ip_address = NettyPlayerNetwork::get_ip_address(self.get_network());
        let latest_ip = PlayerDao::get_latest_ip(self.details.get_id());

        if latest_ip.is_empty() || latest_ip != ip_address {
            PlayerDao::log_ip_address(self.details.get_id(), &ip_address);
        }

        // Set trade ban back to 0, easier for db querying
        if self.details.get_trade_ban_expiration() > 0 && !self.details.is_trade_banned() {
            self.details.set_trade_ban_expiration(0);
            ItemDao::save_trade_ban_expire(self.details.get_id(), 0);
        }

        let mut stats = PlayerStatisticsDao::get_statistics(self.details.get_id());

        if stats.is_empty() {
            PlayerStatisticsDao::new_statistics(self.details.get_id(), &Self::random_uuid());
            stats = PlayerStatisticsDao::get_statistics(self.details.get_id());
        }

        self.statistic_manager = PlayerStatisticManager::new(self.details.get_id(), stats);
        self.achievement_manager.load_achievements(self.details.get_id());
        self.badge_manager.load_badges(self.details.get_id());
        self.details.reset_next_handout();
        self.refresh_joined_groups();

        self.send(&RIGHTS::new(self.get_fuserights()));
        self.send(&LOGIN);

        if GameConfiguration::get_instance().get_bool("welcome.message.enabled") {
            let alert_message = GameConfiguration::get_instance()
                .get_string("welcome.message.content")
                .replace("%username%", self.details.get_name());
            self.send(&ALERT::new(&alert_message));
        }

        if PlayerManager::get_instance().is_maintenance() {
            let maintenance_at = PlayerManager::get_instance()
                .get_maintenance_at()
                .unwrap_or_default();
            self.send(&INFO_HOTEL_CLOSING::new(maintenance_at));
        }

        if ClubSubscription::is_gift_due(self) {
            let gifts_due = self.statistic_manager.get_int_value(PlayerStatistic::GiftsDue);
            self.send(&CLUB_GIFT::new(std::cmp::max(1, gifts_due)));
        }

        if let Some(messenger) = &self.messenger {
            messenger.send_status_update();
        }

        // Guide checks
        self.guide_manager.set_player_id(self.details.get_id());
        let is_guide = GuideManager::get_instance().is_guide(self);
        self.guide_manager.set_guide(is_guide);

        if GameConfiguration::get_instance().get_bool("tutorial.enabled") {
            if self.guide_manager.is_guide() {
                self.guide_manager.set_has_tutorial(false);
                self.guide_manager.refresh_guiding_users();
            } else {
                self.guide_manager.set_has_tutorial(
                    self.statistic_manager.get_int_value(PlayerStatistic::HasTutorial) == 1,
                );
            }
        }

        if GameConfiguration::get_instance().get_bool("tutorial.enabled") {
            if self.guide_manager.has_tutorial() {
                self.send(&HELP_ITEMS::new(vec![1, 2, 3, 4, 5, 6, 7, 8]));
            }
        }
    }

    /// Refresh club for player.
    pub fn refresh_club(&mut self) {
        if self.details.has_club_subscription() {
            self.send(&AVAILABLE_SETS::new(format!(
                "[{}]",
                GameConfiguration::get_instance().get_string("users.figure.parts.club")
            )));
        }

        if !self.details.has_club_subscription() {
            // If the database still thinks we have Habbo club even after it expired, reset it back to 0.
            if self.details.get_club_expiration() > 0 {
                self.details.set_club_expiration(0);
                self.send(&RIGHTS::new(self.get_fuserights()));
                PlayerDao::save_subscription(
                    self.details.get_id(),
                    self.details.get_first_club_subscription(),
                    self.details.get_club_expiration(),
                );
            }
        } else {
            ClubSubscription::check_badges(self);

            if ClubSubscription::is_gift_due(self) {
                let gifts_due = self.statistic_manager.get_int_value(PlayerStatistic::GiftsDue);
                self.send(&CLUB_GIFT::new(gifts_due));
            }
        }

        ClubSubscription::send_hc_days(self);
    }

    /// Send fuseright permissions for player.
    pub fn get_fuserights(&self) -> Vec<Fuseright> {
        let mut fuserights = FuserightsManager::get_instance().get_fuserights_for_rank(
            self.details.get_rank().unwrap_or(PlayerRank::Normal),
        );

        if self.details.has_club_subscription() {
            fuserights.extend(FuserightsManager::get_instance().get_club_fuserights());
        }

        fuserights.retain(|fuseright| fuseright.name().starts_with("fuse_"));

        fuserights
    }

    /// Send a response to the player.
    pub fn send(&self, response: &dyn MessageComposer) {
        self.network.send(response)
    }

    /// Send a object to the player.
    pub fn send_object(&self, object: &dyn Any) {
        self.network.send_object(object)
    }

    /// Send a queued response to the player.
    pub fn send_queued(&self, response: &dyn MessageComposer) {
        self.network.send_queued(response)
    }

    /// Flush queue.
    pub fn flush(&self) {
        self.network.flush()
    }

    /// Get the messenger instance for the player.
    pub fn get_messenger(&self) -> Option<&Messenger> {
        self.messenger.as_ref()
    }

    /// Mirrors `getDetails()` (mutable form, used by `ChangeMottoCommand`).
    pub fn get_details_mut(&mut self) -> &mut PlayerDetails {
        &mut self.details
    }

    /// Get the inventory handler for player.
    pub fn get_inventory(&self) -> Option<&Inventory> {
        self.inventory.as_ref()
    }

    /// Get the badge manager for player.
    pub fn get_badge_manager(&self) -> &BadgeManager {
        &self.badge_manager
    }

    /// Get the player logger.
    pub fn get_logger(&self) -> &tracing::Span {
        &self.log
    }

    /// Get the network handler for the player.
    pub fn get_network(&self) -> &NettyPlayerNetwork {
        &self.network
    }

    /// Get the user achievement manager.
    pub fn get_achievement_manager(&self) -> &UserAchievementManager {
        &self.achievement_manager
    }

    /// Get the guide manager for the user.
    pub fn get_guide_manager(&self) -> &PlayerGuideManager {
        &self.guide_manager
    }

    /// Get the statistic manager for the user.
    pub fn get_statistic_manager(&self) -> &PlayerStatisticManager {
        &self.statistic_manager
    }

    /// Get if the player has logged in or not.
    pub fn is_logged_in(&self) -> bool {
        self.logged_in
    }

    /// Get if the connection has timed out or not.
    pub fn is_ping_ok(&self) -> bool {
        self.ping_ok
    }

    /// Get if the socket has been disconnected.
    pub fn is_disconnected(&self) -> bool {
        self.disconnected
    }

    /// Get if the client has received its possible achievement list.
    pub fn has_possible_achievements_sent(&self) -> bool {
        self.possible_achievements_sent
    }

    /// Mark the possible achievement list as sent to the client.
    pub fn set_possible_achievements_sent(&mut self) {
        self.possible_achievements_sent = true;
    }

    /// Mirrors `getInboundCipher`.
    pub fn get_inbound_cipher(&self) -> Option<HabboCipher> {
        self.inbound_cipher
    }

    /// Mirrors `setInboundCipher`.
    pub fn set_inbound_cipher(&mut self, inbound_cipher: Option<HabboCipher>) {
        self.inbound_cipher = inbound_cipher
    }

    /// Mirrors `getOutboundCipher`.
    pub fn get_outbound_cipher(&self) -> Option<HabboCipher> {
        self.outbound_cipher
    }

    /// Mirrors `setOutboundCipher`.
    pub fn set_outbound_cipher(&mut self, outbound_cipher: Option<HabboCipher>) {
        self.outbound_cipher = outbound_cipher
    }

    /// Mirrors `isInboundEncrypted`.
    pub fn is_inbound_encrypted(&self) -> bool {
        self.inbound_encrypted
    }

    /// Mirrors `setInboundEncrypted`.
    pub fn set_inbound_encrypted(&mut self, inbound_encrypted: bool) {
        self.inbound_encrypted = inbound_encrypted
    }

    /// Mirrors `isOutboundEncrypted`.
    pub fn is_outbound_encrypted(&self) -> bool {
        self.outbound_encrypted
    }

    /// Mirrors `setOutboundEncrypted`.
    pub fn set_outbound_encrypted(&mut self, outbound_encrypted: bool) {
        self.outbound_encrypted = outbound_encrypted
    }

    /// Mirrors `getCryptoMode`.
    pub fn get_crypto_mode(&self) -> CryptoMode {
        self.crypto_mode
    }

    /// Mirrors `setCryptoMode` (the Java null-check is inapplicable here).
    pub fn set_crypto_mode(&mut self, crypto_mode: CryptoMode) {
        self.crypto_mode = crypto_mode
    }

    /// Mirrors `resetCrypto`.
    pub fn reset_crypto(&mut self) {
        self.inbound_cipher = None;
        self.outbound_cipher = None;
        self.inbound_encrypted = false;
        self.outbound_encrypted = false;
        self.crypto_mode = CryptoMode::None
    }

    /// Set if the connection has timed out or not.
    pub fn set_ping_ok(&mut self, ping_ok: bool) {
        self.ping_ok = ping_ok
    }

    /// Get rid of the player from the server.
    pub fn kick_from_server(&mut self) {
        self.network.send(&HOTEL_LOGOUT::new(LogoutReason::Disconnect));
        self.network.disconnect();
        self.dispose();
    }

    /// Refresh the groups the user has joined.
    pub fn refresh_joined_groups(&mut self) {
        self.joined_groups = Some(GroupDao::get_joined_groups(self.details.get_id()));
    }

    /// Get the list of groups the user has joined.
    pub fn get_joined_groups(&self) -> Option<&[Group]> {
        self.joined_groups.as_deref()
    }

    /// Get the joined group.
    pub fn get_joined_group(&self, joined_group_id: i32) -> Option<&Group> {
        self.joined_groups
            .as_deref()
            .and_then(|groups| groups.iter().find(|group| group.get_id() == joined_group_id))
    }

    /// Get the ignored list.
    pub fn get_ignored_list(&self) -> &HashSet<String> {
        &self.ignored_list
    }

    /// Mirrors `getIgnoredList().add(String)`.
    pub fn add_ignored_name(&mut self, name: &str) {
        self.ignored_list.insert(name.to_string());
    }

    /// Mirrors `getIgnoredList().remove(String)`.
    pub fn remove_ignored_name(&mut self, name: &str) -> bool {
        self.ignored_list.remove(name)
    }

    /// Mirrors `setLastGift`.
    pub fn set_last_gift(&mut self, last_gift: Option<&str>) {
        self.last_gift = last_gift.map(String::from);
    }

    /// Mirrors `getLastGift`.
    pub fn get_last_gift(&self) -> Option<&str> {
        self.last_gift.as_deref()
    }
}

impl Entity for Player {
    fn has_fuse(&self, permission: &Fuseright) -> bool {
        FuserightsManager::get_instance().has_fuseright(*permission, &self.details)
    }

    fn get_details(&self) -> &PlayerDetails {
        &self.details
    }

    fn get_room_user(&self) -> Option<&RoomEntity> {
        // The Java override returns the `RoomPlayer`; the `Entity` trait
        // demands `RoomEntity`, so the composed base entity is handed out.
        Some(&self.room_player.entity)
    }

    fn get_type(&self) -> EntityType {
        EntityType::Player
    }

    fn dispose(&mut self) {
        if self.logged_in {
            if let Some(room) = self.room_player.entity.get_room() {
                room.get_entity_manager().leave_room(&room, self, false);
            }
            // Port note: `stopObservingGame` (via `Game::remove_observer`)
            // and `PlayerManager.remove_player` take an
            // `Arc<Mutex<Player>>`, which is not available from
            // `&mut Player`.
            if let Some(game_player) = self.room_player.entity.get_game_player() {
                if let Some(game) = game_player.lock().get_game() {
                    game.leave_game(&game_player);
                }
            }
            ClubSubscription::count_member_days(self);

            let logged_in_time = DateUtil::get_current_time_seconds() - self.time_connected;
            self.statistic_manager
                .increment_value(PlayerStatistic::OnlineTime, logged_in_time);
            self.details
                .set_last_online(DateUtil::get_current_time_seconds() as i64);

            if self.details.get_name() != "Abigail.Ryan" {
                PlayerDao::save_last_online(self.details.get_id(), self.details.get_last_online(), false);
            }

            SettingsDao::update_setting(
                "players.online",
                &PlayerManager::get_instance().get_players().len().to_string(),
            );

            if let Some(messenger) = &self.messenger {
                messenger.send_status_update();
            }

            self.disconnected = true;
            self.logged_in = false;
        }
    }

    fn as_player(&self) -> Option<&Player> {
        Some(self)
    }

    fn as_player_mut(&mut self) -> Option<&mut Player> {
        Some(self)
    }
}
