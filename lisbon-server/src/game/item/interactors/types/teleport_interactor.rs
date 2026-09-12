//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.TeleportInteractor`.
use crate::dao::mysql::item_dao::ItemDao;
use crate::dao::mysql::teleporter_dao::TeleporterDao;
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::item::item_manager::ItemManager;
use crate::game::pathfinder::position::Position;
use crate::game::game_scheduler::GameScheduler;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::game::room::room::Room;
use crate::game::triggers::generic_trigger::GenericTrigger;
use crate::messages::outgoing::rooms::items::broadcast_teleporter::BROADCAST_TELEPORTER;
use crate::messages::outgoing::rooms::user::logout::LOGOUT;

pub struct TeleportInteractor {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl TeleportInteractor {
    /// Mirrors the `TELEPORTER_CLOSE` constant.
    pub const TELEPORTER_CLOSE: &'static str = "FALSE";

    /// Mirrors the `TELEPORTER_OPEN` constant.
    pub const TELEPORTER_OPEN: &'static str = "TRUE";

    /// Mirrors the `TELEPORT_FLASH_DELAY_MS` constant.
    pub const TELEPORT_FLASH_DELAY_MS: i64 = 500;

    /// Mirrors the `TELEPORT_EXIT_DELAY_MS` constant.
    pub const TELEPORT_EXIT_DELAY_MS: i64 = 900;

