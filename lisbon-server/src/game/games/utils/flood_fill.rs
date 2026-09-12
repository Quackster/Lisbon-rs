//! Mirrors `net.h4bbo.lisbon.game.games.utils.FloodFill`.
// Port note: the Java `HashSet` (object identity) is a `Vec` checked with
// `Arc` pointer equality.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::enums::battle_ball_colour_state::BattleBallColourState;
use crate::game::games::battleball::battle_ball_tile::BattleBallTile;
use crate::game::games::battleball::enums::battle_ball_tile_state::BattleBallTileState;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::pathfinder::pathfinder::Pathfinder;
use crate::game::pathfinder::position::Position;

pub struct FloodFill;

impl FloodFill {
    // Port note: deviation from the Java signature; the `Game` (a
    // zero-sized Rust stub whose `get_game()` is always `None`) is
    // replaced by the `BattleBallGame` (mirrors the `PowerUpUtil`
    // pattern).
    /// Mirrors `getFill(GamePlayer, BattleBallTile)`.
    pub fn get_fill(
        game: &BattleBallGame,
        game_player: &GamePlayer,
        start_tile: Arc<Mutex<BattleBallTile>>,
    ) -> Vec<Option<Arc<Mutex<BattleBallTile>>>> {
        let mut closed: Vec<Option<Arc<Mutex<BattleBallTile>>>> = Vec::new();
        let mut stack: Vec<Option<Arc<Mutex<BattleBallTile>>>> = vec![Some(start_tile)];

        while let Some(entry) = stack.pop() {
            let Some(tile) = entry else {
                continue;
            };

            for loop_tile in Self::neighbours(game, tile.lock().get_position()) {
                match &loop_tile {
                    None => {
                        closed.clear();
                        return closed;
                    }
                    Some(loop_tile) => {
                        {
                            let loop_tile_guard = loop_tile.lock();

                            if loop_tile_guard.get_colour() == BattleBallColourState::Disabled {
                                closed.clear();
                                return closed;
                            }

                            let colour_match = loop_tile_guard
                                .get_colour()
                                .get_colour_id()
                                != game_player.get_team_id();
                            let not_sealed =
                                loop_tile_guard.get_state() != BattleBallTileState::Sealed;

                            if (colour_match || not_sealed)
                                && !closed.iter().any(|candidate| {
                                    Self::same_tile(candidate.as_ref(), Some(loop_tile))
                                })
                                && !stack.iter().any(|candidate| {
                                    Self::same_tile(candidate.as_ref(), Some(loop_tile))
                                })
                            {
                                stack.push(Some(Arc::clone(loop_tile)));
                            }
                        }
                    }
                }
            }

            closed.push(Some(tile));
        }

        closed
    }

    /// Object identity check (the Java `HashSet` uses `BattleBallTile`
    /// identity, the `null` entries are compared by `Option` shape).
    fn same_tile(
        candidate: Option<&Arc<Mutex<BattleBallTile>>>,
        tile: Option<&Arc<Mutex<BattleBallTile>>>,
    ) -> bool {
        match (candidate, tile) {
            (Some(candidate), Some(tile)) => {
                std::ptr::eq(Arc::as_ptr(candidate), Arc::as_ptr(tile))
            }
            (None, None) => true,
            _ => false,
        }
    }

    /// Mirrors `neighbours(Game, Position)` (the Java `HashSet` is a `Vec`;
    /// the `null` tile entries mirror the Java `null` set entries; the
    /// `Game` is the `BattleBallGame`, see the `get_fill` note).
    pub fn neighbours(
        game: &BattleBallGame,
        position: &Position,
    ) -> Vec<Option<Arc<Mutex<BattleBallTile>>>> {
        let mut battleball_tiles = Vec::new();

        for point in Pathfinder::MOVE_POINTS {
            let tmp = position.copy().add(&point);
            battleball_tiles.push(game.get_tile(tmp.get_x(), tmp.get_y()));
        }

        battleball_tiles
    }
}
