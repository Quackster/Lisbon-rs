//! Mirrors `net.h4bbo.lisbon.messages.incoming.jukebox.RESET_JUKEBOX`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::messages::outgoing::songs::song_playlist::SONG_PLAYLIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct RESET_JUKEBOX;

impl MessageEvent for RESET_JUKEBOX {
    /// Mirrors `handle(Player, NettyRequest)`.
    #[allow(unreachable_code)]
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let Some(_sound_machine) = room.get_item_manager().get_sound_machine() else {
            return Ok(());
        };

        if !room.has_rights(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return Ok(());
        }

        SongMachineDao::clear_playlist(_sound_machine.get_id());

        room.send(&SONG_PLAYLIST::new(Vec::new()));

        Ok(())
    }
}
