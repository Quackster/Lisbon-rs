//! Mirrors `net.h4bbo.lisbon.game.games.wobblesquabble.WobbleSquabbleStatus`.

use crate::game::games::wobblesquabble::wobble_squabble_move::WobbleSquabbleMove;

pub struct WobbleSquabbleStatus {
    position: i32,
    balance: i32,
    move_: WobbleSquabbleMove,
    hit: bool,
}

impl WobbleSquabbleStatus {
    /// Mirrors the `WobbleSquabbleStatus(int, int, WobbleSquabbleMove, boolean)` constructor.
    pub fn new(position: i32, balance: i32, move_: WobbleSquabbleMove, is_hit: bool) -> Self {
        Self {
            position,
            balance,
            move_,
            hit: is_hit,
        }
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

    /// Mirrors `getMove()`.
    pub fn get_move(&self) -> WobbleSquabbleMove {
        self.move_
    }

    /// Mirrors `isHit()`.
    pub fn is_hit(&self) -> bool {
        self.hit
    }

    /// Mirrors `setHit(boolean)`.
    pub fn set_hit(&mut self, hit: bool) {
        self.hit = hit
    }
}
