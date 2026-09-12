//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.SAVE_SONG`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::messages::outgoing::songs::song_new::SONG_NEW;
use crate::messages::outgoing::songs::sound_packages::SOUND_PACKAGES;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct SAVE_SONG;

impl MessageEvent for SAVE_SONG {
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

        let title = format!("Trax item song {}", sound_machine.get_id());
        let data = StringUtil::filter_input(&reader.read_string(), true);

        SongMachineDao::delete_song(sound_machine.get_id());
        SongMachineDao::add_song(
            sound_machine.get_id(),
            player.get_details().get_id(),
            sound_machine.get_id(),
            "",
            Self::calculate_song_length(&data),
            &data,
        );

        player.send(&SOUND_PACKAGES::new(SongMachineDao::get_tracks(sound_machine.get_id())));
        player.send(&SONG_NEW::new(sound_machine.get_id(), &title));

        Ok(())
    }
}

impl SAVE_SONG {
    /// Mirrors `calculateSongLength(String)`.
    pub(crate) fn calculate_song_length(song: &str) -> i32 {
        if song.is_empty() {
            return 0;
        }

        let song_data = &song[..song.len() - 1];
        let song_data = song_data.replace(":4:", "|");
        let song_data = song_data.replace(":3:", "|");
        let song_data = song_data.replace(":2:", "|");
        let song_data = song_data.replace("1:", "");

        let data: Vec<&str> = song_data.split('|').collect();

        if data.len() < 4 {
            return 0;
        }

        let tracks = [data[0], data[1], data[2], data[3]];
        let mut song_length = 0;

        for track in tracks {
            let samples: Vec<&str> = track.split(';').collect();
            let mut track_length = 0;

            for sample in samples {
                let sample_seconds: i32 = match sample
                    .split(',')
                    .nth(1)
                    .and_then(|value| value.parse().ok())
                {
                    Some(value) => value,
                    None => return 0,
                };
                track_length += sample_seconds * 2;
            }

            if track_length > song_length {
                song_length = track_length;
            }
        }

        song_length
    }
}
