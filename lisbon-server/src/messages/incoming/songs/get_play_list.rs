//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.GET_PLAY_LIST`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::messages::outgoing::songs::song_playlist::SONG_PLAYLIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_PLAY_LIST;

impl MessageEvent for GET_PLAY_LIST {
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

        if sound_machine.has_behaviour(ItemBehaviour::SoundMachine)
            || sound_machine.has_behaviour(ItemBehaviour::Jukebox)
        {
            player.send(&SONG_PLAYLIST::new(
                SongMachineDao::get_song_playlist(sound_machine.get_id()),
            ));
        }

        Ok(())
    }
}
