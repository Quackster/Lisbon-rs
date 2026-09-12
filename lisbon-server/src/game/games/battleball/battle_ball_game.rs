//! Mirrors `net.h4bbo.lisbon.game.games.battleball.BattleBallGame`.
//!
//! The Java `extends Game` is composition: the `Game` base fields
//! (`id`, `gameCreatorId`, `spectators`, `observers`,
//! `preparingGameSecondsLeft`) live on the `Game` enum wrapper
//! (`GameBase`); the remaining Java `Game` state is held directly on
//! this struct. The `start_game` / `game_prepare` methods hold the
//! Java override parts; the base (`super`) parts live on the `Game`
//! enum methods. The Java `BlockingQueue`s are `Mutex<Vec<...>>`
//! drained via `mem::take`. The `Map<GamePlayer, ...>` key is reduced
//! to the player user id (the Java `GamePlayer` identity is not
//! hashable).
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Weak;

use parking_lot::Mutex;
use rand::Rng;

use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::game::entity::entity::Entity;
use crate::game::games::battleball::battle_ball_power_up::BattleBallPowerUp;
use crate::game::games::battleball::battle_ball_tile::BattleBallTile;
use crate::game::games::battleball::enums::battle_ball_colour_state::BattleBallColourState;
use crate::game::games::battleball::enums::battle_ball_player_state::BattleBallPlayerState;
use crate::game::games::battleball::enums::battle_ball_power_type::BattleBallPowerType;
use crate::game::games::battleball::enums::battle_ball_tile_state::BattleBallTileState;
use crate::game::games::battleball::events::despawn_object_event::DespawnObjectEvent;
use crate::game::games::battleball::events::player_move_event::PlayerMoveEvent;
use crate::game::games::battleball::events::power_up_spawn_event::PowerUpSpawnEvent;
use crate::game::games::battleball::objects::player_object::PlayerObject;
use crate::game::games::enums::game_state::GameState;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game_event::GameEvent;
use crate::game::games::game_object::GameObject;
use crate::game::games::game_manager::GameManager;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::player::game_team::GameTeam;
use crate::game::pathfinder::position::Position;
use crate::game::player::player::Player;
use crate::game::room::mapping::room_tile_state::RoomTileState;
use crate::game::room::models::room_model::RoomModel;
use crate::game::room::room::Room;
use crate::util::config::game_configuration::GameConfiguration;

/// Mirrors `MAX_POWERS_ACTIVE`.
pub const MAX_POWERS_ACTIVE: i32 = 2;

pub struct BattleBallGame {
    self_weak: Weak<BattleBallGame>,
    id: AtomicI32,
    map_id: i32,
    game_type: GameType,
    name: String,
    team_amount: i32,
    game_creator: Arc<Mutex<Player>>,
    teams: HashMap<i32, Arc<Mutex<GameTeam>>>,
    events_queue: Mutex<Vec<Box<dyn GameEvent>>>,
    objects_queue: Mutex<Vec<Box<dyn GameObject>>>,
    objects: Mutex<Vec<Arc<dyn GameObject>>>,
    game_state: GameState,
    total_seconds_left: AtomicI32,
    room: Room,
    room_model: Option<RoomModel>,
    object_id: AtomicI32,
    battleball_tiles: Vec<Vec<Option<Arc<Mutex<BattleBallTile>>>>>,
    tiles: Vec<Arc<Mutex<BattleBallTile>>>,
    allowed_power_ups: Vec<i32>,
    active_powers: Mutex<Vec<Arc<Mutex<BattleBallPowerUp>>>>,
    stored_powers: Mutex<HashMap<i32, Vec<Arc<Mutex<BattleBallPowerUp>>>>>,
    spawned_initial_powers: AtomicBool,
    update_tiles_queue: Mutex<Vec<Arc<Mutex<BattleBallTile>>>>,
    fill_tiles_queue: Mutex<Vec<Arc<Mutex<BattleBallTile>>>>,
}

