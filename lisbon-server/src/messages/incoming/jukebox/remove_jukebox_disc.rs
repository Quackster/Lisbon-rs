//! Mirrors `net.h4bbo.lisbon.messages.incoming.jukebox.REMOVE_JUKEBOX_DISC`.
use crate::dao::mysql::jukebox_dao::JukeboxDao;
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::song::jukebox::jukebox_manager::JukeboxManager;
use crate::game::song::song_playlist::SongPlaylist;
use crate::messages::incoming::jukebox::get_user_song_disks::GET_USER_SONG_DISCS;
use crate::messages::outgoing::jukebox::jukebox_disks::JUKEBOX_DISCS;
use crate::messages::outgoing::songs::song_playlist::SONG_PLAYLIST;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct REMOVE_JUKEBOX_DISC;

impl MessageEvent for REMOVE_JUKEBOX_DISC {
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

        let slot_id = _reader.read_int();
        let Some(burned_disk) =
            JukeboxDao::get_disk(_sound_machine.get_id() as i64, slot_id)
        else {
            return Ok(());
        };

        let Some(inventory) = player.get_inventory() else {
            return Ok(());
        };

        let Some(mut song_disk) = inventory
            .get_items()
            .into_iter()
            .find(|item| burned_disk.get_item_id() == item.get_id() as i64 && item.is_hidden())
        else {
            return Ok(());
        };

        song_disk.set_hidden(false);
        inventory.add_item(&song_disk);
        song_disk.save();

        SongMachineDao::remove_playlist_song(
            burned_disk.get_song_id(),
            _sound_machine.get_id() as i64,
        );

        JukeboxDao::edit_disk(song_disk.get_id(), 0, 0);

        inventory.view(player, "new");
        let _ = GET_USER_SONG_DISCS.handle(player, _reader);

        let playlist: Vec<SongPlaylist> = SongMachineDao::get_song_playlist(_sound_machine.get_id());
        room.send(&SONG_PLAYLIST::new(playlist));

        room.send(&JUKEBOX_DISCS::new(
            JukeboxManager::get_instance()
                .get_disks(_sound_machine.get_id() as i64)
                .into_iter()
                .collect(),
        ));

        Ok(())
    }
}
