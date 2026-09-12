//! Mirrors `net.h4bbo.lisbon.messages.incoming.infobus.TRYBUS`.
use crate::game::entity::entity::Entity;
use crate::game::infobus::infobus_manager::InfobusManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::infobus::cannot_enter_bus::CANNOT_ENTER_BUS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct TRYBUS;

impl MessageEvent for TRYBUS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        // Do not process public room items
        if !room.is_public_room() {
            return Ok(());
        }

        let infobus_manager = InfobusManager::get_instance();

        if !infobus_manager.is_door_open() {
            player.send(&CANNOT_ENTER_BUS::new(
                "The Infobus is closed, there is no event right now. Please check back later.",
            ));
            return Ok(());
        }

        let current_item = room_user.get_current_item();

        if let Some(current_item) = &current_item {
            if current_item.get_definition().get_sprite() == "queue_tile2" {
                if let Some(next_queue_tile) =
                    infobus_manager.get_next_queue_tile(current_item.get_position())
                {
                    room_user.walk_to(next_queue_tile.get_x(), next_queue_tile.get_y());
                }

                return Ok(());
            }
        }

        room_user.walk_to(
            infobus_manager.get_queue_start_x(),
            infobus_manager.get_queue_start_y(),
        );

        Ok(())
    }
}
