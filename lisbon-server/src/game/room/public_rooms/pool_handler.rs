//! Mirrors `net.h4bbo.lisbon.game.room.public_rooms.PoolHandler`.
use crate::game::entity::entity::Entity;
use crate::game::item::item::Item;
use crate::game::player::player::Player;
use crate::game::room::room::Room;

pub struct PoolHandler;

impl PoolHandler {
    /// Mirrors `setupRedirections`.
    pub fn setup_redirections(room: &Room, item: &Item) {
        if item.get_definition().get_sprite() == "poolBooth" {
            let mapping = room.get_mapping();

            if item.get_position().get_x() == 17 && item.get_position().get_y() == 11 {
        if let Some(mut tile) = mapping.lock().get_tile(room, 18, 11) {
                    tile.set_highest_item(item);
                }
            }

            if item.get_position().get_x() == 17 && item.get_position().get_y() == 9 {
        if let Some(mut tile) = mapping.lock().get_tile(room, 18, 9) {
                    tile.set_highest_item(item);
                }
            }

            if item.get_position().get_x() == 8 && item.get_position().get_y() == 1 {
        if let Some(mut tile) = mapping.lock().get_tile(room, 8, 0) {
                    tile.set_highest_item(item);
                }
            }

            if item.get_position().get_x() == 9 && item.get_position().get_y() == 1 {
        if let Some(mut tile) = mapping.lock().get_tile(room, 9, 0) {
                    tile.set_highest_item(item);
                }
            }
        }
    }

    /// Mirrors `exitBooth(Player)`.
    pub fn exit_booth(player: &Player) {
        let Some(mut item) = player.get_room_user().and_then(|room_user| room_user.get_current_item()) else {
            return;
        };
        let Some(room) = player.get_room_user().and_then(|room_user| room_user.get_room()) else {
            return;
        };

        if item.get_definition().get_sprite() != "poolBooth" {
            return;
        }

        if room.get_data().get_model() != "pool_a" && room.get_data().get_model() != "md_a" {
            return;
        }

        item.show_program(Some("open"));
        player.get_room_user().map(|room_user| room_user.set_walking_allowed(true));

        if room.get_data().get_model() == "pool_a" {
            let y = player.get_room_user().map(|room_user| room_user.get_position().get_y()).unwrap_or(0);

            if y == 11 {
                player.get_room_user().map(|room_user| room_user.walk_to(19, 11));
            }

            if y == 9 {
                player.get_room_user().map(|room_user| room_user.walk_to(19, 9));
            }
        }

        if room.get_data().get_model() == "md_a" {
            let x = player.get_room_user().map(|room_user| room_user.get_position().get_x()).unwrap_or(0);

            if x == 8 {
                player.get_room_user().map(|room_user| room_user.walk_to(8, 2));
            }

            if x == 9 {
                player.get_room_user().map(|room_user| room_user.walk_to(9, 2));
            }
        }
    }
}
