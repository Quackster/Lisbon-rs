//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.SAVE_SONG_NEW`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::messages::incoming::songs::save_song::SAVE_SONG;
use crate::messages::outgoing::songs::song_new::SONG_NEW;
use crate::messages::outgoing::songs::sound_packages::SOUND_PACKAGES;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct SAVE_SONG_NEW;

impl MessageEvent for SAVE_SONG_NEW {
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

        let title = StringUtil::filter_input(&reader.read_string(), true);
        let data = StringUtil::filter_input(&reader.read_string(), true);

        SongMachineDao::add_song_new(
            player.get_details().get_id(),
            sound_machine.get_id(),
            &title,
            SAVE_SONG::calculate_song_length(&data),
            &data,
        );

        player.send(&SOUND_PACKAGES::new(SongMachineDao::get_tracks(sound_machine.get_id())));
        player.send(&SONG_NEW::new(sound_machine.get_id(), &title));

        Ok(())
    }
}
