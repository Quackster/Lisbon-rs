//! Mirrors `net.h4bbo.lisbon.game.games.Game`.
//!
//! The Java `abstract class` is an `enum` here (Rust has no
//! inheritance); each variant wraps the concrete game (`Arc<Mutex>`
//! handles, mirroring the `GAMESTATUS` / `SnowStormGameTask` handle
//! convention) together with the shared base state the Java `Game`
//! fields provide (`GameBase`). The Java abstract methods (`hasEnoughPlayers`,
//! `canTimerContinue`, `assignSpawnPoints`, `getTileMap`, `buildMap`,
//! `gameTick`) live on the concrete types.
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::battle_ball_tile::BattleBallTile;
use crate::game::games::enums::game_state::GameState;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game_manager::GameManager;
use crate::game::games::game_object::GameObject;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::player::game_team::GameTeam;
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::game::games::snowstorm::tasks::snow_storm_game_task::SnowStormGameTask;
use crate::game::room::models::room_model::RoomModel;
use crate::game::room::room::Room;
use crate::game::room::room_manager::RoomManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::games::gamedeleted::GAMEDELETED;
use crate::messages::outgoing::games::gameinstance::GAMEINSTANCE;
use crate::messages::outgoing::games::gamelocation::GAMELOCATION;
use crate::messages::outgoing::games::gamereset::GAMERESET;
use crate::messages::outgoing::games::fullgamestatus::FULLGAMESTATUS;
use crate::messages::types::MessageComposer;
use crate::util::config::game_configuration::GameConfiguration;

/// Mirrors the shared `Game` base fields (the concrete Rust game types
/// hold the remaining Java `Game` state directly).
struct GameBase {
    game_creator_id: i32,
    spectators: Mutex<Vec<Arc<Mutex<GamePlayer>>>>,
    observers: Mutex<Vec<Arc<Mutex<Player>>>>,
    preparing_game_seconds_left: AtomicI32,
}

impl Clone for GameBase {
    fn clone(&self) -> Self {
        Self {
            game_creator_id: self.game_creator_id,
            spectators: Mutex::new(self.spectators.lock().clone()),
            observers: Mutex::new(self.observers.lock().clone()),
            preparing_game_seconds_left: AtomicI32::new(
                self.preparing_game_seconds_left.load(Ordering::SeqCst),
            ),
        }
    }
}

impl fmt::Debug for GameBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameBase").finish()
    }
}

impl GameBase {
    fn new(game_type: GameType, game_creator: &Arc<Mutex<Player>>) -> Self {
        Self {
            game_creator_id: game_creator.lock().get_details().get_id(),
            spectators: Mutex::new(Vec::new()),
            observers: Mutex::new(Vec::new()),
            // Mirrors `initialise()` (the base `initialise` is not
            // reachable through the shared handle; the config value it
            // would set is applied here).
            preparing_game_seconds_left: AtomicI32::new(
                GameManager::get_instance().get_preparing_seconds(game_type),
            ),
        }
    }
}

/// Mirrors the abstract `Game` class.
pub enum Game {
    SnowStorm {
        base: GameBase,
        game: Arc<SnowStormGame>,
    },
    BattleBall {
        base: GameBase,
        game: Arc<BattleBallGame>,
    },
}

impl Clone for Game {
    fn clone(&self) -> Self {
        match self {
            Self::SnowStorm { base, game } => Self::SnowStorm {
                base: base.clone(),
                game: Arc::clone(game),
            },
            Self::BattleBall { base, game } => Self::BattleBall {
                base: base.clone(),
                game: Arc::clone(game),
            },
        }
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SnowStorm { game, .. } => f
                .debug_struct("Game::SnowStorm")
                .field("id", &game.get_id())
                .finish(),
            Self::BattleBall { game, .. } => f
                .debug_struct("Game::BattleBall")
                .field("id", &game.get_id())
                .finish(),
        }
    }
}

