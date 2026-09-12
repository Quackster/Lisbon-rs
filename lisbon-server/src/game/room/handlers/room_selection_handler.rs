//! Mirrors `net.h4bbo.lisbon.game.room.handlers.RoomSelectionHandler`.
use crate::dao::mysql::item_dao::ItemDao;
use crate::dao::mysql::navigator_dao::NavigatorDao;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::dao::mysql::room_dao::RoomDao;
use crate::game::item::item::Item;
use crate::game::item::item_manager::ItemManager;

pub struct RoomSelectionHandler;

impl RoomSelectionHandler {
    /// Mirrors `selectRoom(int, int)` (the Java `SQLException` is dropped;
    // the Java `getDefinitionBySprite` NPE is an early `false` return
    // here).
    pub fn select_room(user_id: i32, room_type: i32) -> bool {
        let Some(player_details) = PlayerDao::get_details(user_id) else {
            return false;
        };

        if room_type < 0 || room_type > 5 {
            return false;
        }

        let (stool_x, stool_y, stool_rotation, floor, wallpaper) = match room_type {
            0 => (1, 6, 2, 601, 1501),
            1 => (3, 6, 4, 0, 607),
            2 => (2, 2, 4, 301, 1901),
            3 => (1, 2, 2, 110, 1801),
            4 => (3, 6, 0, 104, 503),
            5 => (3, 6, 0, 107, 804),
            _ => unreachable!(),
        };

        let room_id = NavigatorDao::create_room(
            user_id,
            &format!("{}'s Room", player_details.get_name()),
            "model_s",
            true,
            0,
        );

        let Some(mut room) = RoomDao::get_room_by_id(room_id) else {
            return false;
        };

        room.get_data_mut().set_wallpaper(wallpaper);
        room.get_data_mut().set_floor(floor);
        RoomDao::save_decorations(&room);
        room.get_data_mut()
            .set_description(&format!("{} has entered the building", player_details.get_name()));
        RoomDao::save(&room);

        let Some(stool_definition) =
            ItemManager::get_instance().get_definition_by_sprite(&format!("noob_stool{}", room_type + 1))
        else {
            return false;
        };

        let mut item = Item::new();
        item.set_definition_id(stool_definition.get_id());
        item.set_owner_id(user_id);
        item.get_position_mut().set_x(stool_x);
        item.get_position_mut().set_y(stool_y);
        item.get_position_mut().set_rotation(stool_rotation);
        item.set_room_id(room_id);
        ItemDao::new_item(&mut item);
        ItemDao::update_item(&item);

        let Some(table_definition) =
            ItemManager::get_instance().get_definition_by_sprite(&format!("noob_table{}", room_type + 1))
        else {
            return false;
        };

        let mut table = Item::new();
        table.set_definition_id(table_definition.get_id());
        table.set_owner_id(user_id);
        ItemDao::new_item(&mut table);

        let Some(window_definition) =
            ItemManager::get_instance().get_definition_by_sprite("noob_window_double")
        else {
            return false;
        };

        let mut window = Item::new();
        window.set_wall_position(":w=3,0 l=13,71 r");
        window.set_definition_id(window_definition.get_id());
        window.set_owner_id(user_id);
        window.set_room_id(room_id);
        ItemDao::new_item(&mut window);
        ItemDao::update_item(&window);

        PlayerDao::save_selected_room(user_id, room_id);
        true
    }
}
