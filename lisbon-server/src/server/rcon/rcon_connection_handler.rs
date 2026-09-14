//! Mirrors `net.h4bbo.lisbon.server.rcon.RconConnectionHandler`.
//!
//! The Java `ChannelInboundHandlerAdapter` becomes a per-connection async task:
//! `channelRegistered` / `channelUnregistered` bracket the read loop, and
//! `channelRead` is `handle_message`. RCON is inbound-only (the Java handler
//! never writes back), so the outbound half of the split stream is dropped.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

use parking_lot::Mutex;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::dao::mysql::messenger_dao::MessengerDao;
use crate::dao::mysql::photo_dao::PhotoDao;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::dao::mysql::transaction_dao::TransactionDao;
use crate::game::achievements::achievement_manager::AchievementManager;
use crate::game::achievements::achievement_type::AchievementType;
use crate::game::entity::entity::Entity;
use crate::game::infobus::infobus_manager::InfobusManager;
use crate::game::item::item_manager::ItemManager;
use crate::game::messenger::messenger_user::MessengerUser;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::game::room::room_manager::RoomManager;
use crate::lisbon::Lisbon;
use crate::log::Log;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::handshake::rights::RIGHTS;
use crate::messages::outgoing::rooms::groups::group_badges::GROUP_BADGES;
use crate::messages::outgoing::rooms::groups::group_membership_update::GROUP_MEMBERSHIP_UPDATE;
use crate::messages::outgoing::user::currencies::credit_balance::CREDIT_BALANCE;
use crate::server::netty::netty_server::ConnectionRecord;
use crate::server::rcon::codec::rcon_network_decoder::RconNetworkDecoder;
use crate::server::rcon::messages::rcon_header::RconHeader;
use crate::server::rcon::messages::rcon_message::RconMessage;

/// Per-connection RCON handler (the tokio equivalent of the inbound handler).
pub struct RconConnectionHandler {
    channels: Arc<Mutex<Vec<ConnectionRecord>>>,
    connection_ids: Arc<AtomicI64>,
}

impl RconConnectionHandler {
    /// Create a handler for one accepted connection.
    pub fn new(
        channels: Arc<Mutex<Vec<ConnectionRecord>>>,
        connection_ids: Arc<AtomicI64>,
    ) -> Self {
        Self { channels, connection_ids }
    }

    /// Run the per-connection state machine.
    ///
    /// Mirrors `channelRegistered` → `channelRead` (loop) → `channelUnregistered`.
    pub async fn handle(self, stream: TcpStream) {
        let peer_ip = stream
            .peer_addr()
            .map(|a| a.ip().to_string())
            .unwrap_or_default();

        // channelRegistered
        let connection_id = self.connection_ids.fetch_add(1, Ordering::Relaxed);
        self.channels
            .lock()
            .push(ConnectionRecord { id: connection_id, ip: peer_ip.clone() });
        if Lisbon::is_shutting_down() {
            Log::get_error_logger()
                .error(format!("Could not accept RCON connection from {}", peer_ip));
            self.channels.lock().retain(|c| c.id != connection_id);
            self.connection_ids.fetch_sub(1, Ordering::Relaxed);
            return;
        }

        // channelRead (loop)
        let mut stream = stream;
        let mut buf: Vec<u8> = Vec::new();
        let mut chunk = vec![0u8; 65536];
        loop {
            match stream.read(&mut chunk).await {
                Ok(0) => break,
                Ok(n) => buf.extend_from_slice(&chunk[..n]),
                Err(error) => {
                    Self::on_error(&error);
                    break;
                }
            }

            loop {
                if let Some(message) = RconNetworkDecoder::decode(&mut buf) {
                    // Run the (synchronous) handler on a blocking thread: it
                    // reaches the DAOs, which `block_on` the dedicated storage
                    // runtime, and that panics on a runtime worker thread.
                    let _ = tokio::task::spawn_blocking(move || {
                        Self::handle_message(&message)
                    })
                    .await;
                } else {
                    break;
                }
            }
        }

        // channelUnregistered
        self.connection_ids.fetch_sub(1, Ordering::Relaxed);
        self.channels.lock().retain(|c| c.id != connection_id);
    }

    /// Look up a player by id from a message value (mirrors
    /// `PlayerManager.getInstance().getPlayerById(Integer.parseInt(...))`).
    fn get_player(values: &HashMap<String, String>, key: &str) -> Option<Arc<Mutex<Player>>> {
        values
            .get(key)
            .and_then(|v| v.parse::<i32>().ok())
            .and_then(|id| PlayerManager::get_instance().get_player_by_id(id))
    }

