//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.GET_SONG_INFO`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::songs::song_info::SONG_INFO;
use crate::messages::outgoing::songs::sound_packages::SOUND_PACKAGES;
use crate::messages::outgoing::songs::user_sound_packages::USER_SOUND_PACKAGES;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_SONG_INFO;

impl MessageEvent for GET_SONG_INFO {
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

        if let Some(inventory) = player.get_inventory() {
            player.send(&USER_SOUND_PACKAGES::new(inventory.get_soundsets()));
        }
        player.send(&SOUND_PACKAGES::new(SongMachineDao::get_tracks(sound_machine.get_id())));

        let song_list = SongMachineDao::get_song_list(sound_machine.get_id());

        if let Some(song) = song_list.first() {
            player.send(&SONG_INFO::new(song.clone()));
        }

        Ok(())
    }
}
