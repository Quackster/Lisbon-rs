//! Mirrors `net.h4bbo.lisbon.game.games.battleball.powerups.CannonHandle`.
//! The Java per-player `try/catch` (exception logging) has no Rust
//! equivalent.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::game_scheduler::GameScheduler;
use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::enums::battle_ball_player_state::BattleBallPlayerState;
use crate::game::games::battleball::enums::battle_ball_tile_state::BattleBallTileState;
use crate::game::games::battleball::enums::battle_ball_colour_state::BattleBallColourState;
use crate::game::games::battleball::events::player_move_event::PlayerMoveEvent;
use crate::game::games::battleball::objects::player_update_object::PlayerUpdateObject;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::utils::power_up_util::PowerUpUtil;
use crate::game::games::utils::tile_util::TileUtil;
use crate::game::pathfinder::position::Position;
use crate::game::room::room::Room;

pub struct CannonHandle;

impl CannonHandle {
    /// Mirrors `handle(BattleBallGame, GamePlayer, Room)`.
    pub fn handle(
        game: Arc<BattleBallGame>,
        game_player: Arc<Mutex<GamePlayer>>,
        _room: &Room,
    ) {
        let mut player = game_player.lock();

        let first_position = {
            let player_guard = player.get_player().lock();
            let Some(room_user) = player_guard.get_room_user() else {
                return;
            };

            room_user.stop_walking();
            room_user.set_walking_allowed(false);

            room_user.get_position()
        };
        let rotation = first_position.get_rotation();

        let mut next_position = first_position.copy();
        let mut tiles_to_update: Vec<Arc<Mutex<crate::game::games::battleball::battle_ball_tile::BattleBallTile>>> =
            Vec::new();

        loop {
            let tile = game.get_tile(
                next_position.get_x(),
                next_position.get_y(),
            );

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

            let tile = game.get_tile(
                next_position.get_x(),
                next_position.get_y(),
            );

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

        if tiles_to_update.is_empty() {
            next_position = first_position.copy();

            // Port note: the Java list can hold a `null` tile here; the
            // `None` equivalent is skipped.
            if let Some(tile) = game.get_tile(
                next_position.get_x(),
                next_position.get_y(),
            ) {
                tiles_to_update.push(tile);
            }
        }

        // Stun the players in the direction of the cannon and make them
        // move out of the way (the Java 200ms scheduled pass).
        GameScheduler::get_instance().schedule(
            {
                let game = Arc::clone(&game);
                let game_player = Arc::clone(&game_player);
                let tiles_to_update = tiles_to_update.clone();
                move || {
        let mut stunned_players: Vec<(Arc<Mutex<GamePlayer>>, Position)> = Vec::new();

        for tile in &tiles_to_update {
            let tile_position = tile.lock().get_position().copy();
            let players = tile.lock().get_players(&game, &tile_position);

            for p in players {
                if Arc::ptr_eq(&p, &game_player) {
                    continue;
                }

                stunned_players.push((p, tile_position.copy()));
            }
        }

        for (stunned_player, pushed_from_base) in stunned_players {
            // Port note: move the player out of the way of the user using the
            // cannon:
            // https://www.youtube.com/watch?v=YX1UZky5pg0&feature=youtu.be&t=98
            {
                let stunned = stunned_player.lock();

                {
                    let room_user_guard = stunned.get_player().lock();
                    if let Some(room_user) = room_user_guard.get_room_user() {
                        if room_user.is_walking() {
                            room_user.stop_walking();
                        }
                    }
                }

                let mut pushed_from = pushed_from_base.copy();
                pushed_from.set_rotation(rotation);

                let pushed_to: Vec<Position> = vec![
                    pushed_from.get_square_right(),
                    pushed_from.get_square_left(),
                ];

                // Find the best position to move the player to.
                let mut set_position: Option<Position> = None;

                for position in pushed_to {
                    let tile = game.get_tile(position.get_x(), position.get_y());

                    let valid = match &tile {
                        Some(tile) => {
                            let tile_guard = tile.lock();
                            TileUtil::is_valid_game_tile(&stunned, Some(&tile_guard), true)
                        }
                        None => false,
                    };

                    if valid {
                        set_position = Some(position);
                        break;
                    }
                }

                if let Some(set_position) = &set_position {
                    game.add_player_move(&PlayerMoveEvent::new(
                        Arc::clone(&stunned_player),
                        set_position.copy(),
                    ));
                }

                // Stun the player.
                drop(stunned);
                PowerUpUtil::stun_player(
                    Arc::clone(&game),
                    Arc::clone(&stunned_player),
                    BattleBallPlayerState::Stunned,
                );

                // Set the player at their new spot.
                if let Some(mut set_position) = set_position {
                    let current_rotation = stunned_player
                        .lock()
                        .get_player()
                        .lock()
                        .get_room_user()
                        .map(|room_user| room_user.get_position())
                        .map(|position| position.get_rotation())
                        .unwrap_or(0);

                    set_position.set_rotation(current_rotation);

                    //stunnedPlayer.getPlayer().getRoomUser().setPosition(setPosition);
                    let p_guard = stunned_player.lock();
                    {
                        let pl_guard = p_guard.get_player().lock();
                        if let Some(room_user) = pl_guard.get_room_user() {
                            room_user.warp(&set_position, false, false);
                        }
                    }
                }
            }
        }
                }
            },
            200,
        );

        let mut fill_tiles = game.get_fill_tiles_queue().lock();
        let mut update_tiles = game.get_update_tiles_queue().lock();

        for tile_arc in &tiles_to_update {
            let mut tile = tile_arc.lock();

            if tile.get_colour() == BattleBallColourState::Disabled {
                continue;
            }

            // Port note: the Java `gamePlayer.getTeam().getId()` is
            // reduced to the team id field (the `GameTeam` lookup is a
            // stub).
            let team_id = player.get_team_id();

            if tile.get_state() == BattleBallTileState::Sealed
                && tile.get_colour().get_colour_id() == team_id
            {
                continue;
            }

            let new_state = BattleBallTileState::Sealed;
            let new_colour = BattleBallColourState::get_colour_by_id(team_id)
                .unwrap_or(BattleBallColourState::Disabled);

            //tile.addSealedPoints(gamePlayer.getTeam());
            tile.get_new_points(&player, new_state, new_colour);

            tile.set_colour(new_colour);
            tile.set_state(new_state);

            tile.check_fill(&game, &player, &mut fill_tiles);
            update_tiles.push(Arc::clone(tile_arc));
        }

        let Some(last_tile) = tiles_to_update.last() else {
            // Port note: the Java `LinkedList.getLast` throws on an
            // empty list (`NullPointerException` equivalent).
            return;
        };

        let mut last_position = last_tile.lock().get_position().copy();
        last_position.set_rotation(rotation);

        player.set_player_state(BattleBallPlayerState::FlyingThroughAir);

        game.add_object_to_queue(Box::new(PlayerUpdateObject::new(
            Arc::clone(&game_player),
        )));
        game.add_player_move(&PlayerMoveEvent::new(
            Arc::clone(&game_player),
            last_position.copy(),
        ));

        // Stun the player 800ms after the launch.
        GameScheduler::get_instance().schedule(
            {
                let game = Arc::clone(&game);
                let game_player = Arc::clone(&game_player);
                move || {
                    PowerUpUtil::stun_player(
                        game,
                        game_player,
                        BattleBallPlayerState::Stunned,
                    );
                }
            },
            800,
        );

        {
            let player_guard = player.get_player().lock();
            let Some(room_user) = player_guard.get_room_user() else {
                return;
            };

            room_user.warp(&last_position, false, false);
        }
    }
}
