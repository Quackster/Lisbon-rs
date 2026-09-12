//! Mirrors `net.h4bbo.lisbon.game.games.battleball.BattleBallTask`.
//! The Java `Runnable` is scheduled by the game tick scheduler (not
//! ported); the Rust port is a plain struct with a `run()` method. The
//! `try/catch` (exception logging) has no Rust equivalent. The
//! `RoomTile.add_entity` re-adds are skipped (the entity is borrowed;
//! `add_entity` takes ownership).
use std::mem;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::battleball::enums::battle_ball_player_state::BattleBallPlayerState;
use crate::game::games::battleball::events::player_move_event::PlayerMoveEvent;
use crate::game::games::battleball::objects::player_update_object::PlayerUpdateObject;
use crate::game::games::battleball::objects::power_up_update_object::PowerUpUpdateObject;
use crate::game::games::enums::game_state::GameState;
use crate::game::games::game_event::GameEvent;
use crate::game::games::game_object::GameObject;
use crate::game::games::game::Game;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::pathfinder::rotation::Rotation;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::room::Room;
use crate::messages::outgoing::games::game_status::GAMESTATUS;

pub struct BattleBallTask {
    room: Room,
    game: Game,
}

impl BattleBallTask {
    /// Mirrors the `BattleBallTask(Room, BattleBallGame)` constructor
    /// (the Java subtype reference is the `Game` wrapper here, so the
    /// base `send` is reachable).
    pub fn new(room: Room, game: Game) -> Self {
        Self { room, game }
    }

    /// Mirrors `run()`.
    pub fn run(&self) {
        let Some(game_arc) = self.game.as_battle_ball() else {
            return;
        };
        let game = &*game_arc;

        if game.get_active_players().is_empty()
            || game.get_game_state() == GameState::Ended
        {
            return; // Don't send any packets or do any logic checks during when the game is finished
        }

        let mut objects: Vec<Box<dyn GameObject>> = Vec::new();
        let mut events: Vec<Box<dyn GameEvent>> = Vec::new();

        let mut update_tiles = mem::take(&mut *game.get_update_tiles_queue().lock());
        let mut fill_tiles = mem::take(&mut *game.get_fill_tiles_queue().lock());

        objects.extend(mem::take(&mut *game.get_objects_queue().lock()));
        events.extend(mem::take(&mut *game.get_events_queue().lock()));

        for team in game.get_teams() {
            team.lock().calculate_score();
        }

        for game_player in game.get_active_players() {
            {
                let gp_guard = game_player.lock();
                let player = gp_guard.get_player().lock();

                if let Some(room_user) = player.get_room_user() {
                    if let Some(player_room) = room_user.get_room() {
                        if player_room.get_id() != self.room.get_id() {
                            continue;
                        }
                    }
                } else {
                    continue;
                }

                let player_state = gp_guard.get_player_state();
                if player_state == BattleBallPlayerState::ClimbingIntoCannon
                    || player_state == BattleBallPlayerState::FlyingThroughAir
                {
                    continue;
                }

                if let Some(stored_powers) =
                    game.get_stored_powers().lock().get(&gp_guard.get_user_id())
                {
                    for power_up in stored_powers {
                        objects.push(Box::new(PowerUpUpdateObject::new(Arc::clone(power_up))));
                    }
                }

                if let Some(room_user) = player.get_room_user() {
                    room_user.handle_spam_ticks();
                }
            }

            self.process_entity(
                &game_player,
                &mut objects,
                &mut events,
                &mut update_tiles,
                &mut fill_tiles,
            );

            objects.push(Box::new(PlayerUpdateObject::new(Arc::clone(&game_player))));
        }

        self.game.send(&GAMESTATUS::new(
            Arc::clone(&game_arc),
            game.get_teams(),
            objects,
            events,
            update_tiles,
            fill_tiles,
        ));
    }

