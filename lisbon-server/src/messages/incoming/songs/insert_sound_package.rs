//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.INSERT_SOUND_PACKAGE`.
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::messages::outgoing::songs::sound_packages::SOUND_PACKAGES;
use crate::messages::outgoing::songs::user_sound_packages::USER_SOUND_PACKAGES;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct INSERT_SOUND_PACKAGE;

impl MessageEvent for INSERT_SOUND_PACKAGE {
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

        let tracks = SongMachineDao::get_tracks(sound_machine.get_id());

        let sound_set_id = reader.read_int();
        let mut slot_id = 1;

        while tracks.contains_key(&slot_id) {
            slot_id += 1;
        }

        if tracks.contains_key(&slot_id) || slot_id >= 5 || slot_id < 0 {
            return Ok(());
        }

        let mut track_id = -1;
        let mut track_item = None;

        if let Some(inventory) = player.get_inventory() {
            for item in inventory.get_items() {
                if item.has_behaviour(ItemBehaviour::SoundMachineSampleSet) && !item.is_hidden() {
                    let song_id: i32 = item
                        .get_definition()
                        .get_sprite()
                        .split("_")
                        .nth(2)
                        .and_then(|value| value.parse().ok())
                        .unwrap_or(-1);

                    if song_id == sound_set_id {
                        track_id = song_id;
                        track_item = Some(item);
                        break;
                    }
                }
            }
        }

        if track_id == -1 {
            return Ok(());
        }

        let Some(mut track_item) = track_item else {
            return Ok(());
        };

        track_item.set_hidden(true);
        track_item.save();

        let inventory = player.get_inventory().unwrap();
        inventory.view(player, "new");
        SongMachineDao::add_track(sound_machine.get_id(), sound_set_id, slot_id);

        player.send(&SOUND_PACKAGES::new(SongMachineDao::get_tracks(sound_machine.get_id())));
        player.send(&USER_SOUND_PACKAGES::new(inventory.get_soundsets()));

        Ok(())
    }
}
