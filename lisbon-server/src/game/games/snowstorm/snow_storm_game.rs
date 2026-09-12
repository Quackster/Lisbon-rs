//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.SnowStormGame`.
//!
//! The Java `extends Game` is composition: the `Game` base fields
//! (`id`, `gameCreatorId`, `spectators`, `observers`,
//! `preparingGameSecondsLeft`) live on the `Game` enum wrapper
//! (`GameBase`); the remaining Java `Game` state is held directly on
//! this struct. The base `initialise` / `finishGame` bodies (room
//! setup, game history) are not ported.
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, AtomicI64, Ordering};

use parking_lot::Mutex;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::game::entity::entity::Entity;
use crate::game::games::enums::game_state::GameState;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game_object::GameObject;
use crate::game::games::game_manager::GameManager;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::player::game_team::GameTeam;
use crate::game::games::snowstorm::events::snow_storm_delete_object_event::SnowStormDeleteObjectEvent;
use crate::game::games::snowstorm::events::snow_storm_hit_event::SnowStormHitEvent;
use crate::game::games::snowstorm::events::snow_storm_stun_event::SnowStormStunEvent;
use crate::game::games::snowstorm::objects::snow_storm_avatar_object::SnowStormAvatarObject;
use crate::game::games::snowstorm::objects::snow_storm_machine_object::SnowStormMachineObject;
use crate::game::games::snowstorm::mapping::snow_storm_map::SnowStormMap;
use crate::game::games::snowstorm::objects::snowball_object::SnowballObject;
use crate::game::games::snowstorm::snowstorm_maps_manager::SnowStormMapsManager;
use crate::game::games::snowstorm::tasks::snow_storm_game_task::SnowStormGameTask;
use crate::game::games::snowstorm::util::snow_storm_activity_state::SnowStormActivityState;
use crate::game::pathfinder::position::Position;
use crate::game::pathfinder::rotation::Rotation;
use crate::game::player::player::Player;
use crate::game::room::models::room_model::RoomModel;
use crate::game::room::room::Room;
use crate::util::date_util::DateUtil;
use crate::util::config::game_configuration::GameConfiguration;

pub struct SnowStormGame {
    id: AtomicI32,
    map_id: i32,
    game_type: GameType,
    name: String,
    team_amount: i32,
    game_creator: Arc<Mutex<Player>>,
    teams: HashMap<i32, Arc<Mutex<GameTeam>>>,
    objects: Mutex<Vec<Arc<dyn GameObject>>>,
    game_state: Mutex<GameState>,
    room: Mutex<Room>,
    room_model: Mutex<Option<RoomModel>>,
    object_id: AtomicI32,
    total_seconds_left: AtomicI32,
    game_length_choice: i32,
    executing_events: Mutex<Vec<Arc<dyn GameObject>>>,
    game_started: AtomicI64,
}

impl SnowStormGame {
    /// Mirrors `MAX_QUICK_THROW_DISTANCE`.
    pub const MAX_QUICK_THROW_DISTANCE: i32 = 22;

    /// Mirrors the `SnowStormGame(int, int, String, int, Player, int,
    /// boolean)` constructor (the `Game` base constructor is mirrored
    /// by the `GameBase` state, see the module note).
    pub fn new(
        id: i32,
        map_id: i32,
        name: String,
        team_amount: i32,
        game_creator: Arc<Mutex<Player>>,
        game_length_choice: i32,
        _private_game: bool,
    ) -> Arc<Self> {
        let mut teams: HashMap<i32, Arc<Mutex<GameTeam>>> = HashMap::new();

        for i in 0..team_amount {
            teams.insert(i, Arc::new(Mutex::new(GameTeam::new(i))));
        }

        Arc::new(Self {
            id: AtomicI32::new(id),
            map_id,
            game_type: GameType::Snowstorm,
            name,
            team_amount,
            game_creator,
            teams,
            objects: Mutex::new(Vec::new()),
            game_state: Mutex::new(GameState::Waiting),
            room: Mutex::new(Room::default()),
            room_model: Mutex::new(None),
            object_id: AtomicI32::new(0),
            total_seconds_left: AtomicI32::new(0),
            game_length_choice,
            executing_events: Mutex::new(Vec::new()),
            game_started: AtomicI64::new(0),
        })
    }

