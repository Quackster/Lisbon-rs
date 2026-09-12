//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.GET_SONG_LIST`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::songs::song_list::SONG_LIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_SONG_LIST;

impl MessageEvent for GET_SONG_LIST {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let Some(sound_machine) = room.get_item_manager().get_sound_machine() else {
            return Ok(());
        };

        player.send(&SONG_LIST::new(SongMachineDao::get_song_list(sound_machine.get_id())));

        Ok(())
    }
}
