//! Mirrors `net.h4bbo.lisbon.game.games.enums.GameEventType`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameEventType {
    BattleballPlayerEvent,
    BattleballObjectSpawn,
    BattleballDespawnObject,
    BattleballPowerUpGet,
    BattleballPowerUpActivate,
}

impl GameEventType {
    /// Mirrors `getEventId()`.
    pub fn get_event_id(&self) -> i32 {
        match self {
            GameEventType::BattleballPlayerEvent => 2,
            GameEventType::BattleballObjectSpawn => 0,
            GameEventType::BattleballDespawnObject => 1,
            GameEventType::BattleballPowerUpGet => 3,
            GameEventType::BattleballPowerUpActivate => 5,
        }
    }
}
