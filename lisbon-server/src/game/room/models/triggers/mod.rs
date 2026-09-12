//! Mirrors `net.h4bbo.lisbon.game.room.models.triggers.*`.
pub mod battleball_lobby_trigger;
pub mod diving_deck_trigger;
pub mod flat_trigger;
pub mod habbo_lido_trigger;
pub mod infobus_park_trigger;
pub mod infobus_poll_trigger;
pub mod rooftop_rumble_trigger;
pub mod snow_storm_lobby_trigger;
pub mod space_cafe_trigger;

use std::any::Any;

use crate::game::entity::entity::Entity;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::room::room::Room;
use crate::game::triggers::game_lobby_trigger::GameLobbyTrigger;
use crate::game::triggers::generic_trigger::Trigger;

use self::battleball_lobby_trigger::BattleballLobbyTrigger;
use self::diving_deck_trigger::DivingDeckTrigger;
use self::flat_trigger::FlatTrigger;
use self::habbo_lido_trigger::HabboLidoTrigger;
use self::infobus_park_trigger::InfobusParkTrigger;
use self::infobus_poll_trigger::InfobusPollTrigger;
use self::rooftop_rumble_trigger::RooftopRumbleTrigger;
use self::snow_storm_lobby_trigger::SnowStormLobbyTrigger;
use self::space_cafe_trigger::SpaceCafeTrigger;

/// Mirrors the Java `GenericTrigger` reference returned by
/// `RoomModelTriggerType.getRoomTrigger` (the concrete trigger
/// variants).
pub enum RoomTrigger {
    Flat(FlatTrigger),
    BattleballLobby(BattleballLobbyTrigger),
    SnowstormLobby(SnowStormLobbyTrigger),
    SpaceCafe(SpaceCafeTrigger),
    HabboLido(HabboLidoTrigger),
    RooftopRumble(RooftopRumbleTrigger),
    DivingDeck(DivingDeckTrigger),
    InfobusPark(InfobusParkTrigger),
    InfobusPoll(InfobusPollTrigger),
}

impl Trigger for RoomTrigger {
    fn on_room_entry(
        &self,
        entity: &dyn Entity,
        room: &Room,
        first_entry: bool,
        custom_args: &[Box<dyn Any>],
    ) {
        match self {
            Self::Flat(trigger) => trigger.on_room_entry(entity, room, first_entry, custom_args),
            Self::BattleballLobby(trigger) => {
                trigger.on_room_entry(entity, room, first_entry, custom_args)
            }
            Self::SnowstormLobby(trigger) => {
                trigger.on_room_entry(entity, room, first_entry, custom_args)
            }
            Self::SpaceCafe(trigger) => {
                trigger.on_room_entry(entity, room, first_entry, custom_args)
            }
            Self::HabboLido(trigger) => {
                trigger.on_room_entry(entity, room, first_entry, custom_args)
            }
            Self::RooftopRumble(trigger) => {
                trigger.on_room_entry(entity, room, first_entry, custom_args)
            }
            Self::DivingDeck(trigger) => {
                trigger.on_room_entry(entity, room, first_entry, custom_args)
            }
            Self::InfobusPark(trigger) => {
                trigger.on_room_entry(entity, room, first_entry, custom_args)
            }
            Self::InfobusPoll(trigger) => {
                trigger.on_room_entry(entity, room, first_entry, custom_args)
            }
        }
    }

    fn on_room_leave(
        &self,
        entity: &dyn Entity,
        room: &Room,
        custom_args: &[Box<dyn Any>],
    ) {
        match self {
            Self::Flat(trigger) => trigger.on_room_leave(entity, room, custom_args),
            Self::BattleballLobby(trigger) => trigger.on_room_leave(entity, room, custom_args),
            Self::SnowstormLobby(trigger) => trigger.on_room_leave(entity, room, custom_args),
            Self::SpaceCafe(trigger) => trigger.on_room_leave(entity, room, custom_args),
            Self::HabboLido(trigger) => trigger.on_room_leave(entity, room, custom_args),
            Self::RooftopRumble(trigger) => trigger.on_room_leave(entity, room, custom_args),
            Self::DivingDeck(trigger) => trigger.on_room_leave(entity, room, custom_args),
            Self::InfobusPark(trigger) => trigger.on_room_leave(entity, room, custom_args),
            Self::InfobusPoll(trigger) => trigger.on_room_leave(entity, room, custom_args),
        }
    }

    fn on_entity_move(&self, entity: &dyn Entity, position: &Position, room: &Room) {
        match self {
            Self::Flat(trigger) => trigger.on_entity_move(entity, position, room),
            Self::BattleballLobby(trigger) => trigger.on_entity_move(entity, position, room),
            Self::SnowstormLobby(trigger) => trigger.on_entity_move(entity, position, room),
            Self::SpaceCafe(trigger) => trigger.on_entity_move(entity, position, room),
            Self::HabboLido(trigger) => trigger.on_entity_move(entity, position, room),
            Self::RooftopRumble(trigger) => trigger.on_entity_move(entity, position, room),
            Self::DivingDeck(trigger) => trigger.on_entity_move(entity, position, room),
            Self::InfobusPark(trigger) => trigger.on_entity_move(entity, position, room),
            Self::InfobusPoll(trigger) => trigger.on_entity_move(entity, position, room),
        }
    }

