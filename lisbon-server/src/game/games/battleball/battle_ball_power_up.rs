//! Mirrors `net.h4bbo.lisbon.game.games.battleball.BattleBallPowerUp`.
// Port note: the Java `PowerObject` reference held by the game objects
// list is the *same* object instance the `BattleBallPowerUp` holds;
// here the shared `Arc<PowerObject>` is used for both. The Java
// `Game` reference is a `Weak` (the Java reference cycle is not
// expressible with `Arc` without a leak).
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Weak;

use parking_lot::Mutex;
use rand::Rng;

use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::enums::battle_ball_colour_state::BattleBallColourState;
use crate::game::games::battleball::enums::battle_ball_player_state::BattleBallPlayerState;
use crate::game::games::battleball::enums::battle_ball_power_type::BattleBallPowerType;
use crate::game::games::battleball::enums::battle_ball_tile_state::BattleBallTileState;
use crate::game::games::battleball::events::acquire_power_up_event::AcquirePowerUpEvent;
use crate::game::games::battleball::objects::power_object::PowerObject;
use crate::game::games::battleball::objects::power_up_update_object::PowerUpUpdateObject;
use crate::game::games::battleball::powerups::bomb_handle::BombHandle;
use crate::game::games::battleball::powerups::cannon_handle::CannonHandle;
use crate::game::games::battleball::powerups::harlequin_handle::HarlequinHandle;
use crate::game::games::battleball::powerups::lightbulb_handle::LightbulbHandle;
use crate::game::games::battleball::powerups::nail_box_handle::NailBoxHandle;
use crate::game::games::battleball::powerups::spring_handle::SpringHandle;
use crate::game::games::battleball::powerups::torch_handle::TorchHandle;
use crate::game::games::battleball::powerups::vacuum_handle::VacuumHandle;
use crate::game::games::battleball::battle_ball_tile::BattleBallTile;
use crate::game::games::game_event::GameEvent;
use crate::game::games::game_object::GameObject;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::utils::tile_util::TileUtil;
use crate::game::pathfinder::position::Position;

pub struct BattleBallPowerUp {
    id: i32,
    object: Arc<PowerObject>,
    time_to_despawn: AtomicI32,
    tile: Arc<Mutex<BattleBallTile>>,
    position: Position,
    game: Weak<BattleBallGame>,
    player_holding: Option<i32>,
    power_type: BattleBallPowerType,
}

impl BattleBallPowerUp {
    /// Mirrors the `BattleBallPowerUp(int, BattleBallGame,
    // BattleBallTile)` constructor (the game back-reference is a
    // `Weak`, see the module note).
    pub fn new(
        id: i32,
        game: Weak<BattleBallGame>,
        tile: Arc<Mutex<BattleBallTile>>,
    ) -> Arc<Mutex<Self>> {
        let position = tile.lock().get_position().copy();

        // Port note: the Java `ThreadLocalRandom.nextInt(20, 31)` is
        // `gen_range(20..31)`.
        let time_to_despawn = rand::thread_rng().gen_range(20..31);

        let allowed = match game.upgrade() {
            Some(game) => game.get_allowed_power_ups(),
            None => Vec::new(),
        };

        // Port note: the Java `IndexOutOfBoundsException` on an empty
        // list is swallowed by the `gameTick` catch; the `QUESTION_MARK`
        // default is the Rust equivalent.
        let power_type = if allowed.is_empty() {
            BattleBallPowerType::QuestionMark
        } else {
            let index = rand::thread_rng().gen_range(0..allowed.len());
            BattleBallPowerType::get_by_id(allowed[index])
                .unwrap_or(BattleBallPowerType::QuestionMark)
        };

        Arc::new_cyclic(|weak| {
            Mutex::new(Self {
                id,
                object: Arc::new(PowerObject::new(weak.clone())),
                time_to_despawn: AtomicI32::new(time_to_despawn),
                tile,
                position,
                game,
                player_holding: None,
                power_type,
            })
        })
    }

