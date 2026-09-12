//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.DELETE_SONG`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::messages::outgoing::songs::song_playlist::SONG_PLAYLIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct DELETE_SONG;

impl MessageEvent for DELETE_SONG {
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

        // We don't want a user to get kicked when making cool beats.
        room_user.reset_room_timer();

        let song_id = reader.read_int();
        let Some(song) = SongMachineDao::get_song(song_id) else {
            return Ok(());
        };

        if song.get_user_id() != player.get_details().get_id() {
            return Ok(());
        }

        SongMachineDao::clear_song(song_id);
        SongMachineDao::remove_playlist_song(song_id, sound_machine.get_id() as i64);

        if sound_machine.has_behaviour(ItemBehaviour::SoundMachine) {
            player.send(&SONG_PLAYLIST::new(
                SongMachineDao::get_song_playlist(sound_machine.get_id()),
            ));
        }

        Ok(())
    }
}