    /// Mirrors the `TeleportInteractor()` constructor.
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
    trigger: GenericTrigger,
        }
    }

    /// Mirrors `onInteract(Player, Room, Item, int)`.
    pub fn on_interact(&self, player: &Player, room: &Room, item: &mut Item, status: i32) {
        let Some(room_user) = player.get_room_user() else {
            return;
        };

        if status != 1 && room_user.get_authenticate_teleporter_id() != -1 {
            return;
        }

        let current_position = room_user.get_position();
        let front = Self::get_teleporter_front_square(item);

        if status == 1 {
            if item.get_position() == &current_position {
                return;
            }

            if &front != &current_position {
                return;
            }

            room_user.set_pending_teleporter_id(item.get_id());
            Self::open_teleporter(item);
            room_user.walk_to(item.get_position().get_x(), item.get_position().get_y());
            return;
        }

        if status == 2 {
            if item.get_position() != &current_position {
                if room_user.get_pending_teleporter_id() == item.get_id()
                    || item.get_position() == &room_user.get_goal()
                {
                    room_user.set_queued_teleporter_id(item.get_id());
                }

                return;
            }
        }

        room_user.set_pending_teleporter_id(-1);
        room_user.set_queued_teleporter_id(-1);
        room_user.set_walking_allowed(false);
        room_user.set_last_item_interaction(item);

        let pair_id = TeleporterDao::get_teleporter_id(item.get_id());

        // The `RoomEntity` clone shares the entity state, so the
        // scheduled block operates on the same instance as the Java
        // captured `roomUser`.
        let shared_room_user = (*room_user).clone();

        if pair_id == -1 || ItemDao::get_item(pair_id).is_none() {
            let teleport_item = item.clone();
            GameScheduler::get_instance().schedule(move || {
                shared_room_user.set_walking_allowed(true);
                let exit_square = Self::get_teleporter_front_square(&teleport_item);
                shared_room_user.walk_to(exit_square.get_x(), exit_square.get_y());
            }, 500);
            return;
        }

        let Some(target_teleporter) = ItemDao::get_item(pair_id) else {
            return;
        };

        if target_teleporter.get_room().is_none() {
            let teleport_item = item.clone();
            GameScheduler::get_instance().schedule(move || {
                shared_room_user.set_walking_allowed(true);
                let exit_square = Self::get_teleporter_front_square(&teleport_item);
                shared_room_user.walk_to(exit_square.get_x(), exit_square.get_y());
            }, 500);
            return;
        }

        let resolved_target = ItemManager::get_instance().resolve_item_by_id(pair_id);
        let mut paired_teleporter = match resolved_target {
            Some(target) => target,
            None => target_teleporter,
        };

        if paired_teleporter.get_room_id() == item.get_room_id() {
            self.handle_same_room_teleport(player, room, item, &mut paired_teleporter);
        } else {
            self.handle_cross_room_teleport(player, room, item, &mut paired_teleporter);
        }
    }

    /// Mirrors `handleSameRoomTeleport(Player, Room, Item, Item)`.
    fn handle_same_room_teleport(
        &self,
        player: &Player,
        room: &Room,
        source_teleporter: &mut Item,
        target_teleporter: &mut Item,
    ) {
        let Some(room_user_ref) = player.get_room_user() else {
            return;
        };

        // The Java guards `roomUser.getRoom() != room` /
        // `getAuthenticateTelporterId() != target.getId()` map to the room
        // id / the stored authenticate-teleporter id.
        let room_user = (*room_user_ref).clone();
        let room = room.clone();
        let room_id = room.get_id();
        let name = player.get_details().get_name().to_string();
        let mut source = (*source_teleporter).clone();
        let mut target = (*target_teleporter).clone();
        let target_id = target.get_id();

        GameScheduler::get_instance().schedule(
            {
                let room_user = room_user.clone();
                let mut target = target.clone();
                move || {
                    if room_user.get_room().map(|r| r.get_id()).is_none_or(|id| id != room_id) {
                        return;
                    }

                    Self::open_teleporter(&mut source);
                    room.send(&BROADCAST_TELEPORTER::new(
                        source.clone(),
                        &name,
                        true,
                    ));
                    room_user.warp(target.get_position(), true, true);
                    room_user.set_authenticate_teleporter_id(target_id);
                    Self::open_teleporter(&mut target);
                    room.send(&BROADCAST_TELEPORTER::new(
                        target,
                        &name,
                        false,
                    ));
                    Self::close_teleporter(&mut source);
                }
            },
            Self::TELEPORT_FLASH_DELAY_MS,
        );

        GameScheduler::get_instance().schedule(
            {
                let room_user = room_user.clone();
                let target = target.clone();
                move || {
                    if room_user.get_room().map(|r| r.get_id()).is_none_or(|id| id != room_id) {
                        return;
                    }

                    if room_user.get_authenticate_teleporter_id() != target_id {
                        return;
                    }

                    room_user.set_walking_allowed(true);
                    let exit_square = Self::get_teleporter_front_square(&target);
                    room_user.walk_to(exit_square.get_x(), exit_square.get_y());
                }
            },
            Self::TELEPORT_EXIT_DELAY_MS,
        );
    }

    /// Mirrors `handleCrossRoomTeleport(Player, Room, Item, Item)`.
    fn handle_cross_room_teleport(
        &self,
        player: &Player,
        room: &Room,
        source_teleporter: &mut Item,
        target_teleporter: &mut Item,
    ) {
        let Some(room_user_ref) = player.get_room_user() else {
            return;
        };

        // The Java `enterRoom(player, null)` resolves the player back to
        // the `Arc` handle via the player id.
        let room_user = (*room_user_ref).clone();
        let room = room.clone();
        let room_id = room.get_id();
        let name = player.get_details().get_name().to_string();
        let user_id = player.get_details().get_id();
        let source = (*source_teleporter).clone();
        let target = (*target_teleporter).clone();

        GameScheduler::get_instance().schedule(
            {
                let room_user = room_user.clone();
                let mut source = source.clone();
                let room = room.clone();
                move || {
                    if room_user.get_room().map(|r| r.get_id()).is_none_or(|id| id != room_id) {
                        return;
                    }

                    Self::open_teleporter(&mut source);
                    room.send(&BROADCAST_TELEPORTER::new(
                        source,
                        &name,
                        true,
                    ));
                }
            },
            Self::TELEPORT_FLASH_DELAY_MS,
        );

        GameScheduler::get_instance().schedule(
            {
                let room_user = room_user.clone();
                let target = target.clone();
                let mut source = source.clone();
                let room = room.clone();
                move || {
            let Some(player_arc) = PlayerManager::get_instance().get_player_by_id(user_id) else {
                return;
            };

            if room_user.get_room().map(|r| r.get_id()).is_none_or(|id| id != room_id) {
                return;
            }

            let player = player_arc.lock();
            room_user.set_authenticate_teleporter_id(target.get_id());
            Self::close_teleporter(&mut source);
            room.send(&LOGOUT::new(room_user.get_instance_id()));

            if let Some(target_room) = target.get_room() {
                let target_guard = target_room.lock();
                room_user.set_authenticate_id(target_guard.get_id());
                target_guard
                    .get_entity_manager()
                    .enter_room_entity(&target_guard, &*player, None);
            }
                }
            },
            Self::TELEPORT_EXIT_DELAY_MS,
        );
    }

    /// Mirrors `openTeleporter(Item)`.
    fn open_teleporter(item: &mut Item) {
        Self::set_teleporter_state(item, Self::TELEPORTER_OPEN);
    }

    /// Mirrors `closeTeleporter(Item)`.
    fn close_teleporter(item: &mut Item) {
        Self::set_teleporter_state(item, Self::TELEPORTER_CLOSE);
    }

    /// Mirrors `setTeleporterState(Item, String)`.
    fn set_teleporter_state(item: &mut Item, state: &str) {
        if state == item.get_custom_data() {
            return;
        }

        item.set_custom_data(state);
        item.update_status();
    }

    /// Mirrors `getTeleporterFrontSquare(Item)`.
    pub fn get_teleporter_front_square(item: &Item) -> Position {
        if item.has_behaviour(ItemBehaviour::RedirectRotation0) {
            return item.get_position().get_square_behind();
        }

        item.get_position().get_square_in_front()
    }
}

impl Default for TeleportInteractor {
    fn default() -> Self {
        Self::new()
    }
}
