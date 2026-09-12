//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.GET_USER_TAGS`.
use crate::dao::mysql::tag_dao::TagDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::rooms::user::tag_list::TAG_LIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_USER_TAGS;

impl MessageEvent for GET_USER_TAGS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(_room) = room_user.get_room() else {
            return Ok(());
        };

        let Some(p) = PlayerManager::get_instance().get_player_by_id(reader.read_int()) else {
            return Ok(());
        };
        let p = p.lock();

        player.send(&TAG_LIST::new(
            p.get_details().get_id(),
            TagDao::get_user_tags(p.get_details().get_id()),
        ));

        Ok(())
    }
}