    /// Mirrors `channelRead(ChannelHandlerContext, Object)`.
    fn handle_message(message: &RconMessage) -> bool {
        let header = message.get_header();
        let values = message.get_values();
        // RCON is inbound-only: `channelRead` never closes the channel.
        let should_close = false;

        match header {
            Some(RconHeader::DisconnectUser) => {
                if let Some(player) = Self::get_player(values, "userId") {
                    let guard = player.lock();
                    guard.get_network().disconnect();
                }
            }
            Some(RconHeader::RefreshLooks) => {
                if let Some(player) = Self::get_player(values, "userId") {
                    let guard = player.lock();
                    if let Some(room_user) = guard.get_room_user() {
                        room_user.refresh_appearance();
                    }
                }
            }
            Some(RconHeader::HotelAlert) => {
                let sender = values.get("sender").map(|s| s.as_str()).unwrap_or("");
                let hotel_alert = values.get("message").map(|s| s.as_str()).unwrap_or("");

                let mut alert = String::new();
                alert.push_str(hotel_alert);
                alert.push_str("<br>");
                alert.push_str("<br>");
                alert.push_str("- ");
                alert.push_str(sender);

                for player in PlayerManager::get_instance().get_players().iter() {
                    player.lock().send(&ALERT::new(&alert));
                }
            }
            Some(RconHeader::RefreshClub) => {
                if let Some(player) = Self::get_player(values, "userId") {
                    let mut guard = player.lock();
                    if let Some(new_details) =
                        PlayerDao::get_details(guard.get_details().get_id())
                    {
                        guard.get_details_mut().set_credits(new_details.get_credits());
                        guard.get_details_mut().set_club_expiration(
                            new_details.get_club_expiration(),
                        );
                        guard.get_details_mut().set_first_club_subscription(
                            new_details.get_first_club_subscription(),
                        );

                        PlayerDao::save_currency(
                            guard.get_details().get_id(),
                            new_details.get_credits(),
                        );

                        guard.send(&CREDIT_BALANCE::new(guard.get_details().get_credits()));
                        guard.send(&RIGHTS::new(guard.get_fuserights()));
                        guard.refresh_club();

                        AchievementManager::get_instance()
                            .try_progress(&AchievementType::Hc, &guard);
                    }
                }
            }
            Some(RconHeader::RefreshHand) => {
                if let Some(player) = Self::get_player(values, "userId") {
                    let guard = player.lock();
                    let room_user = guard.get_room_user();
                    if let Some(inventory) = guard.get_inventory() {
                        inventory.reload(guard.get_details().get_id(), room_user);
                        if room_user.is_some() {
                            inventory.view(&guard, "new");
                        }
                    }
                }
            }
            Some(RconHeader::RefreshCredits) => {
                if let Some(player) = Self::get_player(values, "userId") {
                    let mut guard = player.lock();
                    let credits = CurrencyDao::get_credits(guard.get_details().get_id());
                    guard.get_details_mut().set_credits(credits);
                    guard.send(&CREDIT_BALANCE::new(guard.get_details().get_credits()));
                }
            }
            Some(RconHeader::FriendRequest) => {
                let online = Self::get_player(values, "friendId");
                let request_from = values
                    .get("userId")
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or(0);

                if let Some(player) = online {
                    let guard = player.lock();
                    if let Some(messenger) = guard.get_messenger() {
                        if !messenger.has_request(request_from) {
                            if let Some(data) = PlayerManager::get_instance()
                                .get_player_data_by_id(request_from)
                            {
                                messenger.add_request(&MessengerUser::from_details(&data));
                            }
                        }
                    }
                }
            }
            Some(RconHeader::RefreshMessengerCategories) => {
                if let Some(player) = Self::get_player(values, "userId") {
                    let guard = player.lock();
                    if let Some(messenger) = guard.get_messenger() {
                        messenger.reload_categories(guard.get_details().get_id());

                        // Refresh friends for categories
                        for db_friend in MessengerDao::get_friends(guard.get_details().get_id())
                            .values()
                        {
                            if let Some(friend) = messenger.get_friend(db_friend.get_user_id())
                            {
                                if friend.get_category_id() != db_friend.get_category_id() {
                                    let mut friend = friend;
                                    friend.set_category_id(db_friend.get_category_id());
                                    messenger.queue_friend_update(&friend);
                                }
                            }
                        }
                    }
                }
            }
            Some(RconHeader::RefreshGroupPerms) => {
                if let Some(player) = Self::get_player(values, "userId") {
                    let mut guard = player.lock();
                    guard.refresh_joined_groups();

                    if let Some(new_details) =
                        PlayerDao::get_details(guard.get_details().get_id())
                    {
                        guard
                            .get_details_mut()
                            .set_favourite_group_id(new_details.get_favourite_group_id());

                        let new_group = guard.get_details().get_favourite_group_id();

                        if let Some(room) = guard.get_room_user()
                            .and_then(|ru| ru.get_room())
                        {
                            let group_member = if guard.get_details().get_favourite_group_id() > 0 {
                                guard.get_details_mut().get_group_member()
                            } else {
                                None
                            };
                            if let Some(group_member) = group_member {
                                let member_group_id = group_member.get_group_id();
                                let client_rank = group_member
                                    .get_member_rank()
                                    .map(|r| r.get_client_rank())
                                    .unwrap_or(-1);
                                drop(group_member);
                                if let Some(joined) = guard.get_joined_group(new_group) {
                                    let mut group_badges = HashMap::new();
                                    group_badges.insert(new_group, joined.get_badge());
                                    room.send(&GROUP_BADGES::new(group_badges));
                                }
                                let instance_id = guard.get_room_user()
                                    .map(|ru| ru.get_instance_id())
                                    .unwrap_or(0);
                                room.send(&GROUP_MEMBERSHIP_UPDATE::new(
                                    instance_id,
                                    member_group_id,
                                    client_rank,
                                ));
                            } else {
                                let instance_id = guard.get_room_user()
                                    .map(|ru| ru.get_instance_id())
                                    .unwrap_or(0);
                                room.send(&GROUP_MEMBERSHIP_UPDATE::new(instance_id, -1, -1));
                            }
                        }
                    }
                }
            }
            Some(RconHeader::GroupDeleted) => {
                let group_id = values
                    .get("groupId")
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or(0);

                for player in PlayerManager::get_instance().get_players().iter() {
                    let mut guard = player.lock();
                    if guard.get_details().get_favourite_group_id() == group_id {
                        guard.get_details_mut().set_favourite_group_id(0);

                        if let Some(room) = guard.get_room_user()
                            .and_then(|ru| ru.get_room())
                        {
                            let instance_id = guard.get_room_user()
                                .map(|ru| ru.get_instance_id())
                                .unwrap_or(0);
                            room.send(&GROUP_MEMBERSHIP_UPDATE::new(instance_id, -1, -1));
                        }

                        guard.refresh_joined_groups();
                    }
                }
            }
            Some(RconHeader::RefreshGroup) => {
                let group_id = values
                    .get("groupId")
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or(0);

                for player in PlayerManager::get_instance().get_players().iter() {
                    let mut guard = player.lock();
                    if guard.get_joined_group(group_id).is_some() {
                        guard.refresh_joined_groups();
                    }
                }
            }
            Some(RconHeader::InfobusDoorStatus) => {
                InfobusManager::get_instance().update_door_status(
                    values.get("doorStatus").map(|v| v == "1").unwrap_or(false),
                );
            }
            Some(RconHeader::InfobusEndEvent) => {
                InfobusManager::get_instance().stop_event();
            }
            Some(RconHeader::InfobusPoll) => {
                let poll_id = values
                    .get("pollId")
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or(0);
                InfobusManager::get_instance().start_polling(poll_id);
            }
            Some(RconHeader::ClearPhoto) => {
                let item_id = values
                    .get("itemId")
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or(0);
                let user_id = values
                    .get("userId")
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or(0);

                if let Some(mut item) =
                    ItemManager::get_instance().resolve_item_by_id(item_id)
                {
                    let room = item.get_room().map(|r| r.lock().clone());
                    if let Some(room) = &room {
                        room.get_mapping().lock().remove_item(&room, &mut item);
                    }

                    item.delete();
                    PhotoDao::delete_item(item_id as i64);
                    TransactionDao::create_transaction(
                        user_id,
                        &item_id.to_string(),
                        "0",
                        1,
                        &format!("Hidden photo {}", item_id),
                        0,
                        0,
                        false,
                    );
                }
            }
            Some(RconHeader::RefreshStatistics) => {
                if let Some(player) = Self::get_player(values, "userId") {
                    let guard = player.lock();
                    guard.get_statistic_manager().reload();
                }
            }
            Some(RconHeader::RefreshRoomBadges) => {
                RoomManager::get_instance().reload_badges();
                RoomManager::get_instance().give_badges();
            }
            _ => {}
        }

        should_close
    }

    /// Mirrors `exceptionCaught(ChannelHandlerContext, Throwable)`.
    ///
    /// The Java handler logs non-`IOException` errors and closes the channel;
    /// tokio surfaces read errors here, and the read loop closes the
    /// connection.
    fn on_error(error: &std::io::Error) {
        Log::get_error_logger()
            .error_with("[RCON] Error occurred", error.to_string());
    }
}
