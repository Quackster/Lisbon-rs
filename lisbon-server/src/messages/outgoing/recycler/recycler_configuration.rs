//! Mirrors `net.h4bbo.lisbon.messages.outgoing.recycler.RECYCLER_CONFIGURATION`.
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::recycler::recycler_reward::RecyclerReward;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct RECYCLER_CONFIGURATION {
    is_recycler_enabled: bool,
    recycler_rewards: Vec<RecyclerReward>,
    recycler_timeout_seconds: i32,
    recycler_item_quarantine_seconds: i32,
    recycler_session_length_seconds: i32,
}

impl RECYCLER_CONFIGURATION {
    /// Mirrors the 5-arg `RECYCLER_CONFIGURATION` constructor.
    pub fn new(
        is_recycler_enabled: bool,
        recycler_rewards: Vec<RecyclerReward>,
        recycler_timeout_seconds: i32,
        recycler_item_quarantine_seconds: i32,
        recycler_session_length_seconds: i32,
    ) -> Self {
        Self {
            is_recycler_enabled,
            recycler_rewards,
            recycler_timeout_seconds,
            recycler_item_quarantine_seconds,
            recycler_session_length_seconds,
        }
    }
}

impl MessageComposer for RECYCLER_CONFIGURATION {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_bool(self.is_recycler_enabled);
        response.write_int(self.recycler_item_quarantine_seconds / 60);
        response.write_int(self.recycler_session_length_seconds / 60);
        response.write_int(self.recycler_timeout_seconds / 60);
        response.write_int(self.recycler_rewards.len() as i32);

        for recycler_reward in &self.recycler_rewards {
            response.write_int(recycler_reward.get_item_cost());

            let catalogue_item = match recycler_reward.get_catalogue_item() {
                Some(item) => item,
                // The Java NPEs for a missing catalogue item.
                None => continue,
            };

            let definition = match catalogue_item.get_definition() {
                Some(definition) => definition,
                // The Java NPEs for a missing definition.
                None => continue,
            };

            let is_wall_item = definition.has_behaviour(ItemBehaviour::WallItem);
            response.write_bool(is_wall_item);
            response.write_string(definition.get_sprite());

            if is_wall_item {
                if catalogue_item.get_item_special_id() > 0 {
                    response.write_string(" ");
                    response.write_string(catalogue_item.get_item_special_id());
                }
            } else {
                response.write_int(0);
                response.write_int(definition.get_length());
                response.write_int(definition.get_width());
                response.write_string(definition.get_colour());
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        303 // "Do"
    }
}
