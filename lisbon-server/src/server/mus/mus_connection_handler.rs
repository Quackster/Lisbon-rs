//! Mirrors `net.h4bbo.lisbon.server.mus.MusConnectionHandler`.
//!
//! The Java `SimpleChannelInboundHandler<MusMessage>` becomes a per-connection
//! async task: `channelRegistered` / `channelUnregistered` bracket the read
//! loop, and `channelRead0` is `handle_message` (which returns whether the
//! connection should be closed, mirroring `ctx.channel().close()`).

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

use parking_lot::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::dao::mysql::item_dao::ItemDao;
use crate::dao::mysql::photo_dao::PhotoDao;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::entity::entity::Entity;
use crate::game::item::item::Item;
use crate::game::item::item_manager::ItemManager;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_manager::PlayerManager;
use crate::lisbon::Lisbon;
use crate::log::Log;
use crate::messages::outgoing::user::currencies::film::FILM;
use crate::server::mus::codec::mus_network_encoder::MusNetworkEncoder;
use crate::server::mus::codec::mus_network_decoder::MusNetworkDecoder;
use crate::server::mus::connection::mus_client::MusClient;
use crate::server::mus::streams::mus_message::MusMessage;
use crate::server::mus::streams::mus_prop_list::MusPropList;
use crate::server::mus::streams::mus_types as MusTypes;
use crate::server::netty::netty_server::ConnectionRecord;
use crate::server::netty::netty_player_network::NettyPlayerNetwork;
use crate::util::date_util::{DateUtil, CAMERA_DATE};
use crate::util::string_util::StringUtil;

/// Per-connection MUS handler (the tokio equivalent of the inbound handler).
pub struct MusConnectionHandler {
    channels: Arc<Mutex<Vec<ConnectionRecord>>>,
    connection_ids: Arc<AtomicI64>,
}

impl MusConnectionHandler {
    /// Create a handler for one accepted connection.
    pub fn new(
        channels: Arc<Mutex<Vec<ConnectionRecord>>>,
        connection_ids: Arc<AtomicI64>,
    ) -> Self {
        Self { channels, connection_ids }
    }

