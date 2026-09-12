//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.items.SLIDEOBJECTBUNDLE`.
use rand::Rng;

use crate::game::item::item::Item;
use crate::game::item::roller::rolling_data::RollingData;
use crate::game::pathfinder::position::Position;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;
use crate::util::string_util::StringUtil;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct SLIDEOBJECTBUNDLE {
    roller: Option<Item>,
    rolling_items: Vec<RollingData>,
    rolling_entity: Option<RollingData>,
    position: Position,
    dest_x: i32,
    dest_y: i32,
    dest_z: f64,
    id: i32,
}

impl SLIDEOBJECTBUNDLE {
    /// Mirrors the `SLIDEOBJECTBUNDLE(Position, int, int, float, int)`
    /// constructor.
    pub fn new(position: Position, dest_x: i32, dest_y: i32, dest_z: f64, id: i32) -> Self {
        Self {
            roller: None,
            rolling_items: Vec::new(),
            rolling_entity: None,
            position,
            dest_x,
            dest_y,
            dest_z,
            id,
        }
    }

    /// Mirrors the `SLIDEOBJECTBUNDLE(Item, List<RollingData>, RollingData)`
    /// constructor.
    pub fn new_roller(
        roller: Item,
        rolling_items: Vec<RollingData>,
        rolling_entity: Option<RollingData>,
    ) -> Self {
        Self {
            roller: Some(roller),
            rolling_items,
            rolling_entity,
            position: Position::default(),
            dest_x: 0,
            dest_y: 0,
            dest_z: 0.0,
            id: 0,
        }
    }
}

impl MessageComposer for SLIDEOBJECTBUNDLE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        if self.roller.is_none() {
            response.write_int(self.dest_x);
            response.write_int(self.dest_y);
            response.write_int(self.position.get_x());
            response.write_int(self.position.get_y());
            response.write_int(1);
            response.write_int(self.id);
            response.write_string(StringUtil::format(self.dest_z));
            response.write_string(StringUtil::format(self.position.get_z()));
            response.write_int(rand::thread_rng().gen_range(0..10000));
            response.write_int(0);
        } else {
            let roller = self.roller.as_ref().unwrap();
            let position = roller.get_position();
            let front = position.get_square_in_front();

            response.write_int(position.get_x());
            response.write_int(position.get_y());
            response.write_int(front.get_x());
            response.write_int(front.get_y());
            response.write_int(self.rolling_items.len() as i32);

            for item in &self.rolling_items {
                // The Java NPEs when the rolling data has no item.
                response.write_int(item.get_item().map_or(0, |item| item.get_id()));
                response.write_string(StringUtil::format(item.get_from_position().get_z()));
                response.write_string(StringUtil::format(item.get_next_position().get_z()));
            }

            response.write_int(roller.get_id());
            response.write_int(if self.rolling_entity.is_some() { 2 } else { 0 });

            if let Some(rolling_entity) = &self.rolling_entity {
                response.write_int(rolling_entity.get_entity_instance_id());
                response.write_string(StringUtil::format(rolling_entity.get_from_position().get_z()));
                response.write_string(StringUtil::format(rolling_entity.get_display_height()));
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        230 // "Cf"
    }
}