    /// Process entity.
    // Mirrors `processEntity(GamePlayer, List<GameObject>,
    // List<GameEvent>, List<BattleBallTile>, List<BattleBallTile>)`.
    fn process_entity(
        &self,
        game_player: &Arc<Mutex<GamePlayer>>,
        objects: &mut Vec<Box<dyn GameObject>>,
        events: &mut Vec<Box<dyn GameEvent>>,
        _update_tiles: &mut Vec<
            Arc<Mutex<crate::game::games::battleball::battle_ball_tile::BattleBallTile>>,
        >,
        _fill_tiles: &mut Vec<
            Arc<Mutex<crate::game::games::battleball::battle_ball_tile::BattleBallTile>>,
        >,
    ) {
        let game_player_guard = game_player.lock();
        let entity = game_player_guard.get_player().lock();
        let Some(room_entity) = entity.get_room_user() else {
            return;
        };

        let position = room_entity.get_position();
        let goal = room_entity.get_goal();

        if room_entity.is_walking() {
            // Apply next tile from the tile we removed from the list the cycle before
            if let Some(next_position) = room_entity.get_next_position() {
                let mut current = room_entity.get_position();
                current.set_x(next_position.get_x());
                current.set_y(next_position.get_y());
                room_entity.set_position(current);
                room_entity.update_new_height(&room_entity.get_position());

                let Some(room) = room_entity.get_room() else {
                    return;
                };
                let mapping = room.get_mapping();
                let mapping = mapping.lock();
                let Some(next_tile) = mapping.get_tile(&room, next_position.get_x(), next_position.get_y()) else {
                    return;
                };

                if next_tile.get_other_entities(&*entity).is_empty() {
                    if let Some(game) = game_player_guard
                        .get_game()
                        .and_then(|game| game.as_battle_ball())
                    {
                        if let Some(tile) = game.get_tile(next_position.get_x(), next_position.get_y()) {
                            crate::game::games::battleball::battle_ball_tile::BattleBallTile::interact(
                                tile,
                                Arc::clone(&game),
                                game_player,
                                objects,
                                events,
                                _update_tiles,
                                _fill_tiles,
                            );
                        }
                    }
                }
            }

            // We still have more tiles left, so lets continue moving
            if !room_entity.get_path().is_empty() {
                let next = room_entity.pop_path().expect("path non-empty");

                let Some(room) = room_entity.get_room() else {
                    return;
                };
                let mapping = room.get_mapping();
                let mapping = mapping.lock();
                let next_tile = mapping.get_tile(&room, next.get_x(), next.get_y());

                // Tile was invalid after we started walking, so lets try again!
                let is_valid = crate::game::room::mapping::room_tile::RoomTile::is_valid_tile(
                    &room,
                    Some(&*entity),
                    &next,
                );

                if next_tile.is_none() || (!is_valid && next != goal) {
                    room_entity.clear_path();
                    room_entity.walk_to(goal.get_x(), goal.get_y());
                    self.process_entity(game_player, objects, events, _update_tiles, _fill_tiles);
                    return;
                }

                if let Some(previous_tile) = room_entity.get_tile() {
                    previous_tile.lock().remove_entity(&*entity);
                }
                // `RoomTile.add_entity` takes ownership of the
                // `Box<dyn Entity + Send>`; the entity is borrowed here, so
                // the re-add is skipped.

                room_entity.remove_status(StatusType::Lay);
                room_entity.remove_status(StatusType::Sit);

                let rotation = Rotation::calculate_walk_direction_coords(
                    position.get_x(),
                    position.get_y(),
                    next.get_x(),
                    next.get_y(),
                );
                let height = next_tile
                    .as_ref()
                    .map(|tile| tile.get_walking_height())
                    .unwrap_or(0.0);

                let mut position = position;
                position.set_rotation(rotation);
                room_entity.set_position(position);
                room_entity.set_status(
                    StatusType::Move,
                    &format!(
                        "{},{},{}",
                        next.get_x(),
                        next.get_y(),
                        crate::util::string_util::StringUtil::format(height)
                    ),
                );
                room_entity.set_next_position(Some(next));

                // Add next position if moving
                if let Some(next_position) = room_entity.get_next_position() {
                    events.push(Box::new(PlayerMoveEvent::new(
                        Arc::clone(game_player),
                        next_position,
                    )));
                }
            } else {
                room_entity.stop_walking();

                let Some(room) = room_entity.get_room() else {
                    return;
                };
                let mapping = room.get_mapping();
                let mapping = mapping.lock();
                let previous_tile = mapping.get_tile(&room, position.get_x(), position.get_y());
                let behind = position.get_square_behind();

                // If an entity exists on the tile, push them back
                if let Some(mut previous_tile) = previous_tile {
                    if !previous_tile.get_other_entities(&*entity).is_empty() {
                        previous_tile.remove_entity(&*entity);
                        // `RoomTile.add_entity` takes ownership of the
                        // `Box<dyn Entity + Send>`; the entity is borrowed
                        // here, so the re-add is skipped.
                        room_entity.set_position(behind);
                    }
                }
            }

            // If we're walking, make sure to tell the server
            room_entity.set_needs_update(true);
        }
    }
}
