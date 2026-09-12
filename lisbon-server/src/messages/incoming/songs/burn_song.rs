//! Mirrors `net.h4bbo.lisbon.messages.incoming.songs.BURN_SONG`.
use chrono::Datelike;

use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::dao::mysql::item_dao::ItemDao;
use crate::dao::mysql::jukebox_dao::JukeboxDao;
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::item::Item;
use crate::game::item::item_manager::ItemManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::user::currencies::credit_balance::CREDIT_BALANCE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct BURN_SONG;

impl MessageEvent for BURN_SONG {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
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

        if player.get_details().get_credits() <= 0 {
            return Ok(());
        }

        let song_id = reader.read_int();
        let Some(_song) = SongMachineDao::get_song(song_id) else {
            return Ok(());
        };

        let now = chrono::Local::now();

        let mut item = Item::new();
        item.set_owner_id(player.get_details().get_id());
        item.set_definition_id(
            ItemManager::get_instance()
                .get_definition_by_sprite("song_disk")
                .map(|definition| definition.get_id())
                .unwrap_or(0),
        );
        item.set_custom_data(&format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            player.get_details().get_name(),
            now.day(),
            now.month0(),
            now.year(),
            _song.get_length(),
            _song.get_title()
        ));

        ItemDao::new_item(&mut item);

        if let Some(inventory) = player.get_inventory() {
            inventory.add_item(&item);
            inventory.view(player, "new");
        }

        JukeboxDao::add_disk(item.get_id() as i64, 0, song_id);
        JukeboxDao::set_burned(song_id, true);

        CurrencyDao::decrease_credits(player.get_details(), 1);
        player.send(&CREDIT_BALANCE::new(player.get_details().get_credits()));

        Ok(())
    }
}
