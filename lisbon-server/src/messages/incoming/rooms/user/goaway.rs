//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.GOAWAY`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::handlers::walkways::walkways_manager::WalkwaysManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GOAWAY;

impl MessageEvent for GOAWAY {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if !room_user.is_walking_allowed() {
            return Ok(());
        }

        if room.is_public_room() {
            if let Some(model) = room.get_model() {
                let door_position = model.get_door_location();

                if WalkwaysManager::get_instance()
                    .get_destination(&room, &door_position)
                    .is_some()
                {
                    return Ok(());
                }
            }
        }

        room_user.kick(true);

        Ok(())
    }
}
