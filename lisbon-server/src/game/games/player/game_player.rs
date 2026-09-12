//! Mirrors `net.h4bbo.lisbon.game.games.player.GamePlayer`.
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use parking_lot::Mutex;

use crate::game::games::battleball::enums::battle_ball_player_state::BattleBallPlayerState;
use crate::game::entity::entity::Entity;
use crate::game::games::game::Game;
use crate::game::games::game_manager::GameManager;
use crate::game::games::game_object::GameObject;
use crate::game::games::player::game_team::GameTeam;
use crate::game::games::snowstorm::util::snow_storm_attributes::SnowStormAttributes;
use crate::game::pathfinder::position::Position;
use crate::game::player::player::Player;

pub struct GamePlayer {
    player: Arc<Mutex<Player>>,
    game_object: Option<std::sync::Arc<dyn GameObject>>,
    user_id: i32,
    object_id: i32,
    game_id: i32,
    team_id: i32,
    position: Position,
    entering_game: bool,
    is_spectator: bool,
    in_game: bool,
    clicked_restart: bool,
    player_state: BattleBallPlayerState,
    harlequin_player: Option<Arc<Mutex<GamePlayer>>>,
    assigned_spawn: bool,
    score: AtomicI32,
    xp: i32,
    snow_storm_attributes: SnowStormAttributes,
}

impl GamePlayer {
    /// Mirrors the `GamePlayer(Player)` constructor (the `new Position()` is
    /// the Rust `Default`).
    pub fn new(player: Arc<Mutex<Player>>) -> Self {
        let user_id = player.lock().get_details().get_id();

        Self {
            player,
            game_object: None,
            user_id,
            object_id: -1,
            game_id: -1,
            team_id: -1,
            position: Position::default(),
            entering_game: false,
            is_spectator: false,
            in_game: false,
            clicked_restart: false,
            player_state: BattleBallPlayerState::Normal,
            harlequin_player: None,
            assigned_spawn: false,
            score: AtomicI32::new(0),
            xp: 0,
            snow_storm_attributes: SnowStormAttributes::new(),
        }
    }

    /// Mirrors `setScore(int)`.
    pub fn set_score(&self, score: i32) {
        self.score.store(score, Ordering::SeqCst)
    }

    /// Mirrors `calculateScore()`.
    pub fn calculate_score(&self) {
        if !self.in_game {
            self.score.store(0, Ordering::SeqCst);
            return;
        }

        if let Some(game) = self.get_game() {
            if let Some(battleball_game) = game.as_battle_ball() {
                self.score.store(0, Ordering::SeqCst);

                for battleball_tile in battleball_game.get_tiles() {
                    for score_reference in battleball_tile.lock().get_points_referece() {
                        if score_reference.get_by() != self.user_id {
                            continue;
                        }

                        self.score
                            .fetch_add(score_reference.get_score(), Ordering::SeqCst);
                    }
                }
            }
        }
    }

    /// Mirrors `getScore()`.
    pub fn get_score(&self) -> i32 {
        self.score.load(Ordering::SeqCst)
    }

    /// Mirrors `getXp()`.
    pub fn get_xp(&self) -> i32 {
        self.xp
    }

    /// Mirrors `setXp(int)` (does NOT allow negative numbers).
    pub fn set_xp(&mut self, xp: i32) {
        self.xp = xp;
        if self.xp < 0 {
            self.xp = 0;
        }
    }

    /// Mirrors `getGame()`.
    pub fn get_game(&self) -> Option<Game> {
        GameManager::get_instance().get_game_by_id(self.game_id)
    }

    /// Mirrors `getPlayer()`.
    pub fn get_player(&self) -> &Arc<Mutex<Player>> {
        &self.player
    }

    /// Mirrors `getUserId()`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `getTeam()` (the Harlequin power up is taken into account).
    pub fn get_team(&self) -> Option<Arc<Mutex<GameTeam>>> {
        let team_id = self
            .harlequin_player
            .as_ref()
            .map(|harlequin| harlequin.lock().get_team_id())
            .unwrap_or(self.team_id);

        self.get_game()?.get_team(team_id)
    }

    /// Mirrors `getTeamId()`.
    pub fn get_team_id(&self) -> i32 {
        self.team_id
    }

    /// Mirrors `setTeamId(int)`.
    pub fn set_team_id(&mut self, team_id: i32) {
        self.team_id = team_id
    }

