//! Mirrors `net.h4bbo.lisbon.game.games.battleball.powerups.NailBoxHandle`.
use std::sync::Arc;

use parking_lot::Mutex;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::game_scheduler::GameScheduler;
use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::enums::battle_ball_player_state::BattleBallPlayerState;
use crate::game::games::battleball::events::despawn_object_event::DespawnObjectEvent;
use crate::game::games::battleball::events::pin_spawn_event::PinSpawnEvent;
use crate::game::games::battleball::objects::pin_object::PinObject;
use crate::game::games::game_object::GameObject;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::utils::power_up_util::PowerUpUtil;
use crate::game::pathfinder::position::Position;
use crate::game::room::room::Room;

pub struct NailBoxHandle;

impl NailBoxHandle {
    /// Mirrors `handle(BattleBallGame, GamePlayer, Room)`.
    pub fn handle(
        game: Arc<BattleBallGame>,
        game_player: Arc<Mutex<GamePlayer>>,
        _room: &Room,
    ) {
        //gamePlayer.getPlayer().getRoomUser().stopWalking();

        let mut dizzy_players: Vec<Arc<Mutex<GamePlayer>>> = Vec::new();
        let mut pins: Vec<PinObject> = Vec::new();

        let (player_position, next_position) = {
            let player = game_player.lock();
            let player_guard = player.get_player().lock();
            let room_user = player_guard.get_room_user();

            (
                room_user
                    .map(|room_user| room_user.get_position())
                    .unwrap_or_default(),
                room_user.and_then(|room_user| room_user.get_next_position()),
            )
        };

        let mut tile_position = player_position.copy();

        for _ in 0..6 {
            tile_position = tile_position.get_square_in_front();
        }

        /*
        var tile2 = (BattleBallTile) game.getTile(tilePosition.getX(), tilePosition.getY());
        ...
        game.getUpdateTilesQueue().add(tile2);
        */

        let max_pins = rand::thread_rng().gen_range(8..16);
        let mut selected_positions: Vec<Position> = Vec::new();
        let mut circle_positions = tile_position.get_circle(3);

        circle_positions.shuffle(&mut rand::thread_rng());

        let __mapping_arc = game.get_room().get_mapping();
let mapping = __mapping_arc.lock();

        for mut circle_pos in circle_positions {
            if circle_pos == player_position {
                continue;
            }

            let Some(tile) = game.get_tile(circle_pos.get_x(), circle_pos.get_y())
            else {
                continue;
            };

            if mapping
                .get_tile(game.get_room(), circle_pos.get_x(), circle_pos.get_y())
                .is_none()
            {
                continue;
            }

            if circle_pos == player_position
                || next_position.as_ref() == Some(&circle_pos)
            {
                continue;
            }

            circle_pos.set_z(tile.lock().get_position().get_z());

            if selected_positions.len() < max_pins {
                if rand::thread_rng().gen_bool(0.5) {
                    let pin = PinObject::new(game.create_object_id(), circle_pos.copy());
                    let pin_id = pin.get_id();
                    let pin_position = pin.get_position().copy();
                    pins.push(pin);

                    game.get_events_queue()
                        .lock()
                        .push(Box::new(PinSpawnEvent::new(pin_id, pin_position)));
                    selected_positions.push(circle_pos.copy());

                    if let Some(room_tile) = mapping.get_tile(
                        game.get_room(),
                        circle_pos.get_x(),
                        circle_pos.get_y(),
                    ) {
                        if !room_tile.get_entities().is_empty() {
                            for entity in room_tile.get_entities() {
                                if entity.get_type() != EntityType::Player {
                                    continue;
                                }

                                let Some(player) = entity.as_player() else {
                                    continue;
                                };

                                let Some(room_user) = player.get_room_user() else {
                                    continue;
                                };

                                let Some(game_user) = room_user.get_game_player()
                                else {
                                    continue;
                                };

                                if game_user.lock().is_spectator() {
                                    continue;
                                }

                                dizzy_players.push(Arc::clone(&game_user));
                            }
                        }
                    }
                }
            }
        }

        let pin_ids: Vec<i32> = pins.iter().map(|pin| pin.get_id()).collect();

        game.get_objects().lock().extend(
            pins.into_iter().map(|pin| Arc::new(pin) as Arc<dyn GameObject>),
        );

        // Make all affected players dizzy.
        for dizzy_player in dizzy_players {
            PowerUpUtil::stun_player(
                Arc::clone(&game),
                dizzy_player,
                BattleBallPlayerState::BallBroken,
            );
        }

        // Despawn all pins at their irregular intervals, as seen:
        // https://www.youtube.com/watch?v=yw0MigOIloI&feature=youtu.be&t=94
        // Port note: the Java reference-equality removals are object-id
        // removals (the ids are unique, see `createObjectId`).
        for pin_id in pin_ids {
            let game = Arc::clone(&game);
            GameScheduler::get_instance().schedule(
                {
                    let game = Arc::clone(&game);
                    move || {
                        game.get_events_queue()
                            .lock()
                            .push(Box::new(DespawnObjectEvent::new(pin_id)));

                        game.get_objects()
                            .lock()
                            .retain(|object| object.get_id() != pin_id);
                    }
                },
                (rand::thread_rng().gen_range(12..17) as i64) * 1000,
            );
        }
    }

    /// Mirrors `checkNailTile(GamePlayer)` (the `BattleBallGame` is
    // passed explicitly; the Java version resolves it through
    // `gamePlayer.getGame()`, which the Rust `Game` stub cannot).
    pub fn check_nail_tile(
        game: Arc<BattleBallGame>,
        game_player: &Arc<Mutex<GamePlayer>>,
    ) -> bool {
        let player_position = {
            let player = game_player.lock();
            let player_guard = player.get_player().lock();

            player_guard
                .get_room_user()
                .map(|room_user| room_user.get_position())
                .unwrap_or_default()
        };

        let mut matched_id: Option<i32> = None;

        for object in game.get_objects().lock().iter() {
            // Port note: supertrait (`Any`) methods are not callable on
            // `Arc<dyn GameObject>`; the `&dyn Any` coercion is used.
            let any: &dyn std::any::Any = object.as_ref();

            if let Some(pin) = any.downcast_ref::<PinObject>() {
                if player_position == *pin.get_position() {
                    matched_id = Some(pin.get_id());
                    break;
                }
            }
        }

        if let Some(id) = matched_id {
            game.get_objects().lock().retain(|object| object.get_id() != id);

            game.get_events_queue()
                .lock()
                .push(Box::new(DespawnObjectEvent::new(id)));

            PowerUpUtil::stun_player(
                Arc::clone(&game),
                Arc::clone(game_player),
                BattleBallPlayerState::BallBroken,
            );
            return true;
        }

        false
    }
}
