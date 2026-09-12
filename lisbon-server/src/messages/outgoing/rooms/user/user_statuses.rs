//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.user.USER_STATUSES`.
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_state::EntityState;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;
use crate::util::string_util::StringUtil;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct USER_STATUSES {
    states: Vec<EntityState>,
}

impl USER_STATUSES {
    /// Mirrors the `USER_STATUSES(List<Entity>)` constructor
    /// (`createEntityStates`).
    pub fn new(users: Vec<&(dyn Entity + Send)>) -> Self {
        let mut states = Vec::new();

        for user in users {
            // The Java NPEs when the entity has no room user.
            let Some(room_user) = user.get_room_user() else {
                continue;
            };

            states.push(EntityState::new(
                user.get_details().clone(),
                user.get_details().get_id(),
                room_user.get_instance_id(),
                user.get_type(),
                room_user.get_room().unwrap_or_default(),
                room_user.get_position().copy(),
                room_user.get_statuses().clone(),
            ));
        }

        Self { states }
    }
}

impl MessageComposer for USER_STATUSES {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        for state in &self.states {
            let position = state.get_position();
            response.write_delimeter(state.get_instance_id(), " ");
            response.write_delimeter(position.get_x(), ",");
            response.write_delimeter(position.get_y(), ",");
            response.write_delimeter(StringUtil::format(position.get_z()), ",");
            response.write_delimeter(position.get_head_rotation(), ",");
            response.write_delimeter(position.get_body_rotation(), "/");

            for status in state.get_statuses().values() {
                response.write(status.get_key().status_code());

                if !status.get_value().is_empty() {
                    response.write(" ");
                    response.write(status.get_value());
                }

                response.write("/");
            }

            // The Java writes `Character.toString((char) 13)` (the raw
            // 0x0D byte).
            response.write("\u{0d}");
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        34 // "@b"
    }
}