    fn on_entity_leave(&self, entity: &dyn Entity, room_entity: &RoomEntity, item: &Item) {
        match self {
            Self::Flat(trigger) => trigger.on_entity_leave(entity, room_entity, item),
            Self::BattleballLobby(trigger) => trigger.on_entity_leave(entity, room_entity, item),
            Self::SnowstormLobby(trigger) => trigger.on_entity_leave(entity, room_entity, item),
            Self::SpaceCafe(trigger) => trigger.on_entity_leave(entity, room_entity, item),
            Self::HabboLido(trigger) => trigger.on_entity_leave(entity, room_entity, item),
            Self::RooftopRumble(trigger) => trigger.on_entity_leave(entity, room_entity, item),
            Self::DivingDeck(trigger) => trigger.on_entity_leave(entity, room_entity, item),
            Self::InfobusPark(trigger) => trigger.on_entity_leave(entity, room_entity, item),
            Self::InfobusPoll(trigger) => trigger.on_entity_leave(entity, room_entity, item),
        }
    }

    fn on_entity_stop(
        &self,
        entity: &dyn Entity,
        room_entity: &RoomEntity,
        item: &Item,
        is_rotation: bool,
    ) {
        match self {
            Self::Flat(trigger) => trigger.on_entity_stop(entity, room_entity, item, is_rotation),
            Self::BattleballLobby(trigger) => {
                trigger.on_entity_stop(entity, room_entity, item, is_rotation)
            }
            Self::SnowstormLobby(trigger) => {
                trigger.on_entity_stop(entity, room_entity, item, is_rotation)
            }
            Self::SpaceCafe(trigger) => {
                trigger.on_entity_stop(entity, room_entity, item, is_rotation)
            }
            Self::HabboLido(trigger) => trigger.on_entity_stop(entity, room_entity, item, is_rotation),
            Self::RooftopRumble(trigger) => {
                trigger.on_entity_stop(entity, room_entity, item, is_rotation)
            }
            Self::DivingDeck(trigger) => trigger.on_entity_stop(entity, room_entity, item, is_rotation),
            Self::InfobusPark(trigger) => {
                trigger.on_entity_stop(entity, room_entity, item, is_rotation)
            }
            Self::InfobusPoll(trigger) => {
                trigger.on_entity_stop(entity, room_entity, item, is_rotation)
            }
        }
    }

    fn on_entity_update(&self, entity: &dyn Entity, room_entity: &RoomEntity, item: &Item) {
        match self {
            Self::Flat(trigger) => trigger.on_entity_update(entity, room_entity, item),
            Self::BattleballLobby(trigger) => trigger.on_entity_update(entity, room_entity, item),
            Self::SnowstormLobby(trigger) => trigger.on_entity_update(entity, room_entity, item),
            Self::SpaceCafe(trigger) => trigger.on_entity_update(entity, room_entity, item),
            Self::HabboLido(trigger) => trigger.on_entity_update(entity, room_entity, item),
            Self::RooftopRumble(trigger) => trigger.on_entity_update(entity, room_entity, item),
            Self::DivingDeck(trigger) => trigger.on_entity_update(entity, room_entity, item),
            Self::InfobusPark(trigger) => trigger.on_entity_update(entity, room_entity, item),
            Self::InfobusPoll(trigger) => trigger.on_entity_update(entity, room_entity, item),
        }
    }

    fn on_entity_sit_down(&self, entity: &dyn Entity, room_entity: &RoomEntity, item: &Item) {
        match self {
            Self::Flat(trigger) => trigger.on_entity_sit_down(entity, room_entity, item),
            Self::BattleballLobby(trigger) => trigger.on_entity_sit_down(entity, room_entity, item),
            Self::SnowstormLobby(trigger) => trigger.on_entity_sit_down(entity, room_entity, item),
            Self::SpaceCafe(trigger) => trigger.on_entity_sit_down(entity, room_entity, item),
            Self::HabboLido(trigger) => trigger.on_entity_sit_down(entity, room_entity, item),
            Self::RooftopRumble(trigger) => trigger.on_entity_sit_down(entity, room_entity, item),
            Self::DivingDeck(trigger) => trigger.on_entity_sit_down(entity, room_entity, item),
            Self::InfobusPark(trigger) => trigger.on_entity_sit_down(entity, room_entity, item),
            Self::InfobusPoll(trigger) => trigger.on_entity_sit_down(entity, room_entity, item),
        }
    }

    fn on_entity_wake_up(&self, entity: &dyn Entity, room_entity: &RoomEntity, item: &Item) {
        match self {
            Self::Flat(trigger) => trigger.on_entity_wake_up(entity, room_entity, item),
            Self::BattleballLobby(trigger) => trigger.on_entity_wake_up(entity, room_entity, item),
            Self::SnowstormLobby(trigger) => trigger.on_entity_wake_up(entity, room_entity, item),
            Self::SpaceCafe(trigger) => trigger.on_entity_wake_up(entity, room_entity, item),
            Self::HabboLido(trigger) => trigger.on_entity_wake_up(entity, room_entity, item),
            Self::RooftopRumble(trigger) => trigger.on_entity_wake_up(entity, room_entity, item),
            Self::DivingDeck(trigger) => trigger.on_entity_wake_up(entity, room_entity, item),
            Self::InfobusPark(trigger) => trigger.on_entity_wake_up(entity, room_entity, item),
            Self::InfobusPoll(trigger) => trigger.on_entity_wake_up(entity, room_entity, item),
        }
    }
}

impl RoomTrigger {
    /// Mirrors the Java `instanceof GameLobbyTrigger` checks.
    pub fn as_game_lobby(&self) -> Option<&dyn GameLobbyTrigger> {
        match self {
            Self::BattleballLobby(trigger) => Some(trigger),
            Self::SnowstormLobby(trigger) => Some(trigger),
            _ => None,
        }
    }
}
