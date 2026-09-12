//! Mirrors `net.h4bbo.lisbon.game.room.models.RoomModelTriggerType`.
use crate::game::room::models::triggers::RoomTrigger;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoomModelTriggerType {
    FlatTrigger,
    BattleballLobbyTrigger,
    SnowstormLobbyTrigger,
    SpaceCafeTrigger,
    HabboLidoTrigger,
    RooftopRumbleTrigger,
    DivingDeckTrigger,
    InfobusPark,
    InfobusPoll,
    None,
}

impl RoomModelTriggerType {
    /// Mirrors `Enum.valueOf` (returns `None` for unknown names; Java
    /// throws).
    pub fn from_str(name: &str) -> Option<Self> {
        match name.to_uppercase().as_str() {
            "FLAT_TRIGGER" => Some(Self::FlatTrigger),
            "BATTLEBALL_LOBBY_TRIGGER" => Some(Self::BattleballLobbyTrigger),
            "SNOWSTORM_LOBBY_TRIGGER" => Some(Self::SnowstormLobbyTrigger),
            "SPACE_CAFE_TRIGGER" => Some(Self::SpaceCafeTrigger),
            "HABBO_LIDO_TRIGGER" => Some(Self::HabboLidoTrigger),
            "ROOFTOP_RUMBLE_TRIGGER" => Some(Self::RooftopRumbleTrigger),
            "DIVING_DECK_TRIGGER" => Some(Self::DivingDeckTrigger),
            "INFOBUS_PARK" => Some(Self::InfobusPark),
            "INFOBUS_POLL" => Some(Self::InfobusPoll),
            "NONE" => Some(Self::None),
            _ => None,
        }
    }

    /// Mirrors `getRoomTrigger`.
    pub fn get_room_trigger(&self) -> Option<RoomTrigger> {
        match self {
            Self::FlatTrigger => Some(RoomTrigger::Flat(
                crate::game::room::models::triggers::flat_trigger::FlatTrigger,
            )),
            Self::BattleballLobbyTrigger => Some(RoomTrigger::BattleballLobby(
                crate::game::room::models::triggers::battleball_lobby_trigger::BattleballLobbyTrigger,
            )),
            Self::SnowstormLobbyTrigger => Some(RoomTrigger::SnowstormLobby(
                crate::game::room::models::triggers::snow_storm_lobby_trigger::SnowStormLobbyTrigger,
            )),
            Self::SpaceCafeTrigger => Some(RoomTrigger::SpaceCafe(
                crate::game::room::models::triggers::space_cafe_trigger::SpaceCafeTrigger,
            )),
            Self::HabboLidoTrigger => Some(RoomTrigger::HabboLido(
                crate::game::room::models::triggers::habbo_lido_trigger::HabboLidoTrigger,
            )),
            Self::RooftopRumbleTrigger => Some(RoomTrigger::RooftopRumble(
                crate::game::room::models::triggers::rooftop_rumble_trigger::RooftopRumbleTrigger,
            )),
            Self::DivingDeckTrigger => Some(RoomTrigger::DivingDeck(
                crate::game::room::models::triggers::diving_deck_trigger::DivingDeckTrigger,
            )),
            Self::InfobusPark => Some(RoomTrigger::InfobusPark(
                crate::game::room::models::triggers::infobus_park_trigger::InfobusParkTrigger,
            )),
            Self::InfobusPoll => Some(RoomTrigger::InfobusPoll(
                crate::game::room::models::triggers::infobus_poll_trigger::InfobusPollTrigger,
            )),
            Self::None => None,
        }
    }
}
