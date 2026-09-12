//! Mirrors `net.h4bbo.lisbon.messages.incoming.recycler.GET_FURNI_RECYCLER_STATUS`.
use crate::game::entity::entity::Entity;
use crate::dao::mysql::recycler_dao::RecyclerDao;
use crate::game::player::player::Player;
use crate::game::recycler::recycler_manager::RecyclerManager;
use crate::messages::outgoing::recycler::recycler_status::RECYCLER_STATUS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_FURNI_RECYCLER_STATUS;

impl MessageEvent for GET_FURNI_RECYCLER_STATUS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        player.send(&RECYCLER_STATUS::new(
            RecyclerManager::get_instance().is_recycler_enabled(),
            RecyclerDao::get_session(player.get_details().get_id()),
        ));

        Ok(())
    }
}