impl BattleBallGame {
    /// Mirrors the `BattleBallGame(int, int, GameType, String, int,
    // Player, List<Integer>, boolean)` constructor (the `Game` base
    // constructor is mirrored by the `GameBase` state, see the module
    // note).
    pub fn new(
        id: i32,
        map_id: i32,
        game_type: GameType,
        name: String,
        team_amount: i32,
        game_creator: Arc<Mutex<Player>>,
        allowed_power_ups: Vec<i32>,
        _private_game: bool,
    ) -> Arc<Self> {
        Arc::new_cyclic(|weak| {
            let mut allowed_power_ups = allowed_power_ups;

            if allowed_power_ups.len() >= 2 {
                allowed_power_ups.push(BattleBallPowerType::QuestionMark.get_power_up_id());
            }

            let mut teams: HashMap<i32, Arc<Mutex<GameTeam>>> = HashMap::new();
            for i in 0..team_amount {
                teams.insert(i, Arc::new(Mutex::new(GameTeam::new(i))));
            }

            Self {
                self_weak: weak.clone(),
                id: AtomicI32::new(id),
                map_id,
                game_type,
                name,
                team_amount,
                game_creator,
                teams,
                events_queue: Mutex::new(Vec::new()),
                objects_queue: Mutex::new(Vec::new()),
                objects: Mutex::new(Vec::new()),
                game_state: GameState::Waiting,
                // The Java `initialise` sets this from the lifetime
                // seconds; the base `initialise` is not ported (see
                // `Game::start_game`), so it stays at zero.
                total_seconds_left: AtomicI32::new(0),
                room: Room::default(),
                room_model: None,
                object_id: AtomicI32::new(0),
                battleball_tiles: Vec::new(),
                tiles: Vec::new(),
                allowed_power_ups,
                active_powers: Mutex::new(Vec::new()),
                stored_powers: Mutex::new(HashMap::new()),
                spawned_initial_powers: AtomicBool::new(false),
                update_tiles_queue: Mutex::new(Vec::new()),
                fill_tiles_queue: Mutex::new(Vec::new()),
            }
        })
    }

    /// Mirrors `hasEnoughPlayers()`.
    pub fn has_enough_players(&self) -> bool {
        let mut active_team_count = 0;

        for i in 0..self.team_amount {
            if let Some(team) = self.get_team(i) {
                if !team.lock().get_active_players().is_empty() {
                    active_team_count += 1;
                }
            }
        }

        active_team_count > 0
    }

    /// Mirrors the `startGame()` override part (the base
    // `super.startGame()` part is on `Game::start_game`).
    pub fn start_game(&self) {
        self.update_tiles_queue.lock().clear();
        self.fill_tiles_queue.lock().clear();
        self.events_queue.lock().clear();
        self.objects_queue.lock().clear();
    }

    /// Mirrors the `gamePrepare()` override part (the base
    // `super.gamePrepare()` part is inlined below).
    pub fn game_prepare(&self) {
        // `super.gamePrepare()`
        for game_player in self.get_active_players() {
            let mut game_player_guard = game_player.lock();
            game_player_guard.set_xp(0);
            game_player_guard.set_score(0);
        }

        // Despawn all previous powers.
        let mut events = self.events_queue.lock();
        for power_up in self.active_powers.lock().iter() {
            events.push(Box::new(DespawnObjectEvent::new(
                power_up.lock().get_id(),
            )));
        }
        drop(events);

        self.spawned_initial_powers.store(false, Ordering::SeqCst);

        self.stored_powers.lock().clear();
        self.active_powers.lock().clear();

        let ticket_charge =
            GameConfiguration::get_instance().get_integer("battleball.ticket.charge");

        if ticket_charge > 0 {
            for game_player in self.get_active_players() {
                let game_player_guard = game_player.lock();
                let player = game_player_guard.get_player().lock();

                // BattleBall costs 2 tickets.
                CurrencyDao::decrease_tickets(player.get_details(), 2);
            }
        }
    }

    /// Mirrors the `gamePrepareTick()` override part (the Java base
    // method is empty).
    pub fn game_prepare_tick(&self) {
        if !self.spawned_initial_powers.load(Ordering::SeqCst) {
            if MAX_POWERS_ACTIVE > 0 {
                let initial_powers =
                    rand::thread_rng().gen_range(0..MAX_POWERS_ACTIVE + 1);

                for _ in 0..initial_powers {
                    self.check_spawn_power(false);
                }

                self.spawned_initial_powers.store(true, Ordering::SeqCst);
            }
        }
    }

