//! Mirrors `net.h4bbo.lisbon.game.games.battleball.BattleBallTile`.
//+ Port note: the Java class extends `GameTile`; Rust has no
// inheritance, so the `GameTile` base is embedded. The Java
// `List<BattleBallTile>`s hold shared tile references; here the tiles
// are `Arc<Mutex<...>>` handles and the `Arc` is passed alongside the
// locked tile where the Java code re-adds `this` to a list.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::enums::battle_ball_colour_state::BattleBallColourState;
use crate::game::games::battleball::enums::battle_ball_tile_state::BattleBallTileState;
use crate::game::games::battleball::battle_ball_power_up::BattleBallPowerUp;
use crate::game::games::battleball::powerups::nail_box_handle::NailBoxHandle;
use crate::game::games::game_event::GameEvent;
use crate::game::games::game_object::GameObject;
use crate::game::games::game_tile::GameTile;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::utils::flood_fill::FloodFill;
use crate::game::games::utils::score_reference::ScoreReference;
use crate::game::pathfinder::position::Position;

pub struct BattleBallTile {
    base: GameTile,
    //+ Port note: the Java `colour` / `state` fields start out `null`;
    // the `DEFAULT` defaults avoid panics in the consumers ported here.
    colour: BattleBallColourState,
    state: BattleBallTileState,
    points_referece: Vec<ScoreReference>,
}

