//! Mirrors `net.h4bbo.lisbon.messages.incoming.jukebox.GET_USER_SONG_DISCS`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::player::player::Player;
use crate::messages::outgoing::jukebox::user_song_disks::USER_SONG_DISKS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_USER_SONG_DISCS;

impl MessageEvent for GET_USER_SONG_DISCS {
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

        let Some(inventory) = player.get_inventory() else {
            return Ok(());
        };

        let mut user_disks: Vec<(Item, i32)> = Vec::new();

        for item in inventory.get_items() {
            if item.is_hidden() {
                continue;
            }

            if item.has_behaviour(ItemBehaviour::SongDisk) {
                let id = item.get_id();
                user_disks.push((item, id));
            }
        }

        player.send(&USER_SONG_DISKS::new(user_disks));

        Ok(())
    }
}
