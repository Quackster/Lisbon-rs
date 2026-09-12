//! Mirrors `net.h4bbo.lisbon.game.entity.EntityState`.

use std::collections::HashMap;

use crate::game::badges::badge::Badge;
use crate::game::entity::entity_type::EntityType;
use crate::game::groups::group_member::GroupMember;
use crate::game::pathfinder::position::Position;
use crate::game::player::player_details::PlayerDetails;
use crate::game::room::room::Room;
use crate::game::room::room_user_status::RoomUserStatus;

#[derive(Clone, Debug)]
pub struct EntityState {
    entity_id: i32,
    instance_id: i32,
    details: PlayerDetails,
    entity_type: EntityType,
    room: Room,
    position: Position,
    statuses: HashMap<String, RoomUserStatus>,
    badge_list: Vec<Badge>,
    group_member: Option<GroupMember>,
}

impl EntityState {
    /// Mirrors the `EntityState(...)` constructor.
    pub fn new(
        mut details: PlayerDetails,
        entity_id: i32,
        instance_id: i32,
        entity_type: EntityType,
        room: Room,
        position: Position,
        statuses: HashMap<String, RoomUserStatus>,
    ) -> Self {
        let group_member = if details.get_favourite_group_id() > 0 {
            details.get_group_member().cloned()
        } else {
            None
        };

        Self {
            entity_id,
            instance_id,
            details,
            entity_type,
            room,
            position,
            statuses,
            badge_list: Vec::new(),
            group_member,
        }
    }

    /// Mirrors `getInstanceId`.
    pub fn get_instance_id(&self) -> i32 {
        self.instance_id
    }

    /// Mirrors `getPosition`.
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// Mirrors `getStatuses` (Java uses a `ConcurrentHashMap`; a plain
    /// `HashMap` is used in the port).
    pub fn get_statuses(&self) -> &HashMap<String, RoomUserStatus> {
        &self.statuses
    }

    /// Mirrors `getEntityId`.
    pub fn get_entity_id(&self) -> i32 {
        self.entity_id
    }

    /// Mirrors `getDetails`.
    pub fn get_details(&self) -> &PlayerDetails {
        &self.details
    }

    /// Mirrors `getEntityType`.
    pub fn get_entity_type(&self) -> EntityType {
        self.entity_type
    }

    /// Mirrors `getRoom`.
    pub fn get_room(&self) -> &Room {
        &self.room
    }

    /// Mirrors `getBadges`.
    pub fn get_badges(&self) -> &[Badge] {
        &self.badge_list
    }

    /// Mirrors `getBadges().addAll(...)` (the Java `getBadges` returns the
    /// live list).
    pub fn add_badges(&mut self, badges: Vec<Badge>) {
        self.badge_list.extend(badges);
    }

    /// Mirrors `getGroupMember`.
    pub fn get_group_member(&self) -> Option<&GroupMember> {
        self.group_member.as_ref()
    }
}