impl Game {
    /// Wrap a `SnowStormGame` (mirrors the Java subclass reference).
    pub fn new_snow_storm(game: Arc<SnowStormGame>) -> Self {
        let (game_type, game_creator) = (
            game.get_game_type(),
            Arc::clone(game.get_game_creator()),
        );
        let base = GameBase::new(game_type, &game_creator);
        Self::SnowStorm { base, game }
    }

    /// Wrap a `BattleBallGame` (mirrors the Java subclass reference).
    pub fn new_battle_ball(game: Arc<BattleBallGame>) -> Self {
        let (game_type, game_creator) = (
            game.get_game_type(),
            Arc::clone(game.get_game_creator()),
        );
        let base = GameBase::new(game_type, &game_creator);
        Self::BattleBall { base, game }
    }

    /// Mirrors the Java `(SnowStormGame)` cast.
    pub fn as_snow_storm(&self) -> Option<Arc<SnowStormGame>> {
        match self {
            Self::SnowStorm { game, .. } => Some(Arc::clone(game)),
            _ => None,
        }
    }

    /// Mirrors the Java `(BattleBallGame)` cast.
    pub fn as_battle_ball(&self) -> Option<Arc<BattleBallGame>> {
        match self {
            Self::BattleBall { game, .. } => Some(Arc::clone(game)),
            _ => None,
        }
    }

    /// Mirrors `getUpdateTask()` (the `SnowStormGame` method; `None`
    /// for the other variants).
    pub fn get_update_task(&self) -> Option<Arc<SnowStormGameTask>> {
        self.as_snow_storm()?.get_update_task()
    }

    /// Mirrors `reassignGameId()`.
    pub fn reassign_game_id(&self) {
        match self {
            Self::SnowStorm { game, .. } => game.reassign_game_id(),
            Self::BattleBall { game, .. } => game.reassign_game_id(),
        }
    }

    /// Mirrors `isGameStarted()` (the Java `gameStarted` flag is the
    // `GameState.STARTED` equivalent).
    pub fn is_game_started(&self) -> bool {
        self.game_state() == GameState::Started
    }

    /// Mirrors `isGameFinished()`.
    pub fn is_game_finished(&self) -> bool {
        self.game_state() == GameState::Ended
    }

    /// Mirrors `getTotalSecondsLeft().get()`.
    pub fn get_total_seconds_left(&self) -> i32 {
        match self {
            Self::SnowStorm { game, .. } => game.get_total_seconds_left(),
            Self::BattleBall { game, .. } => game.get_total_seconds_left(),
        }
    }

    /// Mirrors `getGameCreatorId()`.
    pub fn get_game_creator_id(&self) -> i32 {
        self.base().game_creator_id
    }

    /// Mirrors `getPreparingGameSecondsLeft().get()`.
    pub fn get_preparing_game_seconds_left(&self) -> i32 {
        self.base().preparing_game_seconds_left.load(Ordering::SeqCst)
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        match self {
            Self::SnowStorm { game, .. } => game.get_id(),
            Self::BattleBall { game, .. } => game.get_id(),
        }
    }

