//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.user.USER_OBJECTS`.
use std::collections::HashMap;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_state::EntityState;
use crate::game::entity::entity_type::EntityType;
use crate::game::room::room_user_status::RoomUserStatus;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;
use crate::util::string_util::StringUtil;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct USER_OBJECTS {
    states: Vec<EntityState>,
}

impl USER_OBJECTS {
    /// Mirrors the `USER_OBJECTS(List<Entity>)` constructor.
    pub fn new(entities: &[Box<dyn Entity + Send>]) -> Self {
        Self {
            states: Self::create_entity_states(entities),
        }
    }

    /// Mirrors the `USER_OBJECTS(Entity)` constructor.
    pub fn from_entity(entity: &(dyn Entity + Send)) -> Self {
        // The Java NPEs when there is no room user.
        let mut states = Vec::new();

        if let Some(state) = Self::create_entity_state(entity) {
            states.push(state);
        }

        Self { states }
    }

    /// Mirrors `createEntityStates(List<Entity>)`.
    fn create_entity_states(entities: &[Box<dyn Entity + Send>]) -> Vec<EntityState> {
        entities
            .iter()
            .filter_map(|entity| Self::create_entity_state(entity.as_ref()))
            .collect()
    }

    /// Mirrors the per-entity part of `createEntityStates(List<Entity>)`.
    fn create_entity_state(user: &(dyn Entity + Send)) -> Option<EntityState> {
        // The Java NPEs when there is no room user.
        let room_user = user.get_room_user()?;

        let details = user.get_details().clone();
        let room = room_user.get_room().unwrap_or_default();
        let position = room_user.get_position().copy();
        let statuses: HashMap<String, RoomUserStatus> = room_user.get_statuses();

        let mut state = EntityState::new(
            details,
            user.get_details().get_id(),
            room_user.get_instance_id(),
            user.get_type(),
            room,
            position,
            statuses,
        );

        if let Some(player) = user.as_player() {
            state.add_badges(player.get_badge_manager().get_equipped_badges());
        }

        Some(state)
    }

    /// Mirrors `serialiseBadges(EntityState)`.
    fn serialise_badges(&self, states: &EntityState) -> String {
        let mut badges: Vec<&crate::game::badges::badge::Badge> = states
            .get_badges()
            .iter()
            .filter(|badge| badge.get_slot_id() > 0 && !badge.get_badge_code().is_empty())
            .collect();
        badges.sort_by_key(|badge| badge.get_slot_id());

        badges
            .iter()
            .map(|badge| format!("{}:{}", badge.get_slot_id(), badge.get_badge_code()))
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl MessageComposer for USER_OBJECTS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        for state in &self.states {
            if state.get_entity_type() == EntityType::Pet {
                response.write_key_value("i", state.get_instance_id());
                response.write_key_value(
                    "n",
                    format!(
                        "{}{}{}",
                        state.get_instance_id(),
                        '\u{0004}',
                        state.get_details().get_name()
                    ),
                );
                response.write_key_value("f", state.get_details().get_figure());
                response.write_key_value(
                    "l",
                    format!(
                        "{} {} {}",
                        state.get_position().get_x(),
                        state.get_position().get_y(),
                        state.get_position().get_z() as i32
                    ),
                );
                response.write_key_value("c", "");
            } else {
                response.write_key_value("i", state.get_instance_id());
                response.write_key_value("a", state.get_entity_id());
                response.write_key_value("n", state.get_details().get_name());
                response.write_key_value("f", state.get_details().get_figure());
                response.write_key_value(
                    "l",
                    format!(
                        "{} {} {}",
                        state.get_position().get_x(),
                        state.get_position().get_y(),
                        StringUtil::format(state.get_position().get_z())
                    ),
                );
                response.write_key_value("c", state.get_details().get_motto());
                response.write_key_value("s", state.get_details().get_sex());
                response.write_key_value("b", self.serialise_badges(state));

                if let Some(model) = state.get_room().get_model() {
                    if model.get_name().starts_with("pool_")
                        || model.get_name() == "md_a"
                    {
                        let pool_figure = state.get_details().get_pool_figure();

                        if !pool_figure.is_empty() {
                            response.write_key_value("p", pool_figure);
                        }
                    }
                }

                if let Some(group_member) = state.get_group_member() {
                    response.write_key_value("g", group_member.get_group_id());
                    // The Java NPEs when the member rank is unset.
                    response.write_key_value(
                        "t",
                        group_member
                            .get_member_rank()
                            .map_or(0, |rank| rank.get_client_rank()),
                    );
                }

                if state.get_entity_type() == EntityType::Bot {
                    response.write_delimeter("[bot]", '\u{000D}');
                }
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        28 // "@\""
    }
}
