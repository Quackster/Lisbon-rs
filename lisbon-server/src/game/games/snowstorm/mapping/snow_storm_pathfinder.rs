//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.mapping.SnowStormPathfinder`.
// Port note: the Java `getOppositionPlayer` / `isBlockedTile` check
// `getNextGoal().equals(player)`, comparing a `Position` with a
// `GamePlayer` (always false); that comparison is omitted here.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::snowstorm::objects::snowball_object::{SnowballObject, SnowballTrajectory};
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::game::games::snowstorm::util::snow_storm_activity_state::SnowStormActivityState;
use crate::game::pathfinder::pathfinder::Pathfinder;
use crate::game::pathfinder::position::Position;

/// Mirrors the `SnowStormPathfinder` class (all Java methods are static).
pub struct SnowStormPathfinder;

impl SnowStormPathfinder {
    /// Mirrors `getNextDirection(SnowStormGame, GamePlayer)`.
    // The callers pass the locked guards (the Java takes the
    // unwrapped objects).
    pub fn get_next_direction(
        snow_storm_game: &SnowStormGame,
        game_player: &GamePlayer,
    ) -> Option<Position> {
        let mut positions: Vec<Position> = Vec::new();

        for point in Pathfinder::DIAGONAL_MOVE_POINTS.iter() {
            let Some(current_position) = game_player
                .get_snow_storm_attributes()
                .get_current_position()
            else {
                // Java `NullPointerException` equivalent.
                continue;
            };

            let temp = current_position.copy().add(point);

            if !Self::is_valid_tile(snow_storm_game, game_player, &temp) {
                continue;
            }

            positions.push(temp);
        }

        let goal = game_player
            .get_snow_storm_attributes()
            .get_walk_goal()
            .cloned();

        positions.sort_by(|a, b| {
            let goal_a = goal.as_ref().map(|g| a.get_distance_squared(g)).unwrap_or(0);
            let goal_b = goal.as_ref().map(|g| b.get_distance_squared(g)).unwrap_or(0);
            goal_a.cmp(&goal_b)
        });

        positions.into_iter().next()
    }

    /// Mirrors `isValidTile(SnowStormGame, GamePlayer, Position)`.
    fn is_valid_tile(
        snow_storm_game: &SnowStormGame,
        game_player: &GamePlayer,
        tmp: &Position,
    ) -> bool {
        for player in snow_storm_game.get_active_players() {
            let player = player.lock();

            if player
                .get_player()
                .lock()
                .get_details()
                .get_id()
                == game_player
                    .get_player()
                    .lock()
                    .get_details()
                    .get_id()
            {
                continue;
            }

            if game_player
                .get_snow_storm_attributes()
                .get_current_position()
                .map(|c| c == tmp)
                .unwrap_or(false)
            {
                return false;
            }

            if game_player
                .get_snow_storm_attributes()
                .get_next_goal()
                .map(|g| g == tmp)
                .unwrap_or(false)
            {
                return false;
            }
        }

        let Some(map) = snow_storm_game.get_map() else {
            return false;
        };

        match map.get_tile(tmp) {
            Some(tile) => tile.is_walkable(),
            None => false,
        }
    }

    /// Check if the point (x0, y0) can see point (x1, y1) by drawing a line
    /// and testing for the "blocking" property at each new tile. Returns the
    /// points on the line if it is, in fact, visible. Otherwise, returns an
    /// empty list (rather than null - Efficient Java, item #43).
    ///
    /// http://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    // Mirrors `getMaxVisibility(SnowballObject, int, int, int, int,
    // SnowballObject.SnowballTrajectory)`.
    pub fn get_max_visibility(
        snowball_object: &SnowballObject,
        mut x0: i32,
        mut y0: i32,
        x1: i32,
        y1: i32,
        trajectory: Option<SnowballTrajectory>,
    ) -> Vec<Position> {
        let mut line: Vec<Position> = Vec::new();
        line.push(Position::new_xy(x0, y0));

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut e2 = 0;

        while !(x0 == x1 && y0 == y1) {
            if Self::is_blocked_tile(snowball_object, &Position::new_xy(x0, y0)) {
                line.push(Position::new_xy(x0, y0));
                return line;
            }

            e2 = 2 * err;

            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }

            if e2 < dx {
                err += dx;
                y0 += sy;
            }

            line.push(Position::new_xy(x0, y0));
        }

        line
    }

    /// Mirrors `getOppositionPlayer(SnowStormGame, GamePlayer, Position)`.
    pub fn get_opposition_player(
        snow_storm_game: &Arc<SnowStormGame>,
        thrower: &Arc<Mutex<GamePlayer>>,
        position: &Position,
    ) -> Option<Arc<Mutex<GamePlayer>>> {
        for player in snow_storm_game.get_active_players() {
            let (position_match, stunned) = {
                let guard = player.lock();
                let attrs = guard.get_snow_storm_attributes();
                let position_match = attrs
                    .get_current_position()
                    .map(|c| c == position)
                    .unwrap_or(false);
                let stunned = attrs.get_activity_state()
                    == Some(SnowStormActivityState::ActivityStateStunned);
                (position_match, stunned)
            };

            if position_match {
                if stunned {
                    continue;
                }

                if snow_storm_game.is_opposition_player(thrower, &player) {
                    return Some(player);
                }
            }
        }

        None
    }

    /// Mirrors `isBlockedTile(SnowballObject, Position)`.
    pub fn is_blocked_tile(snowball_object: &SnowballObject, position: &Position) -> bool {
        if snowball_object.get_trajectory() != Some(SnowballTrajectory::LongTrajectory) {
            let game = snowball_object.get_game().clone();
            let thrower = snowball_object.get_thrower().clone();

            for player in game.get_active_players() {
                let (position_match, stunned) = {
                    let guard = player.lock();
                    let attrs = guard.get_snow_storm_attributes();
                    let position_match = attrs
                        .get_current_position()
                        .map(|c| c == position)
                        .unwrap_or(false);
                    let stunned = attrs.get_activity_state()
                        == Some(SnowStormActivityState::ActivityStateStunned);
                    (position_match, stunned)
                };

                if position_match {
                    if stunned {
                        continue;
                    }

                    if game.is_opposition_player(&thrower, &player) {
                        return true;
                    }
                }
            }
        }

        let game = snowball_object.get_game().clone();
        let map = game.get_map();

        match map
            .as_ref()
            .and_then(|map| map.get_tile(position))
        {
            Some(tile) => tile.is_height_blocking(snowball_object.get_trajectory()),
            None => false,
        }
    }
}