    /// Mirrors `gameStarted()`.
    pub fn game_started(&self) {}

    /// Mirrors `gameTick()`.
    pub fn game_tick(&self) {
        self.check_expire_power();
        self.check_spawn_power(true);
        self.check_stored_expire_power();
    }

    /// Mirrors `checkSpawnPower(boolean)`.
    fn check_spawn_power(&self, do_percent_check: bool) {
        if self.allowed_power_ups.is_empty() || self.map_id == 5 {
            return;
        }

        if self.active_powers.lock().len() >= MAX_POWERS_ACTIVE as usize {
            // There's already an active power so don't spawn another one.
            return;
        }

        if do_percent_check {
            if !(rand::thread_rng().gen::<f64>() < 0.06) {
                return;
            }
        }

        //int powersToSpawn = MAX_POWERS_ACTIVE - this.activePowers.size();
        //for (int i = 0; i < powersToSpawn; i++) {
        let Some(tile) = self.get_random_tile() else {
            return;
        };

        let power_up =
            BattleBallPowerUp::new(self.create_object_id(), self.self_weak.clone(), tile);

        self.events_queue
            .lock()
            .push(Box::new(PowerUpSpawnEvent::new(Arc::clone(&power_up))));

        self.active_powers.lock().push(power_up.clone());
        self.objects
            .lock()
            .push(Arc::clone(power_up.lock().get_object()) as Arc<dyn GameObject>);
    }

    /// Mirrors `checkExpirePower()`.
    fn check_expire_power(&self) {
        if self.allowed_power_ups.is_empty() || self.map_id == 5 {
            return;
        }

        let mut active = self.active_powers.lock();
        let expired: Vec<Arc<Mutex<BattleBallPowerUp>>> = active
            .iter()
            .filter(|power_up| self.expire_power(power_up))
            .cloned()
            .collect();

        for power in &expired {
            active.retain(|power_up| !Arc::ptr_eq(power_up, power));

            let object_id = power.lock().get_id();

            self.objects.lock().retain(|object| object.get_id() != object_id);
        }
    }

    /// Mirrors `checkStoredExpirePower()`.
    fn check_stored_expire_power(&self) {
        let mut stored = self.stored_powers.lock();

        for powers in stored.values_mut() {
            let expired: Vec<Arc<Mutex<BattleBallPowerUp>>> = powers
                .iter()
                .filter(|power| self.expire_power(power))
                .cloned()
                .collect();

            for power in &expired {
                powers.retain(|p| !Arc::ptr_eq(p, power));
            }
        }
    }

    /// Mirrors `expirePower(BattleBallPowerUp)`.
    fn expire_power(&self, power_up: &Arc<Mutex<BattleBallPowerUp>>) -> bool {
        if power_up.lock().get_time_to_despawn() > 0 {
            if power_up.lock().decrement_get_time_to_despawn() != 0 {
                return false;
            }
        }

        self.events_queue
            .lock()
            .push(Box::new(DespawnObjectEvent::new(
                power_up.lock().get_id(),
            )));
        true
    }

    /// Mirrors `buildMap()`.
    pub fn build_map(&mut self) {
        let manager = GameManager::get_instance();
        let Some(tile_map) = manager.get_battleball_tile_map(self.map_id) else {
            // Java `NullPointerException` equivalent.
            return;
        };

        let Some(model) = self.room_model.as_ref() else {
            // Java `NullPointerException` equivalent.
            return;
        };

        let size_x = model.get_map_size_x() as usize;
        let size_y = model.get_map_size_y() as usize;

        self.battleball_tiles = vec![vec![None; size_y]; size_x];
        self.tiles.clear();

        for y in 0..size_y as i32 {
            for x in 0..size_x as i32 {
                let tile_state = model.get_tile_state(x, y);

                let tile = BattleBallTile::new(Position::new(
                    x,
                    y,
                    model.get_tile_height(x, y),
                ));

                self.battleball_tiles[x as usize][y as usize] = Some(Arc::clone(&tile));
                tile.lock().set_state(BattleBallTileState::Default);

                if tile_state == RoomTileState::Closed {
                    tile.lock().set_colour(BattleBallColourState::Disabled);
                    continue;
                }

                if !tile_map.is_game_tile(x, y) {
                    tile.lock().set_colour(BattleBallColourState::Disabled);
                    continue;
                }

                tile.lock().set_colour(BattleBallColourState::Default);
                self.tiles.push(tile);
            }
        }
    }

