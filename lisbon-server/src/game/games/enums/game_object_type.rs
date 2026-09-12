//! Mirrors `net.h4bbo.lisbon.game.games.enums.GameObjectType`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameObjectType {
    BattleballPlayerObject,
    BattleballPowerObject,
    BattleballPinObject,
    SnowwarAvatarObject,
    SnowwarSnowmachineObject,
    SnowwarObjectEvent,
    SnowwarAvatarMoveEvent,
    SnowwarAvatarStopEvent,
    SnowwarRemoveObjectEvent,
    SnowwarThrowEvent,
    SnowwarCreateSnowballEvent,
    SnowwarTargetThrowEvent,
    SnowwarMachineMoveSnowballsEvent,
    SnowwarMachineAddSnowballEvent,
    SnowstormHitEvent,
    SnowwarStunEvent,
}

impl GameObjectType {
    /// Mirrors `getObjectId()`.
    pub fn get_object_id(&self) -> i32 {
        match self {
            GameObjectType::BattleballPlayerObject => 0,
            GameObjectType::BattleballPowerObject => 1,
            GameObjectType::BattleballPinObject => 2,
            GameObjectType::SnowwarAvatarObject => 5,
            GameObjectType::SnowwarSnowmachineObject => 4,
            GameObjectType::SnowwarObjectEvent => 0,
            GameObjectType::SnowwarAvatarMoveEvent => 2,
            GameObjectType::SnowwarAvatarStopEvent => 6,
            GameObjectType::SnowwarRemoveObjectEvent => 1,
            GameObjectType::SnowwarThrowEvent => 8,
            GameObjectType::SnowwarCreateSnowballEvent => 7,
            GameObjectType::SnowwarTargetThrowEvent => 4,
            GameObjectType::SnowwarMachineMoveSnowballsEvent => 12,
            GameObjectType::SnowwarMachineAddSnowballEvent => 11,
            GameObjectType::SnowstormHitEvent => 5,
            GameObjectType::SnowwarStunEvent => 9,
        }
    }
}
