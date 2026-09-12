//! Mirrors `net.h4bbo.lisbon.game.games.gamehalls.utils.GameToken`.

#[derive(Clone, Debug)]
pub struct GameToken {
    token: char,
    winning_token: char,
    moves: i32,
}

impl GameToken {
    /// Mirrors the `GameToken(char, char)` constructor.
    pub fn new(token: char, winning_token: char) -> Self {
        Self {
            token,
            winning_token,
            moves: 0,
        }
    }

    /// Mirrors `getToken()`.
    pub fn get_token(&self) -> char {
        self.token
    }

    /// Mirrors `getWinningToken()`.
    pub fn get_winning_token(&self) -> char {
        self.winning_token
    }

    /// Mirrors `getMoves()`.
    pub fn get_moves(&self) -> i32 {
        self.moves
    }

    /// Mirrors `incrementMoves()`.
    pub fn increment_moves(&mut self) {
        self.moves += 1;
    }
}
