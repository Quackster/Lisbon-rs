//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.DANCE`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::room::enums::status_type::StatusType;
use crate::messages::outgoing::rooms::user::user_statuses::USER_STATUSES;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct DANCE;

impl MessageEvent for DANCE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if room_user.contains_status(StatusType::Sit) || room_user.contains_status(StatusType::Lay) {
            return Ok(());
        }

        let content = reader.contents().unwrap_or_default();

        if StringUtil::is_null_or_empty(Some(&content)) {
            room_user.set_status(StatusType::Dance, "");
        } else {
            if !player.has_fuse(&Fuseright::UseClubDance) {
                return Ok(());
            }

            let dance_id = reader.read_int();
            let dance_id = dance_id.to_string();
            room_user.set_status(StatusType::Dance, &dance_id);
        }

        room_user.remove_status(StatusType::CarryDrink);
        room_user.remove_status(StatusType::CarryFood);

        if room_user.is_walking() {
            room_user.set_needs_update(true);
            return Ok(());
        }

        let refs: Vec<&(dyn Entity + Send)> = vec![player];
        room.send(&USER_STATUSES::new(refs));
        room_user.reset_room_timer();

        Ok(())
    }
}