    /// Run the per-connection state machine.
    ///
    /// Mirrors `channelRegistered` → `channelRead0` (loop) → `channelUnregistered`.
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
                .error(format!("Could not accept MUS connection from {}", peer_ip));
            self.channels.lock().retain(|c| c.id != connection_id);
            self.connection_ids.fetch_sub(1, Ordering::Relaxed);
            return;
        }

        let (write_tx, mut write_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let mut client = MusClient::new(write_tx);

        let (mut read, mut write) = stream.into_split();

        // Outbound write task (mirrors `ctx.channel().writeAndFlush(...)`).
        let write_task = tokio::spawn(async move {
            while let Some(frame) = write_rx.recv().await {
                let _ = write.write_all(&frame).await;
                let _ = write.flush().await;
            }
        });

        // channelRead0 (loop)
        let mut buf: Vec<u8> = Vec::new();
        let mut chunk = vec![0u8; 65536];
        loop {
            match read.read(&mut chunk).await {
                Ok(0) => break,
                Ok(n) => buf.extend_from_slice(&chunk[..n]),
                Err(error) => {
                    Self::on_error(&error);
                    break;
                }
            }

            let mut should_close = false;
            loop {
                match MusNetworkDecoder::decode(&mut buf) {
                    Some((message, close)) => {
                        should_close = should_close || close;
                        should_close = should_close || Self::handle_message(&mut client, &message, &peer_ip);
                    }
                    None => break,
                }
            }
            if should_close {
                break;
            }
        }

        // channelUnregistered
        self.connection_ids.fetch_sub(1, Ordering::Relaxed);
        self.channels.lock().retain(|c| c.id != connection_id);

        write_task.abort();
    }

    /// Mirrors `channelRead0(ChannelHandlerContext, MusMessage)`.
    ///
    /// Returns `true` when the connection should be closed (mirrors
    /// `ctx.channel().close()` in the `LOGIN` anti-spoof path).
    fn handle_message(client: &mut MusClient, message: &MusMessage, peer_ip: &str) -> bool {
        let subject = message.get_subject().to_string();
        let mut should_close = false;

        if subject == "Logon" {
            let mut reply = MusMessage::new();
            reply.set_subject("Logon");
            reply.set_content_type(MusTypes::STRING);
            reply.set_content_string("Kepler: Habbo Hotel shockwave emulator");
            client.send(MusNetworkEncoder::encode(&mut reply));

            let mut reply = MusMessage::new();
            reply.set_subject("HELLO");
            reply.set_content_type(MusTypes::STRING);
            reply.set_content_string("");
            client.send(MusNetworkEncoder::encode(&mut reply));
        }

        if subject == "LOGIN" {
            let credentials: Vec<&str> = message.get_content_string().splitn(2, ' ').collect();
            let Some(credential_0) = credentials.first().copied() else {
                return should_close;
            };

            let player = if is_numeric(credential_0) {
                let user_id: i32 = credential_0.parse().unwrap_or(-1);
                PlayerManager::get_instance().get_player_by_id(user_id)
            } else if let Some(credential_1) = credentials.get(1).copied() {
                let mut details = PlayerDetails::new();
                if PlayerDao::login(&mut details, credential_0, credential_1) {
                    PlayerManager::get_instance().get_player_by_id(details.get_id())
                } else {
                    None
                }
            } else {
                // The Java `credentials[1]` throws on a missing password; the
                // connection is closed through the branch below.
                None
            };

            // Er, ma, gerd, we logged in! ;O
            match player {
                Some(player) => {
                    let player_guard = player.lock();
                    let player_ip = NettyPlayerNetwork::get_ip_address(player_guard.get_network());
                    if player_ip == peer_ip {
                        client.set_user_id(player_guard.get_details().get_id());
                    } else {
                        tracing::info!(
                            "[MUS] RCON user kicked due to inappropriate formed message {}",
                            peer_ip
                        );
                        should_close = true; // Lol, bye, imposter scum!
                    }
                }
                None => {
                    tracing::info!(
                        "[MUS] RCON user kicked due to inappropriate formed message {}",
                        peer_ip
                    );
                    should_close = true;
                }
            }
        }

        if subject == "PHOTOTXT" && client.get_user_id() > 0 {
            let content = message.get_content_string();
            let trimmed = if content.is_empty() {
                String::new()
            } else {
                content.chars().skip(1).collect()
            };
            client.set_photo_text(&StringUtil::filter_input(&trimmed, true));
        }

        if subject == "BINDATA" {
            let user_id = client.get_user_id();
            if user_id < 1 {
                return should_close;
            }
            let Some(player) = PlayerManager::get_instance().get_player_by_id(user_id) else {
                return should_close;
            };
            let player_guard = player.lock();
            let Some(room_user) = player_guard.get_room_user() else {
                return should_close;
            };
            if room_user.get_room().is_none() {
                return should_close;
            }

            let time_seconds = DateUtil::get_current_time_seconds();

            let cs = message
                .get_content_prop_list()
                .map(|pl| pl.get_prop_as_int("cs"))
                .unwrap_or(-1);
            let image = message
                .get_content_prop_list()
                .map(|pl| pl.get_prop_as_bytes("image"))
                .unwrap_or_default();
            let photo_text = client.get_photo_text();

            let mut photo = Item::new();
            photo.set_owner_id(user_id);
            let definition_id = ItemManager::get_instance()
                .get_definition_by_sprite("photo")
                .map(|definition| definition.get_id())
                .unwrap_or(0);
            photo.set_definition_id(definition_id);
            let mut custom_data = DateUtil::get_date(time_seconds as i64, CAMERA_DATE);
            if !photo_text.is_empty() {
                custom_data.push('\r');
                custom_data.push_str(&photo_text);
            }
            photo.set_custom_data(&custom_data);
            ItemDao::new_item(&mut photo);

            PhotoDao::add_photo(
                photo.get_id() as i64,
                user_id,
                time_seconds as i64,
                &image,
                cs,
            );

            let mut reply = MusMessage::new();
            reply.set_subject("BINDATA_SAVED");
            reply.set_content_type(MusTypes::STRING);
            reply.set_content_string(&user_id.to_string());
            client.send(MusNetworkEncoder::encode(&mut reply));

            if let Some(inventory) = player_guard.get_inventory() {
                inventory.add_item(&photo);
                inventory.view(&player_guard, "new");
            }

            CurrencyDao::decrease_film(player_guard.get_details(), 1);
            player_guard.send(&FILM::new(player_guard.get_details()));
        }

        if subject == "GETBINDATA" {
            let photo_id = message
                .get_content_string()
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0);

            let Some(photo) = PhotoDao::get_photo(photo_id) else {
                return should_close;
            };
            if client.get_user_id() < 1 {
                return should_close;
            }

            let mut reply = MusMessage::new();
            reply.set_subject("BINARYDATA");
            reply.set_content_type(MusTypes::PROPLIST);
            let mut prop_list = MusPropList::new(3);
            prop_list.set_prop_as_bytes("image", MusTypes::MEDIA, photo.get_data().to_vec());
            prop_list.set_prop_as_string("time", &DateUtil::get_date(photo.get_time(), CAMERA_DATE));
            prop_list.set_prop_as_int("cs", photo.get_checksum());
            reply.set_content_prop_list(Some(prop_list));
            client.send(MusNetworkEncoder::encode(&mut reply));
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
            .error_with("[MUS] Netty error occurred", error.to_string());
    }
}

/// Mirrors `org.apache.commons.lang3.StringUtils.isNumeric`.
fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit())
}