impl BattleBallTile {
    /// Mirrors the `BattleBallTile(Position)` constructor (the tile
    // handles are shared `Arc`s, as the Java lists hold the same
    // tile references).
    pub fn new(position: Position) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            base: GameTile::new(position),
            colour: BattleBallColourState::Default,
            state: BattleBallTileState::Default,
            points_referece: Vec::new(),
        }))
    }

    /// Mirrors `getPlayers(Game, Position)` (the Java `Game` parameter is
    // a `BattleBallGame` here, since the Rust `Game` stub cannot be
    // downcast).
    pub fn get_players(
        &self,
        game: &BattleBallGame,
        position: &Position,
    ) -> Vec<Arc<Mutex<GamePlayer>>> {
        let __mapping_arc = game.get_room().get_mapping();
let mapping = __mapping_arc.lock();
        let Some(tile) = mapping.get_tile(game.get_room(), position.get_x(), position.get_y())
        else {
            return Vec::new();
        };

        let mut game_players: Vec<Arc<Mutex<GamePlayer>>> = Vec::new();

        for entity in tile.get_entities() {
            if entity.get_type() != EntityType::Player {
                continue;
            }

            let Some(player) = entity.as_player() else {
                continue;
            };

            let Some(room_user) = player.get_room_user() else {
                continue;
            };

            let Some(game_user) = room_user.get_game_player() else {
                continue;
            };

            if game_players.iter().any(|p| Arc::ptr_eq(p, &game_user)) {
                continue;
            }

            if room_user.get_position() != *self.base.get_position() {
                continue;
            }

            game_players.push(game_user);
        }

        game_players
    }

    /// Mirrors `interact(GamePlayer, List<GameObject>, List<GameEvent>,
    // List<BattleBallTile>, List<BattleBallTile>)` (the `BattleBallGame`
    // is passed explicitly; the Java version resolves it through
    // `gamePlayer.getGame()`, which the Rust `Game` stub cannot).
    pub fn interact(
        self_arc: Arc<Mutex<BattleBallTile>>,
        game: Arc<BattleBallGame>,
        game_player: &Arc<Mutex<GamePlayer>>,
        objects: &mut Vec<Box<dyn GameObject>>,
        events: &mut Vec<Box<dyn GameEvent>>,
        update_tiles: &mut Vec<Arc<Mutex<BattleBallTile>>>,
        update_fill_tiles: &mut Vec<Arc<Mutex<BattleBallTile>>>,
    ) {
        let mut tile = self_arc.lock();
        let player = game_player.lock();

        BattleBallPowerUp::check_power_up(&game, &tile, game_player, objects, events);

        if NailBoxHandle::check_nail_tile(Arc::clone(&game), game_player) {
            return;
        }

        if BattleBallPowerUp::has_used_power(
            &mut tile,
            &game,
            &player,
            update_tiles,
            update_fill_tiles,
            &self_arc,
        ) {
            return;
        }

        tile.change_state(&game, &player, update_tiles, update_fill_tiles, &self_arc);
    }

    /// Mirrors `changeState(GamePlayer, List<BattleBallTile>,
    // List<BattleBallTile>)`.
    fn change_state(
        &mut self,
        game: &BattleBallGame,
        game_player: &GamePlayer,
        update_tiles: &mut Vec<Arc<Mutex<BattleBallTile>>>,
        update_fill_tiles: &mut Vec<Arc<Mutex<BattleBallTile>>>,
        self_arc: &Arc<Mutex<BattleBallTile>>,
    ) {
        if self.get_colour() == BattleBallColourState::Disabled {
            return;
        }

        let state = self.get_state();
        let colour = self.get_colour();

        // The Java `NullPointerException` equivalent is an early return.
        let Some(team) = game_player.get_team() else {
            return;
        };

        if colour == BattleBallColourState::Disabled {
            return;
        }

        if state != BattleBallTileState::Sealed {
            //+ Port note: the Java initial `this.getState()` value is
            // always reassigned before use; the Rust binding is
            // uninitialised.
            let mut new_state;
            let mut new_colour = self.get_colour();

            if colour.get_colour_id() == team.lock().get_id() {
                let Some(next) =
                    BattleBallTileState::get_state_by_id(state.get_tile_state_id() + 1)
                else {
                    return; // Java `NullPointerException` equivalent.
                };
                new_state = next;
            } else {
                if game.get_map_id() == 5 {
                    // Barebones classic takes 4 hits.
                    new_state = BattleBallTileState::Touched;
                } else {
                    new_state = BattleBallTileState::Clicked;
                }
                //+ Port note: `BattleBallColourState.getColourById` returns
                // `None` for unknown ids; the Java `NullPointerException`
                // equivalent uses `DISABLED`.
                new_colour = BattleBallColourState::get_colour_by_id(team.lock().get_id())
                    .unwrap_or(BattleBallColourState::Disabled);
            }

            self.get_new_points(game_player, new_state, new_colour);

            self.colour = new_colour;
            self.state = new_state;

            if new_state == BattleBallTileState::Sealed {
                self.check_fill(game, game_player, update_fill_tiles);
            }

            update_tiles.push(Arc::clone(self_arc));
        }
    }

    /// Mirrors `getNewPoints(GamePlayer, BattleBallTileState,
    // BattleBallColourState)`.
    pub fn get_new_points(
        &mut self,
        game_player: &GamePlayer,
        new_state: BattleBallTileState,
        new_colour: BattleBallColourState,
    ) {
        // The Java `NullPointerException` equivalent is an early return.
        let Some(team) = game_player.get_team() else {
            return;
        };

        let mut new_points = -1;

        if new_state == BattleBallTileState::Touched {
            new_points = 2;
        } else if new_state == BattleBallTileState::Clicked {
            new_points = 6;
        } else if new_state == BattleBallTileState::Pressed {
            new_points = 10;
        } else if new_state == BattleBallTileState::Sealed {
            if self.colour == new_colour {
                new_points = 14;
            }
        }

        // If the user stole the tile, add the points!
        if self.colour != new_colour && self.colour != BattleBallColourState::Default {
            new_points += 2;
        }

        if self.colour != new_colour {
            // Clear the team's previous scores.
            self.points_referece
                .retain(|reference| {
                    reference.get_game_team().get_id() == team.lock().get_id()
                });
        }

        if new_points != -1 {
            let by = game_player
                .get_harlequin_player()
                .map(|harlequin| harlequin.lock().get_user_id())
                .unwrap_or(game_player.get_user_id());

            self.points_referece
                .push(ScoreReference::new(new_points, team.lock().clone_arc(), by));
        }
    }

    /// Mirrors `checkFill(GamePlayer, Collection<BattleBallTile>)`.
    pub fn check_fill(
        &self,
        game: &BattleBallGame,
        game_player: &GamePlayer,
        update_fill_tiles: &mut Vec<Arc<Mutex<BattleBallTile>>>,
    ) {
        // The team is only used by the commented-out `addSealedPoints`
        // in the Java source.
        let _team = game_player.get_team();

        for neighbour in FloodFill::neighbours(game, self.base.get_position()) {
            let Some(neighbour) = neighbour else {
                continue;
            };

            {
                let neighbour_guard = neighbour.lock();

                if neighbour_guard.get_state() == BattleBallTileState::Sealed
                    || neighbour_guard.get_colour() == BattleBallColourState::Disabled
                {
                    continue;
                }
            }

            let fill_tiles = FloodFill::get_fill(game, game_player, Arc::clone(&neighbour));

            if fill_tiles.len() > 0 {
                for filled_tile in fill_tiles {
                    let Some(filled_tile) = filled_tile else {
                        continue;
                    };

                    let mut guard = filled_tile.lock();

                    if guard.get_state() == BattleBallTileState::Sealed {
                        continue;
                    }

                    guard.set_colour(self.get_colour());
                    guard.set_state(BattleBallTileState::Sealed);

                    update_fill_tiles.push(Arc::clone(&filled_tile));
                }
            }
        }
    }

    /// Mirrors `getColour()`.
    pub fn get_colour(&self) -> BattleBallColourState {
        self.colour
    }

    /// Mirrors `setColour(BattleBallColourState)`.
    pub fn set_colour(&mut self, colour: BattleBallColourState) {
        self.colour = colour;
    }

    /// Mirrors `getState()`.
    pub fn get_state(&self) -> BattleBallTileState {
        self.state
    }

    /// Mirrors `setState(BattleBallTileState)`.
    pub fn set_state(&mut self, state: BattleBallTileState) {
        self.state = state;
    }

    /// Mirrors `getPosition()` (inherited from `GameTile`).
    pub fn get_position(&self) -> &Position {
        self.base.get_position()
    }

    /// Mirrors `getPointsReferece()` (the `CopyOnWriteArrayList` is a
    // plain `Vec`).
    pub fn get_points_referece(&self) -> &[ScoreReference] {
        &self.points_referece
    }

    /// Mirrors `getPointsReferece().clear()` (exposed separately since the
    // Java list is returned by reference).
    pub fn clear_points_referece(&mut self) {
        self.points_referece.clear();
    }

    /// Mirrors `isSpawnOccupied()` (inherited from `GameTile`).
    pub fn is_spawn_occupied(&self) -> bool {
        self.base.is_spawn_occupied()
    }

    /// Mirrors `setSpawnOccupied(boolean)` (inherited from `GameTile`).
    pub fn set_spawn_occupied(&mut self, spawn_occupied: bool) {
        self.base.set_spawn_occupied(spawn_occupied);
    }
}
