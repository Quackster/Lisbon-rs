//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.UPDATE_PLAY_LIST`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::messages::outgoing::songs::song_playlist::SONG_PLAYLIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct UPDATE_PLAY_LIST;

impl MessageEvent for UPDATE_PLAY_LIST {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if !room.is_owner(player.get_details().get_id()) && !player.has_fuse(&Fuseright::AnyRoomController) {
            return Ok(());
        }

        let Some(sound_machine) = room.get_item_manager().get_sound_machine() else {
            return Ok(());
        };

        let amount = reader.read_int();

        if amount >= 6 {
            return Ok(());
        }

        // We don't want a user to get kicked when making cool beats.
        room_user.reset_room_timer();

        SongMachineDao::clear_playlist(sound_machine.get_id());

        for slot_id in 0..amount {
            let song_id = reader.read_int();
            SongMachineDao::add_playlist(sound_machine.get_id(), song_id, slot_id);
        }

        room.send(&SONG_PLAYLIST::new(
            SongMachineDao::get_song_playlist(sound_machine.get_id()),
        ));

        Ok(())
    }
}
