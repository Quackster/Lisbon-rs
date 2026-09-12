//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.GET_IGNORE_LIST`.
use crate::dao::mysql::player_dao::PlayerDao;
use crate::dao::mysql::users_mutes_dao::UsersMutesDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::user::ignored_list::IGNORED_LIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_IGNORE_LIST;

impl MessageEvent for GET_IGNORE_LIST {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if !player.get_ignored_list().is_empty() {
            return Ok(());
        }

        let ignore_list = UsersMutesDao::get_muted_users(player.get_details().get_id());

        for user_id in ignore_list {
            if let Some(name) = PlayerDao::get_name(user_id) {
                player.add_ignored_name(&name);
            }
        }

        player.send(&IGNORED_LIST::new(
            player.get_ignored_list().iter().cloned().collect(),
        ));

        Ok(())
    }
}