    /// Mirrors `hasEnoughPlayers()`.
    pub fn has_enough_players(&self) -> bool {
        if self.team_amount == 1 {
            return !self.get_active_players().is_empty();
        }

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

    /// Mirrors `initialise()`.
    pub fn initialise(&self) {
        let height_map = SnowStormMapsManager::get_instance().get_height_map(self.map_id);
        let model = RoomModel::new(
            "snowwar_arena_0",
            "snowwar_arena_0",
            0,
            0,
            0.0,
            0,
            &height_map,
            None,
        );

        let mut seconds = 0;

        if self.game_length_choice == 1 {
            seconds = 2 * 60;
        }

        if self.game_length_choice == 2 {
            seconds = 3 * 60;
        }

        if self.game_length_choice == 3 {
            seconds = 5 * 60;
        }

        if GameManager::get_instance().get_lifetime_seconds(self.game_type) > 0 {
            seconds = GameManager::get_instance().get_lifetime_seconds(self.game_type);
        }

        // Mirrors `super.initialise(seconds, "SnowStorm Arena", model)`.
        *self.game_state.lock() = GameState::Started;
        // Port note: the Java `preparingGameSecondsLeft`
        // (`GameManager.getPreparingSeconds`) field is not ported.
        self.total_seconds_left.store(seconds, Ordering::SeqCst);
        *self.room_model.lock() = Some(model.clone());

        {
            let mut room = self.room.lock();
            *room = Room::default();
            room.get_data_mut().fill_brief(0, "SnowStorm Arena", "");
            room.set_room_model(model.clone());
            // Port note: the Java `room.getData().setGame(this)` needs an
            // `Arc<SnowStormGame>`, which the `&self` here cannot mint.
            room.set_game_arena(true);
            room.get_data_mut().set_game_lobby(Some(self.game_type.get_lobby_model()));
        }

        self.objects.lock().clear();
        self.executing_events.lock().clear();
        {
            let room = self.room.lock();
            room.get_mapping().lock().regenerate_collision_map(&room);
        }
        self.build_map();

        self.game_started.store(DateUtil::get_current_time_seconds() as i64, Ordering::SeqCst);

        if let Some(map) = self.get_map() {
            for snowball_item in map.get_items() {
                if snowball_item.is_snowball_machine() {
                    self.objects.lock().push(Arc::new(SnowStormMachineObject::new(
                        self.create_object_id(),
                        snowball_item.get_x(),
                        snowball_item.get_y(),
                        0,
                    )));
                }
            }
        }
        //this.getTotalSecondsLeft().set(seconds) // Override with game length choice
    }

    /// Mirrors `gamePrepare()`.
    pub fn game_prepare(&self) {
        // `super.gamePrepare()`
        for game_player in self.get_active_players() {
            let mut game_player = game_player.lock();
            game_player.set_xp(0);
            game_player.set_score(0);
        }

        let ticket_charge = GameConfiguration::get_instance().get_integer("snowstorm.ticket.charge");

        if ticket_charge > 0 {
            for game_player in self.get_active_players() {
                let game_player = game_player.lock();
                let player = game_player.get_player().lock();

                // BattleBall costs 2 tickets.
                CurrencyDao::decrease_tickets(player.get_details(), 2);
            }
        }
    }

    /// Mirrors `finishGame()`.
    pub fn finish_game(&self) {
        for p in self.get_active_players() {
            let p = p.lock();
            let score = p.get_snow_storm_attributes().get_score();
            p.set_score(score);
        }

        for team in self.teams.values() {
            team.lock().calculate_score();
        }

        // Port note: the Java base `finishGame()` (the game history
        // save and the walking-allowed reset) is not ported.
        *self.game_state.lock() = GameState::Ended;
    }

    /// Mirrors `assignSpawnPoints()`.
    pub fn assign_spawn_points(&self) {
        let room = self.room.lock();
        room.get_mapping().lock().regenerate_collision_map(&room);

        for team in self.teams.values() {
            for p in team.lock().get_players() {
                p.lock().set_assigned_spawn(false);
            }
        }

        for team in self.teams.values() {
            for p in team.lock().get_players() {
                let p_arc = p;
                self.generate_spawn(&p_arc);

                let mut p = p_arc.lock();
                p.get_snow_storm_attributes_mut()
                    .set_rotation(rand::thread_rng().gen_range(0..7));
                p.get_snow_storm_attributes_mut()
                    .set_activity_state(Arc::clone(&p_arc), SnowStormActivityState::ActivityStateNormal, None);
                //p.getPlayer().getBadgeManager().tryAddBadge("SS_BETA", null);
                p.get_snow_storm_attributes_mut().set_walking(false);
                let spawn_position = p.get_spawn_position().copy();
                p.get_snow_storm_attributes_mut()
                    .set_current_position(Some(spawn_position));
                p.get_snow_storm_attributes_mut().set_walk_goal(None);
                p.get_snow_storm_attributes_mut().set_next_goal(None);
                p.get_snow_storm_attributes_mut().set_immunity_expiry(0);
                p.get_snow_storm_attributes_mut().set_score(0);
                p.get_snow_storm_attributes_mut().set_snowballs(5);
                p.get_snow_storm_attributes_mut().set_health(4);
                p.get_snow_storm_attributes_mut().set_goal_world_coordinates(None);

                p.set_object_id(self.create_object_id());
                p.set_score(0);

                p.set_game_object(Some(std::sync::Arc::new(SnowStormAvatarObject::new(
                    Arc::clone(&p_arc),
                ))));



                // Port note: the Java `this.getObjects().add(p.getGameObject())`
                // needs the same `GameObject` instance shared between the
                // `GamePlayer` (`Box`) and the game objects list (`Arc`);
                // the shared-reference add is skipped.
            }
        }
    }

    /// Mirrors `generateSpawn(GamePlayer)` (the Java recursion is a
    /// loop; the `try/catch` fallback is the loop exit).
    fn generate_spawn(&self, p: &Arc<Mutex<GamePlayer>>) {
        let map = match self.get_map() {
            Some(map) => map,
            // Java `NullPointerException` equivalent: the fallback below.
            None => {
                let mut position = p.lock().get_spawn_position().copy();
                position.set_x(15);
                position.set_y(18);
                p.lock().set_spawn_position(position);
                p.lock().set_assigned_spawn(true);
                return;
            }
        };

        if map.get_spawn_clusters().is_empty() {
            let mut position = p.lock().get_spawn_position().copy();
            position.set_x(15);
            position.set_y(18);
            p.lock().set_spawn_position(position);
            p.lock().set_assigned_spawn(true);
            return;
        }

        loop {
            let spawn = &map.get_spawn_clusters()[
                rand::thread_rng().gen_range(0..map.get_spawn_clusters().len())
            ];

            let mut potential_positions = spawn.get_position().get_circle(spawn.get_radius());
            potential_positions.shuffle(&mut rand::thread_rng());

            // Java `ThreadLocalRandom.nextInt(0, size - 1)` panics on a
            // single candidate; the `saturating_sub` guards it.
            let candidate = &potential_positions[
                rand::thread_rng().gen_range(0..potential_positions.len().saturating_sub(1).max(1))
            ];

            let mut rejected = false;

            for game_player in self.get_active_players() {
                let game_player = game_player.lock();

                if !game_player.is_assigned_spawn() {
                    continue;
                }

                let distance = game_player
                    .get_spawn_position()
                    .get_distance_squared(&candidate);

                if distance < spawn.get_min_distance() {
                    rejected = true;
                    break;
                }
            }

            if rejected {
                continue;
            }

            if !map
                .get_tile(&candidate)
                .map(|tile| tile.is_walkable())
                .unwrap_or(false)
            {
                continue;
            }

            let mut position = p.lock().get_spawn_position().copy();
            position.set_x(candidate.get_x());
            position.set_y(candidate.get_y());
            p.lock().set_spawn_position(position);
            p.lock().set_assigned_spawn(true);
            return;
        }
    }

    /// Mirrors `getGameLength()`.
    pub fn get_game_length(&self) -> i32 {
        if *self.game_state.lock() == GameState::Waiting || *self.game_state.lock() == GameState::Ended {
            if self.game_length_choice == 1 {
                return 2 * 60;
            }

            if self.game_length_choice == 2 {
                return 3 * 60;
            }

            if self.game_length_choice == 3 {
                return 5 * 60;
            }
        }

        self.total_seconds_left.load(Ordering::SeqCst)
    }

    /// Mirrors `convertToGameCoordinate(int)`.
    pub fn convert_to_game_coordinate(num: i32) -> i32 {
        let p_accuracy_factor = 100;
        let p_tile_size = 32;
        let t_multiplier = p_tile_size * p_accuracy_factor;

        num / t_multiplier
    }

    /// Mirrors `convertToWorldCoordinate(int)`.
    pub fn convert_to_world_coordinate(num: i32) -> i32 {
        let p_accuracy_factor = 100;
        let p_tile_size = 32;
        let t_multiplier = p_tile_size * p_accuracy_factor;

        num * t_multiplier
    }

    /// Mirrors `isOppositionPlayer(GamePlayer, GamePlayer)`.
    pub fn is_opposition_player(
        &self,
        game_player: &Arc<Mutex<GamePlayer>>,
        player: &Arc<Mutex<GamePlayer>>,
    ) -> bool {
        let game_player_id = game_player
            .lock()
            .get_player()
            .lock()
            .get_details()
            .get_id();
        let player_id = player.lock().get_player().lock().get_details().get_id();

        if game_player_id == player_id {
            return false;
        }

        if self.team_amount == 1 {
            return true;
        }

        game_player.lock().get_team_id() != player.lock().get_team_id()
    }

    /// Mirrors `handleSnowballLanding(SnowballObject, boolean)`.
    pub fn handle_snowball_landing(
        &self,
        snowball: &Arc<Mutex<SnowballObject>>,
        delete_after_hit: bool,
    ) {
        let last_tile_position = Position::new_xy(
            snowball.lock().get_target_x(),
            snowball.lock().get_target_y(),
        );

        let Some(map) = self.get_map() else {
            return;
        };

        let Some(_tile) = map.get_tile(&last_tile_position) else {
            return;
        };

        let thrower = snowball.lock().get_thrower().clone();

        let mut hit_player: Option<Arc<Mutex<GamePlayer>>> = None;

        for p in self.get_active_players() {
            if !self.is_opposition_player(&p, &thrower) {
                continue;
            }

            let (position_match, damageable, health) = {
                let guard = p.lock();
                let attrs = guard.get_snow_storm_attributes();
                let position_match = attrs
                    .get_current_position()
                    .map(|c| c == &last_tile_position)
                    .unwrap_or(false)
                    || attrs
                        .get_next_goal()
                        .map(|g| g == &last_tile_position)
                    .unwrap_or(false);
                (
                    position_match,
                    attrs.is_damageable(),
                    attrs.get_health(),
                )
            };

            if position_match && damageable && health > 0 {
                hit_player = Some(p);
                break;
            }
        }

        if let Some(player) = hit_player {
            let thrower = snowball.lock().get_thrower().clone();

            player
                .lock()
                .get_snow_storm_attributes()
                .increment_score();

            if let Some(update_task) = self.get_update_task() {
                update_task.send_queue(
                    0,
                    1,
                    Arc::new(SnowStormHitEvent::new(
                        thrower.lock().get_object_id(),
                        player.lock().get_object_id(),
                        Rotation::calculate_walk_direction_coords(
                            snowball.lock().get_from_x(),
                            snowball.lock().get_from_y(),
                            last_tile_position.get_x(),
                            last_tile_position.get_y(),
                        ),
                    )),
                );

                if delete_after_hit {
                    update_task.send_queue(
                        0,
                        1,
                        Arc::new(SnowStormDeleteObjectEvent::new(
                            snowball.lock().get_object_id(),
                        )),
                    );
                }
            }

            // System.out.println("Player ... hits ...")
            let health_decremented = player
                .lock()
                .get_snow_storm_attributes()
                .decrement_health_get();

            if health_decremented == 0 {
                Self::stun_player_handler(
                    self,
                    &thrower,
                    &player,
                    last_tile_position,
                    snowball,
                );
            }
        } else {
            if snowball.lock().is_blocked() {
                if let Some(update_task) = self.get_update_task() {
                    update_task.send_queue(
                        0,
                        1,
                        Arc::new(SnowStormDeleteObjectEvent::new(
                            snowball.lock().get_object_id(),
                        )),
                    );
                }
            }
        }
    }

    /// Mirrors `stunPlayerHandler(SnowStormGame, GamePlayer, GamePlayer,
    /// Position, SnowballObject)`.
    pub fn stun_player_handler(
        game: &SnowStormGame,
        thrower: &Arc<Mutex<GamePlayer>>,
        player: &Arc<Mutex<GamePlayer>>,
        landed_position: Position,
        snowball: &Arc<Mutex<SnowballObject>>,
    ) {
        let stun_direction =
            Rotation::calculate_walk_direction_coords(
                snowball.lock().get_from_x(),
                snowball.lock().get_from_y(),
                landed_position.get_x(),
                landed_position.get_y(),
            );

        if let Some(update_task) = game.get_update_task() {
            update_task.send_queue(
                0,
                1,
                Arc::new(SnowStormStunEvent::new(
                    player.lock().get_object_id(),
                    thrower.lock().get_object_id(),
                    45 * stun_direction,
                )),
            );
        }

        // System.out.println("Player ... hits ...")
        thrower
            .lock()
            .get_snow_storm_attributes()
            .add_score(5);
        let player_arc = Arc::clone(player);
        let mut player = player.lock();
        player.get_snow_storm_attributes().set_snowballs(0);
        player.get_snow_storm_attributes().set_health(4);
        player.get_snow_storm_attributes_mut().set_activity_state(
            Arc::clone(&player_arc),
            SnowStormActivityState::ActivityStateStunned,
            Some(Box::new(
                move || {
                    player_arc
                        .lock()
                        .get_snow_storm_attributes_mut()
                        .set_activity_state(
                            Arc::clone(&player_arc),
                            SnowStormActivityState::ActivityStateInvincibleAfterStun,
                            None,
                        );
                },
            )),
        );
    }

    /// Mirrors `getUpdateTask()` (the Java `(SnowStormGameTask)` cast is
    /// the `Arc` downcast).
    pub fn get_update_task(&self) -> Option<Arc<SnowStormGameTask>> {
        self.room
            .lock()
            .get_task_manager()
            .get_task("UpdateTask")?
            .downcast::<SnowStormGameTask>()
            .ok()
    }

    /// Mirrors `gameTick()`.
    pub fn game_tick(&self) {}

    /// Mirrors `canTimerContinue()`.
    pub fn can_timer_continue(&self) -> bool {
        true
    }

    /// Mirrors `getTileMap()`.
    pub fn get_tile_map(&self) -> Vec<Vec<Option<Arc<Mutex<()>>>>> {
        Vec::new()
    }

    /// Mirrors `buildMap()`.
    pub fn build_map(&self) {}

    /// Mirrors `getMap()`.
    pub fn get_map(&self) -> Option<SnowStormMap> {
        SnowStormMapsManager::get_instance().get_map(self.map_id)
    }

    /// Mirrors `getGameLengthChoice()`.
    pub fn get_game_length_choice(&self) -> i32 {
        self.game_length_choice
    }

    /// Mirrors `createObjectId()`.
    pub fn create_object_id(&self) -> i32 {
        self.object_id.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Mirrors `getTotalSecondsLeft().get()`.
    pub fn get_total_seconds_left(&self) -> i32 {
        self.total_seconds_left.load(Ordering::SeqCst)
    }

    /// Mirrors the `executingEvents` field access (the Java field has no
    // accessor; the live handle is returned).
    pub fn get_executing_events(&self) -> &Mutex<Vec<Arc<dyn GameObject>>> {
        &self.executing_events
    }

    /// Mirrors the `gameStarted` field (the Java field has no accessor).
    pub fn get_game_started(&self) -> i64 {
        self.game_started.load(Ordering::SeqCst)
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
        *self.game_state.lock()
    }

    /// Mirrors `getGameCreator()`.
    pub fn get_game_creator(&self) -> &Arc<Mutex<Player>> {
        &self.game_creator
    }

    /// Mirrors `getTeams()`.
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

    /// Mirrors `getObjects()`.
    pub fn get_objects(&self) -> &Mutex<Vec<Arc<dyn GameObject>>> {
        &self.objects
    }

    /// Mirrors `getRoom()`.
    pub fn get_room(&self) -> Room {
        self.room.lock().clone()
    }

    /// Mirrors `getRoomModel()`.
    pub fn get_room_model(&self) -> Option<RoomModel> {
        self.room_model.lock().clone()
    }
}
