//! Mirrors `net.h4bbo.lisbon.game.games.wobblesquabble.WobbleSquabblePlayer`.
//! The `WobbleSquabbleGame` back-reference is a `Weak` (the `Arc` cycle
//! is broken in `WobbleSquabbleGame::new`).
use std::sync::{Arc, Weak};

use parking_lot::Mutex;

use crate::game::games::wobblesquabble::wobble_squabble_game::WobbleSquabbleGame;
use crate::game::games::wobblesquabble::wobble_squabble_move::WobbleSquabbleMove;
use crate::game::player::player::Player;

pub struct WobbleSquabblePlayer {
    player: Arc<Mutex<Player>>,
    ws_game: Weak<WobbleSquabbleGame>,
    position: i32,
    balance: i32,
    rebalanced: bool,
    hit: bool,
    requires_update: bool,
    move_: WobbleSquabbleMove,
    order: i32,
}

impl Clone for WobbleSquabblePlayer {
    fn clone(&self) -> Self {
        Self {
            player: Arc::clone(&self.player),
            ws_game: self.ws_game.clone(),
            position: self.position,
            balance: self.balance,
            rebalanced: self.rebalanced,
            hit: self.hit,
            requires_update: self.requires_update,
            move_: self.move_,
            order: self.order,
        }
    }
}

impl WobbleSquabblePlayer {
    /// Mirrors the `WobbleSquabblePlayer(WobbleSquabbleGame, Player, int)` constructor.
    pub fn new(
        ws_game: &Weak<WobbleSquabbleGame>,
        player: Arc<Mutex<Player>>,
        order: i32,
    ) -> Self {
        Self {
            player,
            ws_game: ws_game.clone(),
            position: 0,
            balance: 0,
            rebalanced: false,
            hit: false,
            requires_update: false,
            move_: WobbleSquabbleMove::None,
            order,
        }
    }

    /// Mirrors `isBalancing()`.
    pub fn is_balancing(&self) -> bool {
        self.balance > -100 && self.balance < 100
    }

    /// Mirrors `getScore()`.
    pub fn get_score(&self) -> i32 {
        if self.is_balancing() {
            if self.balance > 0 || self.balance == 0 {
                100 - self.balance
            } else {
                100 + self.balance
            }
        } else {
            0
        }
    }

    /// Mirrors `resetActions()`.
    pub fn reset_actions(&mut self) {
        self.move_ = WobbleSquabbleMove::None;
        self.requires_update = false;
        self.hit = false;
    }

    /// Mirrors `getPosition()`.
    pub fn get_position(&self) -> i32 {
        self.position
    }

    /// Mirrors `setPosition(int)`.
    pub fn set_position(&mut self, position: i32) {
        self.position = position
    }

    /// Mirrors `getBalance()`.
    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    /// Mirrors `setBalance(int)`.
    pub fn set_balance(&mut self, balance: i32) {
        self.balance = balance
    }

    /// Mirrors `isRebalanced()`.
    pub fn is_rebalanced(&self) -> bool {
        self.rebalanced
    }

    /// Mirrors `setRebalanced(boolean)`.
    pub fn set_rebalanced(&mut self, rebalanced: bool) {
        self.rebalanced = rebalanced
    }

    /// Mirrors `isHit()`.
    pub fn is_hit(&self) -> bool {
        self.hit
    }

    /// Mirrors `setHit(boolean)`.
    pub fn set_hit(&mut self, hit: bool) {
        self.hit = hit
    }

    /// Mirrors `isRequiresUpdate()`.
    pub fn is_requires_update(&self) -> bool {
        self.requires_update
    }

    /// Mirrors `setRequiresUpdate(boolean)`.
    pub fn set_requires_update(&mut self, requires_update: bool) {
        self.requires_update = requires_update
    }

    /// Mirrors `getMove()`.
    pub fn get_move(&self) -> WobbleSquabbleMove {
        self.move_
    }

    /// Mirrors `setMove(WobbleSquabbleMove)`.
    pub fn set_move(&mut self, move_: WobbleSquabbleMove) {
        self.move_ = move_
    }

    /// Mirrors `getGame()`.
    pub fn get_game(&self) -> Option<Arc<WobbleSquabbleGame>> {
        self.ws_game.upgrade()
    }

    /// Mirrors `getPlayer()`.
    pub fn get_player(&self) -> &Arc<Mutex<Player>> {
        &self.player
    }

    /// Mirrors `getOrder()`.
    pub fn get_order(&self) -> i32 {
        self.order
    }

    /// Mirrors `setOrder(int)`.
    pub fn set_order(&mut self, order: i32) {
        self.order = order
    }
}
