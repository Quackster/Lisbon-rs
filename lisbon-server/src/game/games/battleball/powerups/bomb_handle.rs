//! Mirrors `net.h4bbo.lisbon.game.games.battleball.powerups.BombHandle`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::enums::battle_ball_player_state::BattleBallPlayerState;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::utils::power_up_util::PowerUpUtil;
use crate::game::games::utils::tile_util::TileUtil;
use crate::game::pathfinder::rotation::Rotation;
use crate::game::room::room::Room;
use crate::game::room::mapping::room_tile::RoomTile;

pub struct BombHandle;

impl BombHandle {
    /// Mirrors `handle(BattleBallGame, GamePlayer, Room)`.
    pub fn handle(
        game: Arc<BattleBallGame>,
        game_player: Arc<Mutex<GamePlayer>>,
        _room: &Room,
    ) {
        let player_position;

        {
            let player = game_player.lock();

            if let Some(room_user) = player.get_player().lock().get_room_user() {
                room_user.stop_walking();
                room_user.set_walking_allowed(false);
            }

            player_position = player
                .get_player()
                .lock()
                .get_room_user()
                .map(|room_user| room_user.get_position())
                .unwrap_or_default();
        }

        let mut stunned_players: Vec<Arc<Mutex<GamePlayer>>> = Vec::new();

        let __mapping_arc = game.get_room().get_mapping();
let mapping = __mapping_arc.lock();

        for position in player_position.get_circle(3) {
            let Some(room_tile) =
                mapping.get_tile(game.get_room(), position.get_x(), position.get_y())
            else {
                continue;
            };

            drop(room_tile);

            if !RoomTile::is_valid_tile(game.get_room(), None, &position) {
                continue;
            }

            let Some(battleball_tile) = game.get_tile(position.get_x(), position.get_y()) else {
                continue;
            };

            if TileUtil::undo_tile_attributes(&mut *battleball_tile.lock()) {
                game.get_update_tiles_queue().lock().push(battleball_tile);
            }
        }

        for position in player_position.get_circle(7) {
            let Some(room_tile) =
                mapping.get_tile(game.get_room(), position.get_x(), position.get_y())
            else {
                continue;
            };

            drop(room_tile);

            if !RoomTile::is_valid_tile(game.get_room(), None, &position) {
                continue;
            }

            let Some(battleball_tile) = game.get_tile(position.get_x(), position.get_y()) else {
                continue;
            };

            stunned_players.extend(
                battleball_tile.lock().get_players(&game, &position),
            );
        }

        if !stunned_players.iter().any(|p| Arc::ptr_eq(p, &game_player)) {
            stunned_players.push(Arc::clone(&game_player));
        }

        for stunned_player in stunned_players {
            {
                let stunned = stunned_player.lock();

                if let Some(room_user) = stunned.get_player().lock().get_room_user() {
                    room_user.stop_walking();
                    room_user.set_walking_allowed(false);
                }

                // Move the player away from the blast radius:
                // https://www.youtube.com/watch?v=cP3bvGOx53o&feature=youtu.be&t=242
                if !Arc::ptr_eq(&stunned_player, &game_player) {
                    let from = stunned
                        .get_player()
                        .lock()
                        .get_room_user()
                        .map(|room_user| room_user.get_position())
                        .unwrap_or_default();
                    let towards = player_position.copy();

                    let temporary_rotation =
                        Rotation::calculate_walk_direction(&from, &towards);

                    let mut push_back = from.copy();
                    push_back.set_rotation(temporary_rotation);
                    let push_back = push_back.get_square_behind();

                    let Some(battleball_tile) =
                        game.get_tile(push_back.get_x(), push_back.get_y())
                    else {
                        continue;
                    };

                    let valid = {
                        let tile_guard = battleball_tile.lock();
                        TileUtil::is_valid_game_tile(&stunned, Some(&tile_guard), true)
                    };

                    if valid {
                        let player_guard = stunned.get_player().lock();
                        let room_user = player_guard.get_room_user();

                        if let Some(room_user) = room_user {
                            room_user.warp(&push_back, false, false);
                        }
                    }
                }
            }

            PowerUpUtil::stun_player(
                Arc::clone(&game),
                Arc::clone(&stunned_player),
                BattleBallPlayerState::Stunned,
            );
        }
    }
}
