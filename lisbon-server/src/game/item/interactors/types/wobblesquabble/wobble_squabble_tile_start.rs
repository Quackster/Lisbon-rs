//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.wobblesquabble.WobbleSquabbleTileStart`.
use rand::Rng;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::games::wobblesquabble::wobble_squabble_manager::WobbleSquabbleManager;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::room::enums::status_type::StatusType;
use crate::game::triggers::generic_trigger::GenericTrigger;
use crate::messages::outgoing::alert::alert::ALERT;

pub struct WobbleSquabbleTileStart {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl WobbleSquabbleTileStart {
    /// Mirrors the `WobbleSquabbleTileStart()` constructor.
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
    trigger: GenericTrigger,
        }
    }

    /// Mirrors `onEntityStop(Entity, RoomEntity, Item, boolean)`.
    pub fn on_entity_stop(
        &self,
        entity: &(dyn Entity + Send),
        room_entity: &RoomEntity,
        item: &Item,
        _is_rotation: bool,
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        if let Some(room) = room_entity.get_room() {
            if room
                .get_task_manager()
                .has_task(WobbleSquabbleManager::get_instance().get_name())
            {
                return;
            }
        }

        let other_tile_player: Vec<&str> = item.get_current_program().split(',').collect();

        let Some(x) = other_tile_player.first().and_then(|v| v.parse::<i32>().ok()) else {
            return;
        };
        let Some(y) = other_tile_player.get(1).and_then(|v| v.parse::<i32>().ok()) else {
            return;
        };

        let teleport_position = Position::new_xy(x, y);

        let Some(room) = room_entity.get_room() else {
            return;
        };

        let mapping = room.get_mapping();
        let mapping = mapping.lock();
        let Some(room_tile) = mapping.get_tile_by_position(&room, &teleport_position) else {
            return;
        };

        let Some(other_entity) = room_tile.get_entities().first() else {
            return;
        };

        let Some(first_player_ref) = entity.as_player() else {
            return;
        };
        let Some(first_player) = crate::game::player::player_manager::PlayerManager::get_instance()
            .get_player_by_id(first_player_ref.get_details().get_id())
        else {
            return;
        };

        let Some(second_player_ref) = other_entity.as_ref().as_player() else {
            return;
        };
        let Some(second_player) = crate::game::player::player_manager::PlayerManager::get_instance()
            .get_player_by_id(second_player_ref.get_details().get_id())
        else {
            return;
        };

        let players = [first_player_ref, second_player_ref];

        // Two players! :3
        let game = crate::game::games::wobblesquabble::wobble_squabble_game::WobbleSquabbleGame::new(
            first_player,
            second_player,
        );
        let game = parking_lot::Mutex::new(game);

        for player in players.iter() {
            if player.get_details().get_tickets() < WobbleSquabbleManager::WS_GAME_TICKET_COST {
                player.send(&ALERT::new(&format!(
                    "You need at least {} ticket(s) to play Wobble Squabble!",
                    WobbleSquabbleManager::WS_GAME_TICKET_COST
                )));

                let position = player
                    .get_room_user()
                    .map(|room_user| room_user.get_position())
                    .unwrap_or_default();

                let new_x = position.get_x() + if rand::thread_rng().gen_bool(0.5) { -1 } else { 1 };
                let rotation = position.get_rotation();

                let mut position = Position::new_xy(new_x, position.get_y());
                position.set_rotation(rotation);

                if let Some(room_user) = player.get_room_user() {
                    room_user.set_status(StatusType::Swim, "");
                    room_user.warp(&position, true, false);
                }

                return; // Too poor!
            }
        }

        // Disable walking requests
        for player in players.iter() {
            if let Some(room_user) = player.get_room_user() {
                room_user.set_walking_allowed(false);
            }
        }

        let prepare = crate::messages::outgoing::wobblesquabble::pt_prepare::PT_PREPARE::new(
            game.lock().get_player_arc(0).expect("wobble squabble player present").lock().clone(),
            game.lock().get_player_arc(1).expect("wobble squabble player present").lock().clone(),
        );
        game.lock().send(&prepare);

        let task = WobbleSquabbleTask {
            game: std::sync::Arc::new(game),
        };
        room.get_task_manager().schedule_task(
            WobbleSquabbleManager::get_instance().get_name(),
            std::sync::Arc::new(task),
            3000,
            200,
        );
    }
}

struct WobbleSquabbleTask {
    game: std::sync::Arc<parking_lot::Mutex<crate::game::games::wobblesquabble::wobble_squabble_game::WobbleSquabbleGame>>,
}

impl crate::game::room::managers::room_task_manager::Tickable for WobbleSquabbleTask {
    fn tick(&self) {
        self.game.lock().run();
    }
}

impl Default for WobbleSquabbleTileStart {
    fn default() -> Self {
        Self::new()
    }
}
