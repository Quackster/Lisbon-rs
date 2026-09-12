//! Mirrors `net.h4bbo.lisbon.messages.incoming.jukebox.ADD_JUKEBOX_DISC`.
use crate::dao::mysql::jukebox_dao::JukeboxDao;
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::song::jukebox::jukebox_manager::JukeboxManager;
use crate::messages::incoming::jukebox::get_user_song_disks::GET_USER_SONG_DISCS;
use crate::messages::outgoing::jukebox::jukebox_disks::JUKEBOX_DISCS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct ADD_JUKEBOX_DISC;

impl MessageEvent for ADD_JUKEBOX_DISC {
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

        let item_id = _reader.read_int();
        let slot_id = _reader.read_int();

        let Some(inventory) = player.get_inventory() else {
            return Ok(());
        };

        let Some(mut song_disk) = inventory
            .get_items()
            .into_iter()
            .find(|item| item.get_id() == item_id && !item.is_hidden())
        else {
            return Ok(());
        };

        song_disk.set_hidden(true);
        song_disk.save();

        inventory.view(player, "new");

        let song_id = JukeboxDao::get_song_id_by_item(song_disk.get_id() as i64);
        let Some(_song) = SongMachineDao::get_song(song_id) else {
            return Ok(());
        };

        if slot_id < 1 || slot_id > 10 {
            return Ok(());
        }

        JukeboxDao::edit_disk(
            song_disk.get_id(),
            _sound_machine.get_id(),
            slot_id,
        );

        room.send(&JUKEBOX_DISCS::new(
            JukeboxManager::get_instance()
                .get_disks(_sound_machine.get_id() as i64)
                .into_iter()
                .collect(),
        ));

        let _ = GET_USER_SONG_DISCS.handle(player, _reader);

        Ok(())
    }
}
