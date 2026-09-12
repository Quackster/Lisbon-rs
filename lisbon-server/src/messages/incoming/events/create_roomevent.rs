//! Mirrors `net.h4bbo.lisbon.messages.incoming.events.CREATE_ROOMEVENT`.
use crate::game::events::events_manager::EventsManager;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::events::roomevent_info::ROOMEEVENT_INFO;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct CREATE_ROOMEVENT;

impl MessageEvent for CREATE_ROOMEVENT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        if !EventsManager::get_instance().can_create_event(player) {
            return Ok(());
        }

        let category = reader.read_int();

        if category < 1 || category > 11 {
            return Ok(());
        }

        let name = StringUtil::filter_input(&reader.read_string(), true);
        let description = StringUtil::filter_input(&reader.read_string(), true);

        let event =
            EventsManager::get_instance().create_event(player, category, &name, &description);

        if let Some(room) = player.get_room_user().and_then(|room_user| room_user.get_room()) {
            room.send(&ROOMEEVENT_INFO::new(Some(event)));
        }

        Ok(())
    }
}