    /// Assign spawn points to all team members.
    // Mirrors `assignSpawnPoints()`.
    pub fn assign_spawn_points(&self) {
        for team in self.teams.values() {
            let team = team.lock();
            let game_spawns = GameManager::get_instance().get_game_spawns(
                self.game_type,
                self.map_id,
                team.get_id(),
            );

            //if (gameSpawn == null) {
            //    continue;
            //}

            for p in team.get_players() {
                let p_arc = p;
                let mut p = p_arc.lock();
                let mut spawn_position: Option<Position> = None;

                for game_spawn in &game_spawns {
                    let position = game_spawn.get_position();
                    let occupied = self
                        .battleball_tiles
                        .get(position.get_x() as usize)
                        .and_then(|row| row.get(position.get_y() as usize))
                        .and_then(|tile| tile.clone())
                        .map(|tile| tile.lock().is_spawn_occupied())
                        .unwrap_or(true);

                    if !occupied {
                        let mut position = position.copy();
                        position.set_z(
                            self.room_model
                                .as_ref()
                                .map(|model| {
                                    model.get_tile_height(position.get_x(), position.get_y())
                                })
                                .unwrap_or(0.0),
                        );
                        spawn_position = Some(position);
                        break;
                    }
                }

                let Some(spawn_position) = spawn_position else {
                    continue;
                };

                if self.get_tile(spawn_position.get_x(), spawn_position.get_y()).is_none() {
                    continue;
                }

                p.set_player_state(BattleBallPlayerState::Normal);
                p.set_harlequin_player(None);
                let player_object = Arc::new(PlayerObject::new(Arc::clone(&p_arc)));
                p.set_game_object(Some(player_object.clone() as Arc<dyn GameObject>));

                if p.get_object_id() == -1 {
                    p.set_object_id(self.create_object_id());
                }

                self.objects
                    .lock()
                    .push(player_object.clone() as Arc<dyn GameObject>);

                p.set_spawn_position(spawn_position.copy());

                let player = p.get_player().lock();
                if let Some(room_user) = player.get_room_user() {
                    room_user.stop_walking();
                    room_user.set_next_position(None);
                }
                //p.getPlayer().getRoomUser().setPosition(p.getSpawnPosition().copy());

                let Some(tile) = self.get_tile(spawn_position.get_x(), spawn_position.get_y())
                else {
                    continue;
                };

                // Don't allow anyone to spawn on this tile.
                tile.lock().set_spawn_occupied(true);

                if tile.lock().get_colour() != BattleBallColourState::Disabled {
                    // Set spawn colour.
                    tile.lock().set_colour(
                        BattleBallColourState::get_colour_by_id(team.get_id())
                            .unwrap_or(BattleBallColourState::Disabled),
                    );

                    if self.map_id == 5 {
                        tile.lock().set_state(BattleBallTileState::Touched);
                    } else {
                        tile.lock().set_state(BattleBallTileState::Clicked);
                    }
                }
            }
        }
    }

    /// Find a spawn with given coordinates.
    // Mirrors `findSpawn(boolean, AtomicInteger, AtomicInteger,
    // AtomicInteger)`.
    #[allow(dead_code)]
    fn find_spawn(
        &self,
        _flip: bool,
        mut spawn_x: i32,
        mut spawn_y: i32,
        spawn_rotation: i32,
    ) {
        loop {
            let occupied = self
                .get_tile(spawn_x, spawn_y)
                .map(|tile| tile.lock().is_spawn_occupied())
                .unwrap_or(false);

            if !occupied {
                break;
            }

            if spawn_rotation == 0 {
                // flip ? += 1 : -= 1
                if _flip {
                    spawn_x += 1;
                } else {
                    spawn_x -= 1;
                }
            }

            if spawn_rotation == 2 {
                if _flip {
                    spawn_y -= 1;
                } else {
                    spawn_y += 1;
                }
            }

            if spawn_rotation == 4 {
                if _flip {
                    spawn_x -= 1;
                } else {
                    spawn_x += 1;
                }
            }

            if spawn_rotation == 6 {
                if _flip {
                    spawn_y += 1;
                } else {
                    spawn_y -= 1;
                }
            }
        }
    }

