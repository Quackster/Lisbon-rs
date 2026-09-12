//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.pool.SWIMSUIT`.
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::public_rooms::pool_handler::PoolHandler;
use crate::messages::outgoing::rooms::user::user_objects::USER_OBJECTS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct SWIMSUIT;

impl MessageEvent for SWIMSUIT {
    /// `setPoolFigure` needs `&mut` details, so the logic lives in
    /// `handle_mut`; the dispatcher always routes through `handle_mut`.
    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player.get_room_user().and_then(|room_user| room_user.get_room()) else {
            return Ok(());
        };

        if room.get_data().get_model() != "pool_a" && room.get_data().get_model() != "md_a" {
            return Ok(());
        }

        let swimsuit = StringUtil::filter_input(reader.contents().unwrap_or_default().as_str(), true);

        player.get_details_mut().set_pool_figure(&swimsuit);

        room.send(&USER_OBJECTS::from_entity(player));

        let details = player.get_details();
        PlayerDao::save_details(
            details.get_id(),
            details.get_figure(),
            details.get_pool_figure(),
            details.get_sex(),
        );

        PoolHandler::exit_booth(player);

        Ok(())
    }

    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Ok(())
    }
}