    /// Mirrors `usePower(GamePlayer, Position)`.
    pub fn use_power(&self, game_player: Arc<Mutex<GamePlayer>>, _position: &Position) {
        let Some(game) = self.game.upgrade() else {
            return;
        };

        let room = game.get_room();

        match self.power_type {
            BattleBallPowerType::BoxOfPins => {
                NailBoxHandle::handle(Arc::clone(&game), Arc::clone(&game_player), room)
            }
            BattleBallPowerType::Flashlight => {
                TorchHandle::handle(&game, Arc::clone(&game_player), room)
            }
            BattleBallPowerType::Lightblub => {
                LightbulbHandle::handle(&game, Arc::clone(&game_player), room)
            }
            BattleBallPowerType::Drill => {
                VacuumHandle::handle(Arc::clone(&game), Arc::clone(&game_player), room)
            }
            BattleBallPowerType::Spring => {
                SpringHandle::handle(Arc::clone(&game), Arc::clone(&game_player), room)
            }
            BattleBallPowerType::Harlequin => {
                HarlequinHandle::handle(Arc::clone(&game), Arc::clone(&game_player), room)
            }
            BattleBallPowerType::Cannon => {
                CannonHandle::handle(Arc::clone(&game), Arc::clone(&game_player), room)
            }
            BattleBallPowerType::Bomb => {
                BombHandle::handle(Arc::clone(&game), Arc::clone(&game_player), room)
            }
            BattleBallPowerType::QuestionMark => {}
        }
    }

    /// Mirrors `hasUsedPower(BattleBallTile, GamePlayer,
    // List<BattleBallTile>, List<BattleBallTile>)` (the
    // `BattleBallGame` and the tile `Arc` are passed explicitly; the
    // Java version resolves the game through `gamePlayer.getGame()`,
    // which the Rust `Game` stub cannot).
    pub fn has_used_power(
        tile: &mut BattleBallTile,
        game: &BattleBallGame,
        game_player: &GamePlayer,
        update_tiles: &mut Vec<Arc<Mutex<BattleBallTile>>>,
        update_fill_tiles: &mut Vec<Arc<Mutex<BattleBallTile>>>,
        self_arc: &Arc<Mutex<BattleBallTile>>,
    ) -> bool {
        let colour = tile.get_colour();
        let state = tile.get_state();

        if colour == BattleBallColourState::Disabled {
            return false;
        }

        // Port note: the Java `NullPointerException` equivalent (a
        // missing team) is an early return.
        let Some(_team) = game_player.get_team() else {
            return false;
        };

        if game_player.get_player_state() == BattleBallPlayerState::HighJumps {
            if state == BattleBallTileState::Sealed {
                return true;
            }

            tile.clear_points_referece();

            let new_colour =
                BattleBallColourState::get_colour_by_id(game_player.get_team_id())
                    .unwrap_or(BattleBallColourState::Disabled);
            tile.set_colour(new_colour);
            tile.set_state(BattleBallTileState::Sealed);

            tile.get_new_points(game_player, BattleBallTileState::Sealed, new_colour);
            tile.check_fill(game, game_player, update_fill_tiles);

            update_tiles.push(Arc::clone(self_arc));
            return true;
        }

        if game_player.get_player_state() == BattleBallPlayerState::CleaningTiles {
            if TileUtil::undo_tile_attributes(tile) {
                update_tiles.push(Arc::clone(self_arc));
            }
            return true;
        }

        false
    }