    /// Get if the game still has free tiles to use.
    // Mirrors `canTimerContinue()`.
    pub fn can_timer_continue(&self) -> bool {
        let Some(model) = self.room_model.as_ref() else {
            return false; // Java `NullPointerException` equivalent.
        };

        for y in 0..model.get_map_size_y() {
            for x in 0..model.get_map_size_x() {
                let Some(tile) = self.get_tile(x, y) else {
                    continue;
                };
                let tile = tile.lock();

                if tile.get_colour() == BattleBallColourState::Disabled {
                    continue;
                }

                if tile.get_state() != BattleBallTileState::Sealed {
                    return true;
                }
            }
        }

        false
    }

    /// Mirrors `getRandomTile()`.
    pub fn get_random_tile(&self) -> Option<Arc<Mutex<BattleBallTile>>> {
        let (map_size_x, map_size_y) = match self.room_model.as_ref() {
            Some(model) => {
                (model.get_map_size_x(), model.get_map_size_y())
            }
            None => return None, // Java `NullPointerException` equivalent.
        };

        if map_size_x == 0 || map_size_y == 0 {
            return None;
        }

        'outer: loop {
            let x = rand::thread_rng().gen_range(0..map_size_x);
            let y = rand::thread_rng().gen_range(0..map_size_y);

            let Some(battleball_tile) = self.get_tile(x, y) else {
                continue;
            };

            if battleball_tile.lock().get_colour() == BattleBallColourState::Disabled {
                continue;
            }

            let mapping = self.room.get_mapping();
            let mapping = mapping.lock();
            let Some(tile) = mapping.get_tile(&self.room, x, y) else {
                continue 'outer;
            };

            if !tile.get_entities().is_empty() {
                continue 'outer;
            }

            let active = self.active_powers.lock();
            for power_up in active.iter() {
                if *power_up.lock().get_position() == Position::new_xy(x, y) {
                    continue 'outer;
                }
            }

            return Some(battleball_tile);
        }
    }

    /// Mirrors `getTileMap()`.
    pub fn get_tile_map(&self) -> &Vec<Vec<Option<Arc<Mutex<BattleBallTile>>>>> {
        &self.battleball_tiles
    }

    /// Get the power ups allowed for this match.
    // Mirrors `getAllowedPowerUps()`.
    pub fn get_allowed_power_ups(&self) -> Vec<i32> {
        self.allowed_power_ups.clone()
    }

    /// Get the current active powers on the board.
    // Mirrors `getActivePowers()`.
    pub fn get_active_powers(&self) -> Vec<Arc<Mutex<BattleBallPowerUp>>> {
        self.active_powers.lock().clone()
    }

    /// Mirrors the `activePowers` list access (the Java
    // `CopyOnWriteArrayList` is a `Mutex<Vec>`).
    pub fn get_active_powers_handle(&self) -> &Mutex<Vec<Arc<Mutex<BattleBallPowerUp>>>> {
        &self.active_powers
    }

    /// Get the list of powers stored by player (the key is the player
    // user id, see the module note).
    // Mirrors `getStoredPowers()`.
    pub fn get_stored_powers(
        &self,
    ) -> &Mutex<HashMap<i32, Vec<Arc<Mutex<BattleBallPowerUp>>>>> {
        &self.stored_powers
    }

    /// Get the queue for tiles to be updated, used by power ups.
    // Mirrors `getUpdateTilesQueue()`.
    pub fn get_update_tiles_queue(
        &self,
    ) -> &Mutex<Vec<Arc<Mutex<BattleBallTile>>>> {
        &self.update_tiles_queue
    }

    /// Get the fill tiles queue, used by power ups.
    // Mirrors `getFillTilesQueue()`.
    pub fn get_fill_tiles_queue(&self) -> &Mutex<Vec<Arc<Mutex<BattleBallTile>>>> {
        &self.fill_tiles_queue
    }

    /// Return the list of tiles created for the game.
    // Mirrors `getTiles()`.
    pub fn get_tiles(&self) -> Vec<Arc<Mutex<BattleBallTile>>> {
        self.tiles.clone()
    }

    /// Mirrors `createObjectId()` (the Java `AtomicInteger.incrementAndGet`).
    pub fn create_object_id(&self) -> i32 {
        self.object_id.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Mirrors `addObjectToQueue(GameObject)`.
    pub fn add_object_to_queue(&self, object: Box<dyn GameObject>) {
        self.objects_queue.lock().push(object);
    }

    /// Mirrors `addPlayerMove(PlayerMoveEvent)`.
    pub fn add_player_move(&self, event: &PlayerMoveEvent) {
        let target = event.get_game_player();
        let mut queue = self.events_queue.lock();

        queue.retain(|e| {
            let any: &dyn std::any::Any = e.as_ref();
            match any.downcast_ref::<PlayerMoveEvent>() {
                Some(e) => !Arc::ptr_eq(e.get_game_player(), target),
                None => true,
            }
        });

        queue.push(Box::new(PlayerMoveEvent::new(
            Arc::clone(target),
            event.get_next_position().copy(),
        )));
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id.load(Ordering::SeqCst)
    }

    /// Mirrors `reassignGameId()` (the Java `Game` base method; the
    // concrete `id` field is the single source of truth here).
    pub fn reassign_game_id(&self) {
        self.id
            .store(GameManager::get_instance().create_id(), Ordering::SeqCst)
    }

    /// Mirrors `getMapId()`.
    pub fn get_map_id(&self) -> i32 {
        self.map_id
    }

    /// Mirrors `getGameType()`.
    pub fn get_game_type(&self) -> GameType {
        self.game_type
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `getTeamAmount()`.
    pub fn get_team_amount(&self) -> i32 {
        self.team_amount
    }

    /// Mirrors `getGameState()`.
    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }

    /// Mirrors `getTotalSecondsLeft().get()`.
    pub fn get_total_seconds_left(&self) -> i32 {
        self.total_seconds_left.load(Ordering::SeqCst)
    }

    /// Mirrors `getGameCreator()`.
    pub fn get_game_creator(&self) -> &Arc<Mutex<Player>> {
        &self.game_creator
    }

    /// Mirrors `getTeams()` (the Java `Map` values).
    pub fn get_teams(&self) -> Vec<Arc<Mutex<GameTeam>>> {
        self.teams.values().cloned().collect()
    }

    /// Mirrors `getTeams().get(int)`.
    pub fn get_team(&self, team_id: i32) -> Option<Arc<Mutex<GameTeam>>> {
        self.teams.get(&team_id).cloned()
    }

    /// Mirrors `getActivePlayers()`.
    pub fn get_active_players(&self) -> Vec<Arc<Mutex<GamePlayer>>> {
        let mut game_players = Vec::new();

        for team in self.teams.values() {
            game_players.extend(team.lock().get_active_players());
        }

        game_players
    }

    /// Mirrors `getEventsQueue()` (the Java `BlockingQueue` is a
    // `Mutex<Vec>` drained via `mem::take`).
    pub fn get_events_queue(&self) -> &Mutex<Vec<Box<dyn GameEvent>>> {
        &self.events_queue
    }

    /// Mirrors `getObjectsQueue()`.
    pub fn get_objects_queue(&self) -> &Mutex<Vec<Box<dyn GameObject>>> {
        &self.objects_queue
    }

    /// Mirrors `getObjects()` (the Java `CopyOnWriteArrayList` is a
    // `Mutex<Vec>`; the shared `Arc` object handles mirror the Java
    // shared references).
    pub fn get_objects(&self) -> &Mutex<Vec<Arc<dyn GameObject>>> {
        &self.objects
    }

    /// Mirrors `getRoom()`.
    pub fn get_room(&self) -> &Room {
        &self.room
    }

    /// Mirrors `getRoomModel()`.
    pub fn get_room_model(&self) -> Option<&RoomModel> {
        self.room_model.as_ref()
    }

    /// Mirrors `getTile(int, int)` (the Java `GameTile` is cast to
    // `BattleBallTile` by the callers).
    pub fn get_tile(&self, x: i32, y: i32) -> Option<Arc<Mutex<BattleBallTile>>> {
        self.battleball_tiles
            .get(x as usize)
            .and_then(|row| row.get(y as usize))
            .and_then(|tile| tile.clone())
    }
}
