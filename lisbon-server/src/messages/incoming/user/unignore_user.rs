//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.UNIGNORE_USER`.
use crate::dao::mysql::player_dao::PlayerDao;
use crate::dao::mysql::users_mutes_dao::UsersMutesDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::user::ignore_user_result::IGNORE_USER_RESULT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct UNIGNORE_USER;

impl MessageEvent for UNIGNORE_USER {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
        let username = reader.read_string();

        if !player.get_ignored_list().contains(&username) {
            return Ok(());
        }

        let user_id = PlayerDao::get_id(&username);
        UsersMutesDao::remove_muted(player.get_details().get_id(), user_id);

        player.remove_ignored_name(&username);
        player.send(&IGNORE_USER_RESULT::new(3));

        Ok(())
    }
}
