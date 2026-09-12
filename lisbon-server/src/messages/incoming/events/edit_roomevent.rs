//! Mirrors `net.h4bbo.lisbon.messages.incoming.events.EDIT_ROOMEVENT`.
use crate::dao::mysql::events_dao::EventsDao;
use crate::game::events::events_manager::EventsManager;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::events::roomevent_info::ROOMEEVENT_INFO;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct EDIT_ROOMEVENT;

impl MessageEvent for EDIT_ROOMEVENT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player.get_room_user().and_then(|room_user| room_user.get_room()) else {
            return Ok(());
        };

        if !room.is_owner(player.get_details().get_id()) {
            return Ok(());
        }

        let events_manager = EventsManager::get_instance();

        if !events_manager.has_event(room.get_id()) {
            return Ok(());
        }

        let Some(mut event) = events_manager.get_event_by_room_id(room.get_id()) else {
            return Ok(());
        };

        let category = reader.read_int();

        if category < 1 || category > 11 {
            return Ok(());
        }

        let name = StringUtil::filter_input(&reader.read_string(), true);
        let description = StringUtil::filter_input(&reader.read_string(), true);

        event.set_category_id(category);
        event.set_name(name);
        event.set_description(description);

        room.send(&ROOMEEVENT_INFO::new(Some(event.clone())));
        EventsDao::save(&event);

        Ok(())
    }
}