    /// Mirrors `getSpawnPosition()`.
    pub fn get_spawn_position(&self) -> &Position {
        &self.position
    }

    /// Mirrors `getSpawnPosition().setX/Y/Z/Rotation` (the Java code
    // mutates the `Position` in place; Rust `Position` is `Copy`).
    pub fn set_spawn_position(&mut self, position: Position) {
        self.position = position;
    }

    /// Mirrors `getGameId()`.
    pub fn get_game_id(&self) -> i32 {
        self.game_id
    }

    /// Mirrors `setGameId(int)`.
    pub fn set_game_id(&mut self, game_id: i32) {
        self.game_id = game_id
    }

    /// Mirrors `isEnteringGame()`.
    pub fn is_entering_game(&self) -> bool {
        self.entering_game
    }

    /// Mirrors `setEnteringGame(boolean)`.
    pub fn set_entering_game(&mut self, entering_game: bool) {
        self.entering_game = entering_game
    }

    /// Mirrors `isInGame()`.
    pub fn is_in_game(&self) -> bool {
        self.in_game
    }

    /// Mirrors `setInGame(boolean)`.
    pub fn set_in_game(&mut self, in_game: bool) {
        self.in_game = in_game
    }

    /// Mirrors `isClickedRestart()`.
    pub fn is_clicked_restart(&self) -> bool {
        self.clicked_restart
    }

    /// Mirrors `setClickedRestart(boolean)`.
    pub fn set_clicked_restart(&mut self, clicked_restart: bool) {
        self.clicked_restart = clicked_restart
    }

    /// Mirrors `getColouringForOpponentId()`.
    pub fn get_colouring_for_opponent_id(&self) -> i32 {
        self.harlequin_player
            .as_ref()
            .map(|harlequin| harlequin.lock().get_object_id())
            .unwrap_or(-1)
    }

    /// Mirrors `getHarlequinPlayer()`.
    pub fn get_harlequin_player(&self) -> Option<&Arc<Mutex<GamePlayer>>> {
        self.harlequin_player.as_ref()
    }

    /// Mirrors `setHarlequinPlayer(GamePlayer)`.
    pub fn set_harlequin_player(&mut self, harlequin_player: Option<Arc<Mutex<GamePlayer>>>) {
        self.harlequin_player = harlequin_player
    }

    /// Mirrors `isSpectator()`.
    pub fn is_spectator(&self) -> bool {
        self.is_spectator
    }

    /// Mirrors `setSpectator(boolean)`.
    pub fn set_spectator(&mut self, spectator: bool) {
        self.is_spectator = spectator
    }

    /// Mirrors `getPlayerState()`.
    pub fn get_player_state(&self) -> BattleBallPlayerState {
        self.player_state
    }

    /// Mirrors `setPlayerState(BattleBallPlayerState)`.
    pub fn set_player_state(&mut self, player_state: BattleBallPlayerState) {
        self.player_state = player_state
    }

    /// Mirrors `getGameObject()`.
    pub fn get_game_object(&self) -> Option<&(dyn GameObject + 'static)> {
        self.game_object.as_deref()
    }

    /// Mirrors `setGameObject(GameObject)`.
    pub fn set_game_object(&mut self, game_object: Option<std::sync::Arc<dyn GameObject>>) {
        self.game_object = game_object
    }

    /// Mirrors `getObjectId()`.
    pub fn get_object_id(&self) -> i32 {
        self.object_id
    }

    /// Mirrors `setObjectId(int)`.
    pub fn set_object_id(&mut self, object_id: i32) {
        self.object_id = object_id
    }

    /// Mirrors `getSnowStormAttributes()`.
    pub fn get_snow_storm_attributes(&self) -> &SnowStormAttributes {
        &self.snow_storm_attributes
    }

    /// Mirrors the `getSnowStormAttributes` field access.
    pub fn get_snow_storm_attributes_mut(&mut self) -> &mut SnowStormAttributes {
        &mut self.snow_storm_attributes
    }

    /// Mirrors `isAssignedSpawn()`.
    pub fn is_assigned_spawn(&self) -> bool {
        self.assigned_spawn
    }

    /// Mirrors `setAssignedSpawn(boolean)`.
    pub fn set_assigned_spawn(&mut self, assigned_spawn: bool) {
        self.assigned_spawn = assigned_spawn
    }
}