    /// Mirrors `getMapId()`.
    pub fn get_map_id(&self) -> i32 {
        match self {
            Self::SnowStorm { game, .. } => game.get_map_id(),
            Self::BattleBall { game, .. } => game.get_map_id(),
        }
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> String {
        match self {
            Self::SnowStorm { game, .. } => game.get_name().to_string(),
            Self::BattleBall { game, .. } => game.get_name().to_string(),
        }
    }

    /// Mirrors `getGameCreator()` (the Java name string).
    pub fn get_game_creator(&self) -> String {
        match self {
            Self::SnowStorm { game, .. } => game
                .get_game_creator()
                .lock()
                .get_details()
                .get_name()
                .to_string(),
            Self::BattleBall { game, .. } => game
                .get_game_creator()
                .lock()
                .get_details()
                .get_name()
                .to_string(),
        }
    }

    /// Mirrors `getGameType()`.
    pub fn get_game_type(&self) -> GameType {
        match self {
            Self::SnowStorm { game, .. } => game.get_game_type(),
            Self::BattleBall { game, .. } => game.get_game_type(),
        }
    }

    /// Mirrors `getGameState()`.
    pub fn get_game_state(&self) -> GameState {
        self.game_state()
    }

    /// Mirrors `getTeamAmount()`.
    pub fn get_team_amount(&self) -> i32 {
        match self {
            Self::SnowStorm { game, .. } => game.get_team_amount(),
            Self::BattleBall { game, .. } => game.get_team_amount(),
        }
    }

    /// Mirrors `getTeams()` (the shared team handles).
    pub fn get_teams(&self) -> Vec<Arc<Mutex<GameTeam>>> {
        match self {
            Self::SnowStorm { game, .. } => game.get_teams(),
            Self::BattleBall { game, .. } => game.get_teams(),
        }
    }

    /// Mirrors `getTeams().get(int)`.
    pub fn get_team(&self, team_id: i32) -> Option<Arc<Mutex<GameTeam>>> {
        match self {
            Self::SnowStorm { game, .. } => game.get_team(team_id),
            Self::BattleBall { game, .. } => game.get_team(team_id),
        }
    }

    /// Mirrors `getActivePlayers()`.
    pub fn get_active_players(&self) -> Vec<Arc<Mutex<GamePlayer>>> {
        match self {
            Self::SnowStorm { game, .. } => game.get_active_players(),
            Self::BattleBall { game, .. } => game.get_active_players(),
        }
    }

    /// Mirrors `getTotalPlayers()` (the Java body is the
    /// `getActivePlayers` equivalent).
    pub fn get_total_players(&self) -> Vec<Arc<Mutex<GamePlayer>>> {
        self.get_active_players()
    }

    /// Mirrors `getSpectators()`.
    pub fn get_spectators(&self) -> Vec<Arc<Mutex<GamePlayer>>> {
        self.base().spectators.lock().clone()
    }

    /// Mirrors `getObservers()`.
    pub fn get_observers(&self) -> Vec<Arc<Mutex<Player>>> {
        self.base().observers.lock().clone()
    }

    /// Mirrors `getSpectators().add(GamePlayer)`.
    pub fn add_spectator(&self, spectator: Arc<Mutex<GamePlayer>>) {
        self.base().spectators.lock().push(spectator)
    }

    /// Mirrors `getSpectators().remove(GamePlayer)`.
    pub fn remove_spectator(&self, spectator: &Arc<Mutex<GamePlayer>>) {
        self.base()
            .spectators
            .lock()
            .retain(|s| !Arc::ptr_eq(s, spectator))
    }

    /// Mirrors `getObservers().add(Player)`.
    pub fn add_observer(&self, observer: Arc<Mutex<Player>>) {
        self.base().observers.lock().push(observer)
    }

    /// Mirrors `getObservers().remove(Player)`.
    pub fn remove_observer(&self, observer: &Arc<Mutex<Player>>) {
        self.base().observers.lock().retain(|o| !Arc::ptr_eq(o, observer))
    }

    /// Mirrors `getRoom()`.
    pub fn get_room(&self) -> Room {
        match self {
            Self::SnowStorm { game, .. } => game.get_room().clone(),
            Self::BattleBall { game, .. } => game.get_room().clone(),
        }
    }

    /// Mirrors `getRoomModel()`.
    pub fn get_room_model(&self) -> Option<RoomModel> {
        match self {
            Self::SnowStorm { game, .. } => game.get_room_model(),
            Self::BattleBall { game, .. } => game.get_room_model().cloned(),
        }
    }

    /// Mirrors `getObjects()`.
    pub fn get_objects(&self) -> Vec<Arc<dyn GameObject>> {
        match self {
            Self::SnowStorm { game, .. } => game.get_objects().lock().clone(),
            Self::BattleBall { game, .. } => game.get_objects().lock().clone(),
        }
    }

    /// Mirrors `getTile(int, int)` (the Java `GameTile` is cast to
    /// `BattleBallTile` by the callers; `None` for the `SnowStormGame`
    /// variant, which has no tile map).
    pub fn get_tile(&self, x: i32, y: i32) -> Option<Arc<Mutex<BattleBallTile>>> {
        match self {
            Self::BattleBall { game, .. } => game.get_tile(x, y),
            _ => None,
        }
    }

    /// Mirrors `canIncreasePoints()`.
    pub fn can_increase_points(&self) -> bool {
        GameConfiguration::get_instance().get_bool(
            &format!("{}.increase.points", self.get_game_type().get_name()),
        )
    }

    /// Mirrors `getMaxPlayers()`.
    pub fn get_max_players(&self) -> i32 {
        let max_per_team = match self.get_team_amount() {
            1 => 10,
            2 => 5,
            3 => 3,
            _ => 2,
        };

        max_per_team * self.get_team_amount()
    }

    /// Mirrors `canSwitchTeam(int)`.
    pub fn can_switch_team(&self, team_id: i32) -> bool {
        self.get_team(team_id)
            .map(|team| team.lock().get_active_players().len() as i32 <= self.get_max_players() / self.get_team_amount())
            .unwrap_or(false)
    }

    /// Mirrors `getTicketCost()`.
    pub fn get_ticket_cost(&self) -> i32 {
        self.get_game_type().get_ticket_cost()
    }

    /// Mirrors `canGameStart()`.
    pub fn can_game_start(&self) -> bool {
        if self.get_game_type() == GameType::Snowstorm && self.get_team_amount() == 1 {
            return self.get_active_players().len() > 1;
        }

        let mut active_team_count = 0;

        for team in self.get_teams() {
            if !team.lock().get_active_players().is_empty() {
                active_team_count += 1;
            }
        }

        active_team_count
            >= GameConfiguration::get_instance().get_integer(
                &format!(
                    "{}.start.minimum.active.teams",
                    self.get_game_type().get_name()
                ),
            )
    }

    /// Mirrors `send(MessageComposer)`.
    pub fn send(&self, composer: &dyn MessageComposer) {
        for game_player in self.get_active_players() {
            game_player.lock().get_player().lock().send(composer);
        }

        for player in self.get_spectators() {
            player.lock().get_player().lock().send(composer);
        }
    }

    /// Mirrors `sendObservers(MessageComposer)`.
    pub fn send_observers(&self, composer: &dyn MessageComposer) {
        for player in self.get_observers() {
            player.lock().send(composer);
        }
    }

    /// Mirrors `movePlayer(GamePlayer, int, int)`.
    pub fn move_player(
        &self,
        game_player: &Arc<Mutex<GamePlayer>>,
        from_team_id: i32,
        to_team_id: i32,
    ) {
        if from_team_id != -1 {
            if matches!(
                self.game_state(),
                GameState::Waiting | GameState::Ended
            ) {
                if let Some(team) = self.get_team(from_team_id) {
                    team.lock().remove_player(game_player);
                }
            }

            game_player.lock().set_in_game(false); // Leaving team so they're not in game
        }

        if to_team_id != -1 {
            if let Some(team) = self.get_team(to_team_id) {
                let mut team = team.lock();
                if !team
                    .get_players()
                    .iter()
                    .any(|p| Arc::ptr_eq(p, game_player))
                {
                    team.add_player(Arc::clone(game_player));
                }
            }

            game_player.lock().set_team_id(to_team_id);
            game_player.lock().set_in_game(true); // Entering team so they're in game
        } else {
            if matches!(
                self.game_state(),
                GameState::Waiting | GameState::Ended
            ) {
                let team_id = game_player.lock().get_team_id();
                if let Some(team) = self.get_team(team_id) {
                    team.lock().remove_player(game_player);
                }
            } else {
                game_player.lock().set_in_game(false);
            }

            // `setGamePlayer(null)`
            game_player
                .lock()
                .get_player()
                .lock()
                .get_room_user()
                .map(|room_user| room_user.clear_game_player());
        }

        let instance = GAMEINSTANCE::new_game(self.clone());
        self.send(&instance);
        let instance = GAMEINSTANCE::new_game(self.clone());
        self.send_observers(&instance);
    }

    /// Mirrors `leaveGame(GamePlayer)`.
    pub fn leave_game(&self, game_player: &Arc<Mutex<GamePlayer>>) {
        let is_spectator = self
            .get_spectators()
            .iter()
            .any(|s| Arc::ptr_eq(s, game_player));

        self.remove_spectator(game_player);

        if let Some(game_object) = game_player.lock().get_game_object() {
            let game_object_id = game_object.get_id();
            match self {
                Self::SnowStorm { game, .. } => game
                    .get_objects()
                    .lock()
                    .retain(|object| object.get_id() != game_object_id),
                Self::BattleBall { game, .. } => game
                    .get_objects()
                    .lock()
                    .retain(|object| object.get_id() != game_object_id),
            };
        }

        let player = game_player.lock().get_player().clone();
        player.lock().get_room_user().map(|room_user| {
            room_user.clear_game_player();
        });
        player.lock().send(&GAMEDELETED::new(self.get_id()));

        if !is_spectator {
            let from_team_id = game_player.lock().get_team_id();
            self.move_player(game_player, from_team_id, -1);
        } else {
            let instance = GAMEINSTANCE::new_game(self.clone());
            self.send(&instance);
            let instance = GAMEINSTANCE::new_game(self.clone());
            self.send_observers(&instance);
        }

        let creator_left = matches!(self.game_state(), GameState::Waiting)
            && self.get_game_creator_id()
                == game_player
                    .lock()
                    .get_player()
                    .lock()
                    .get_details()
                    .get_id();

        if creator_left || self.get_active_players().is_empty() {
            GameManager::get_instance().remove_game(self.get_id());

            for p in self.get_total_players() {
                p.lock().get_player().lock().get_room_user().map(|room_user| {
                    room_user.clear_game_player();
                });
            }

            for p in self.get_spectators() {
                p.lock().get_player().lock().get_room_user().map(|room_user| {
                    room_user.clear_game_player();
                });
            }

            for p in self.get_observers() {
                p.lock().get_room_user().map(|room_user| {
                    room_user.set_observing_game_id(0);
                });
            }

            let deleted = GAMEDELETED::new(self.get_id());
            self.send(&deleted);
            let deleted = GAMEDELETED::new(self.get_id());
            self.send_observers(&deleted);
            self.kill_spectators();
        }

        player.lock().get_room_user().map(|room_user| {
            room_user.clear_game_player();
        });
        game_player.lock().set_game_id(-1);
    }

    /// Mirrors `restartGame(List<GamePlayer>)`.
    pub fn restart_game(&self, players: Vec<Arc<Mutex<GamePlayer>>>) {
        // The Java `FutureRunnable` cancels (`GameScheduler` stub) are
        // skipped; the port holds no live runnables to cancel.

        for team in self.get_teams() {
            team.lock().clear_players();
        }

        for game_player in &players {
            let team_id = game_player.lock().get_team_id();
            self.move_player(game_player, -1, team_id);
            // Don't allow them to walk, for next game
            game_player
                .lock()
                .get_player()
                .lock()
                .get_room_user()
                .map(|room_user| room_user.set_walking_allowed(false));
        }

        self.reassign_game_id();

        match self {
            Self::SnowStorm { game, .. } => {
                game.initialise();
                game.assign_spawn_points();
            }
            Self::BattleBall { game, .. } => game.assign_spawn_points(),
        }

        for game_player in self.get_active_players() {
            let object_id = game_player.lock().get_object_id();
            let spawn_position = game_player.lock().get_spawn_position().copy();
            game_player.lock().set_game_id(self.get_id());
            game_player
                .lock()
                .get_player()
                .lock()
                .get_room_user()
                .map(|room_user| {
                    room_user.set_instance_id(object_id);
                    room_user.set_position(spawn_position);
                });
        }

        let reset = GAMERESET::new(
            GameManager::get_instance().get_preparing_seconds(self.get_game_type()),
            players,
            self.clone(),
        );
        self.send(&reset);
        let status = FULLGAMESTATUS::new(self.clone());
        self.send(&status);
        let deleted = GAMEDELETED::new(self.get_id());
        self.send_observers(&deleted);

        // The Java preparing-countdown `FutureRunnable` is skipped (the
        // `GameScheduler` stub has no fixed-rate scheduling).
        match self {
            Self::SnowStorm { game, .. } => game.game_prepare(),
            Self::BattleBall { game, .. } => game.game_prepare(),
        }
    }

    /// Mirrors `startGame()`.
    pub fn start_game(&self) {
        match self {
            Self::SnowStorm { game, .. } => {
                game.initialise();
                game.assign_spawn_points();
            }
            Self::BattleBall { game, .. } => game.assign_spawn_points(),
        }

        for game_player in self.get_active_players() {
            game_player.lock().set_entering_game(true);
        }

        self.send(&GAMELOCATION);
        self.send_spectators_to_arena();

        // The Java preparing-countdown `FutureRunnable` is skipped (the
        // `GameScheduler` stub has no fixed-rate scheduling).
        let instance = GAMEINSTANCE::new_game(self.clone());
        self.send_observers(&instance);

        match self {
            Self::SnowStorm { game, .. } => game.game_prepare(),
            Self::BattleBall { game, .. } => {
                game.game_prepare();
                // The Java `BattleBallGame.startGame()` override part.
                game.start_game();
            }
        }
    }

    /// Mirrors `killSpectators()`.
    pub fn kill_spectators(&self) {
        for spectator in self.get_spectators() {
            self.send_to_lobby(&spectator);
        }

        self.base().spectators.lock().clear();
    }

    /// Mirrors `sendToLobby(GamePlayer)`.
    pub fn send_to_lobby(&self, game_player: &Arc<Mutex<GamePlayer>>) {
        let player = game_player.lock().get_player().clone();
        let room = player
            .lock()
            .get_room_user()
            .and_then(|room_user| room_user.get_room());

        let same_room = room
            .as_ref()
            .map(|room| room.get_id() == self.get_room().get_id())
            .unwrap_or(false);

        if !same_room {
            return; // Don't force people to go to a room they didn't request
        }

        if let Some(room) = room {
            let entity: &(dyn Entity + Send) = &*player.lock();
            room.get_entity_manager().leave_room(&room, entity, true);
        }

        if let Some(lobby) = self.get_lobby() {
            let lobby = lobby.lock();
            lobby.forward(&player.lock(), false);
        }
    }

    /// Mirrors `getLobby()`.
    pub fn get_lobby(&self) -> Option<Arc<Mutex<Room>>> {
        RoomManager::get_instance()
            .get_room_by_model(self.get_game_type().get_lobby_model())
    }

    /// Mirrors `sendSpectatorsToArena()`.
    pub fn send_spectators_to_arena(&self) {
        for spectator in self.get_spectators() {
            self.send_spectator_to_arena(&spectator);
        }
    }

    /// Mirrors `gamePrepare()`.
    pub fn game_prepare(&self) {
        for game_player in self.get_active_players() {
            let mut game_player = game_player.lock();
            game_player.set_xp(0);
            game_player.set_score(0);
        }
    }

    /// Mirrors `sendSpectatorToArena(GamePlayer)`.
    pub fn send_spectator_to_arena(&self, spectator: &Arc<Mutex<GamePlayer>>) {
        if !spectator.lock().is_spectator() {
            return;
        }

        let player = spectator.lock().get_player().clone();
        let same_room = player
            .lock()
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
            .map(|room| room.get_id() == self.get_room().get_id())
            .unwrap_or(false);

        if !same_room {
            player.lock().send(&GAMELOCATION);
            spectator.lock().set_entering_game(true);

            // No longer an observer
            self.remove_observer(&player);
        }
    }

    fn base(&self) -> &GameBase {
        match self {
            Self::SnowStorm { base, .. } => base,
            Self::BattleBall { base, .. } => base,
        }
    }

    fn game_state(&self) -> GameState {
        match self {
            Self::SnowStorm { game, .. } => game.get_game_state(),
            Self::BattleBall { game, .. } => game.get_game_state(),
        }
    }
}
