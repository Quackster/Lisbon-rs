//! Mirrors `net.h4bbo.lisbon.game.games.player.GameTeam`.
//!
//! The Java `Game` back-reference is omitted; the `calculateScore`
//! `instanceof BattleBallGame` branch needs it (the concrete game
//! constructors run before the `Game` enum wrapper exists, so the
//! `GameTeam` cannot hold it).
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use parking_lot::Mutex;

use crate::game::games::player::game_player::GamePlayer;

pub struct GameTeam {
    id: i32,
    player_list: Vec<Arc<Mutex<GamePlayer>>>,
    points: AtomicI32,
    // The Java `snowstormPoints` field is never initialised
    // (`NullPointerException` on use).
    snowstorm_points: AtomicI32,
}

impl GameTeam {
    /// Mirrors the `GameTeam(int, Game)` constructor (the
    // `CopyOnWriteArrayList` is a plain `Vec`; the `Game` parameter
    // is omitted, see the module note).
    pub fn new(id: i32) -> Self {
        Self {
            id,
            player_list: Vec::new(),
            points: AtomicI32::new(0),
            snowstorm_points: AtomicI32::new(0),
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getPlayers()` (the Java list is live; `add_player` mirrors
    /// the `CopyOnWriteArrayList.add` use).
    pub fn get_players(&self) -> Vec<Arc<Mutex<GamePlayer>>> {
        self.player_list.clone()
    }

    /// Mirrors `playerList.add(GamePlayer)`.
    pub fn add_player(&mut self, player: Arc<Mutex<GamePlayer>>) {
        self.player_list.push(player)
    }

    /// Mirrors `playerList.remove(GamePlayer)`.
    pub fn remove_player(&mut self, player: &Arc<Mutex<GamePlayer>>) {
        self.player_list
            .retain(|p| !Arc::ptr_eq(p, player))
    }

    /// Mirrors `playerList.clear()`.
    pub fn clear_players(&mut self) {
        self.player_list.clear()
    }

    /// Mirrors `calculateScore()`.
    pub fn calculate_score(&self) {
        self.points.store(0, Ordering::SeqCst);

        // Port note: the `instanceof BattleBallGame` branch needs the
        // `Game` back-reference, see the module note; the player score
        // sum (the Java `else` branch) is always applied.
        let total: i32 = self
            .player_list
            .iter()
            .map(|player| player.lock().get_score())
            .sum();

        self.points.fetch_add(total, Ordering::SeqCst);
    }

    /// Mirrors the shared `GameTeam` references the Java callers hold
    // (the `Arc` copy is a structural clone; the Java reference is not
    // expressible in Rust).
    pub fn clone_arc(&self) -> Arc<Self> {
        Arc::new(Self {
            id: self.id,
            player_list: self.player_list.clone(),
            points: AtomicI32::new(self.points.load(Ordering::SeqCst)),
            snowstorm_points: AtomicI32::new(self.snowstorm_points.load(Ordering::SeqCst)),
        })
    }

    /// Mirrors `getPoints()`.
    pub fn get_points(&self) -> i32 {
        self.points.load(Ordering::SeqCst)
    }

    /// Mirrors `getActivePlayers()`.
    pub fn get_active_players(&self) -> Vec<Arc<Mutex<GamePlayer>>> {
        self.player_list
            .iter()
            .filter(|player| player.lock().is_in_game())
            .cloned()
            .collect()
    }
}
