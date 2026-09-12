//! Mirrors `net.h4bbo.lisbon.game.games.battleball.powerups.TorchHandle`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::enums::battle_ball_colour_state::BattleBallColourState;
use crate::game::games::battleball::enums::battle_ball_tile_state::BattleBallTileState;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::utils::tile_util::TileUtil;
use crate::game::room::room::Room;

pub struct TorchHandle;

impl TorchHandle {
    /// Mirrors `handle(BattleBallGame, GamePlayer, Room)`.
    pub fn handle(
        game: &BattleBallGame,
        game_player: Arc<Mutex<GamePlayer>>,
        _room: &Room,
    ) {
        let player = game_player.lock();

        // Port note: the Java `gamePlayer.getTeam().getId()` is reduced
        // to the team id field (the `GameTeam` lookup is a stub).
        let team_id = player.get_team_id();

        let mut tiles_to_update: Vec<Arc<Mutex<crate::game::games::battleball::battle_ball_tile::BattleBallTile>>> =
            Vec::new();

        let mut next_position = player
            .get_player()
            .lock()
            .get_room_user()
            .map(|room_user| room_user.get_position())
            .unwrap_or_default();

        loop {
            let tile = game.get_tile(next_position.get_x(), next_position.get_y());

            let valid = match &tile {
                Some(tile) => {
                    let tile_guard = tile.lock();
                    TileUtil::is_valid_game_tile(&player, Some(&tile_guard), false)
                }
                None => false,
            };

            if !valid {
                break;
            }

            next_position = next_position.get_square_in_front();

            let tile = game.get_tile(next_position.get_x(), next_position.get_y());

            let valid = match &tile {
                Some(tile) => {
                    let tile_guard = tile.lock();
                    TileUtil::is_valid_game_tile(&player, Some(&tile_guard), false)
                }
                None => false,
            };

            if !valid {
                break;
            }

            if let Some(tile) = tile {
                tiles_to_update.push(tile);
            }
        }

        let mut fill_tiles = game.get_fill_tiles_queue().lock();
        let mut update_tiles = game.get_update_tiles_queue().lock();

        for tile_arc in &tiles_to_update {
            let mut tile = tile_arc.lock();

            if tile.get_state() == BattleBallTileState::Sealed {
                continue;
            }

            if tile.get_colour() == BattleBallColourState::Disabled {
                continue;
            }

            let mut state = tile.get_state();
            let colour = tile.get_colour();

            if state == BattleBallTileState::Default {
                state = BattleBallTileState::Touched; // Don't make it 4 hits, make it 3
            }

            let new_state = if colour.get_colour_id() != team_id {
                BattleBallTileState::Clicked
            } else {
                // Port note: the Java `getStateById` `null` on an unknown
                // id is a `continue` here.
                match BattleBallTileState::get_state_by_id(state.get_tile_state_id() + 1) {
                    Some(state) => state,
                    None => continue,
                }
            };

            let new_colour = BattleBallColourState::get_colour_by_id(team_id)
                .unwrap_or(BattleBallColourState::Disabled);

            tile.get_new_points(&player, new_state, new_colour);

            tile.set_colour(new_colour);
            tile.set_state(new_state);

            if new_state == BattleBallTileState::Sealed {
                tile.check_fill(game, &player, &mut fill_tiles);
            }

            update_tiles.push(Arc::clone(tile_arc));
        }
    }
}
