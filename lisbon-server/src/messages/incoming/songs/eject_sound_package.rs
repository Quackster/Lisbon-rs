//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.EJECT_SOUND_PACKAGE`.
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
pub struct EJECT_SOUND_PACKAGE;

impl MessageEvent for EJECT_SOUND_PACKAGE {
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

        let slot_id = reader.read_int();
        let tracks = SongMachineDao::get_tracks(sound_machine.get_id());

        if !tracks.contains_key(&slot_id) {
            return Ok(());
        }

        SongMachineDao::remove_track(sound_machine.get_id(), slot_id);

        let song_sound_id = tracks[&slot_id];
        let mut soundset = None;

        if let Some(inventory) = player.get_inventory() {
            for item in inventory.get_items() {
                if item.is_hidden() {
                    continue;
                }

                if !item.has_behaviour(ItemBehaviour::SoundMachineSampleSet) {
                    continue;
                }

                let soundset_id: i32 = item
                    .get_definition()
                    .get_sprite()
                    .replace("sound_set_", "")
                    .parse()
                    .unwrap_or(-1);

                if soundset_id == song_sound_id {
                    soundset = Some(item);
                    break;
                }
            }
        }

        let Some(mut soundset) = soundset else {
            return Ok(());
        };

        soundset.set_hidden(false);
        let inventory = player.get_inventory().unwrap();
        inventory.add_item(&soundset); // Re-add at start.
        soundset.save();

        inventory.view(player, "new");

        player.send(&SOUND_PACKAGES::new(SongMachineDao::get_tracks(sound_machine.get_id())));
        player.send(&USER_SOUND_PACKAGES::new(inventory.get_soundsets()));

        Ok(())
    }
}