    /// Mirrors `checkPowerUp(BattleBallTile, GamePlayer,
    // List<GameObject>, List<GameEvent>)` (the `BattleBallGame` and the
    // `GamePlayer` handle are passed explicitly; the Java version
    // resolves the game through `gamePlayer.getGame()`, which the Rust
    // `Game` stub cannot).
    pub fn check_power_up(
        game: &BattleBallGame,
        tile: &BattleBallTile,
        game_player: &Arc<Mutex<GamePlayer>>,
        objects: &mut Vec<Box<dyn GameObject>>,
        events: &mut Vec<Box<dyn GameEvent>>,
    ) {
        let game_player_arc = game_player;
        let game_player = game_player.lock();
        let mut power_up: Option<Arc<Mutex<BattleBallPowerUp>>> = None;

        for power in game.get_active_powers() {
            if power.lock().get_tile().lock().get_position() == tile.get_position() {
                power_up = Some(Arc::clone(&power));
                break;
            }
        }

        let Some(power_up) = power_up else {
            return;
        };

        let user_id = game_player.get_user_id();

        {
            let mut stored = game.get_stored_powers().lock();

            if !stored.contains_key(&user_id) {
                stored.insert(user_id, Vec::new());
            }
        }

        // Select a random power up if it's a question mark.
        {
            let mut power = power_up.lock();

            if power.get_power_type() == BattleBallPowerType::QuestionMark {
                // Create a new list without the question mark.
                let mut power_ups = game.get_allowed_power_ups();
                power_ups.retain(|id| {
                    *id != BattleBallPowerType::QuestionMark.get_power_up_id()
                });

                if !power_ups.is_empty() {
                    let index = rand::thread_rng().gen_range(0..power_ups.len());

                    if let Some(new_type) = BattleBallPowerType::get_by_id(power_ups[index]) {
                        power.set_power_type(new_type);
                    }
                }
            }
        }

        {
            let mut stored = game.get_stored_powers().lock();
            let Some(list) = stored.get_mut(&user_id) else {
                return;
            };

            list.clear();
            list.push(Arc::clone(&power_up));
        }

        {
            let mut active = game.get_active_powers_handle().lock();
            active.retain(|power| !Arc::ptr_eq(power, &power_up));
        }

        // Port note: the Java reference-equality removal
        // (`getObjects().remove(powerUp.getObject())`) is an object-id
        // removal (the ids are unique, see `createObjectId`).
        let object_id = power_up.lock().get_id();

        game.get_objects().lock().retain(|object| object.get_id() != object_id);

        power_up.lock().set_time_to_despawn(15);
        power_up
            .lock()
            .set_player_holding(Some(game_player.get_object_id()));

        events.push(Box::new(AcquirePowerUpEvent::new(
            Arc::clone(game_player_arc),
            Arc::clone(&power_up),
        )));
        objects.push(Box::new(
            PowerUpUpdateObject::new(Arc::clone(&power_up)),
        ));
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getPowerType()`.
    pub fn get_power_type(&self) -> BattleBallPowerType {
        self.power_type
    }

    /// Mirrors `setPowerType(BattleBallPowerType)`.
    pub fn set_power_type(&mut self, power_type: BattleBallPowerType) {
        self.power_type = power_type
    }

    /// Mirrors `getPosition()`.
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// Mirrors `getTile()`.
    pub fn get_tile(&self) -> &Arc<Mutex<BattleBallTile>> {
        &self.tile
    }

    /// Mirrors `getTimeToDespawn().get()`.
    pub fn get_time_to_despawn(&self) -> i32 {
        self.time_to_despawn.load(Ordering::SeqCst)
    }

    /// Mirrors `getTimeToDespawn().decrementAndGet()`.
    pub fn decrement_get_time_to_despawn(&self) -> i32 {
        self.time_to_despawn.fetch_sub(1, Ordering::SeqCst) - 1
    }

    /// Mirrors `getTimeToDespawn().set(int)`.
    pub fn set_time_to_despawn(&self, time: i32) {
        self.time_to_despawn.store(time, Ordering::SeqCst)
    }

    /// Mirrors `setPlayerHolding(GamePlayer)` (the Java `GamePlayer`
    // reference is reduced to its object id).
    pub fn set_player_holding(&mut self, player_holding: Option<i32>) {
        self.player_holding = player_holding
    }

    /// Mirrors `getPlayerHolding()`.
    pub fn get_player_holding(&self) -> i32 {
        self.player_holding.unwrap_or(-1)
    }

    /// Mirrors `getObject()`.
    pub fn get_object(&self) -> &Arc<PowerObject> {
        &self.object
    }
}
