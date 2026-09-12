//! Mirrors `net.h4bbo.lisbon.game.item.public_items.PublicItemParser`.
use std::collections::HashMap;

use rand::Rng;

use crate::dao::mysql::public_rooms_dao::PublicRoomsDao;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::item::item::Item;
use crate::game::item::item_manager::ItemManager;
use crate::game::pathfinder::position::Position;

pub struct PublicItemParser;

impl PublicItemParser {
    /// Mirrors `getPublicItems(int, String)`.
    pub fn get_public_items(room_id: i32, model_id: &str) -> Vec<Item> {
        let mut random_public_id: Vec<String> = Vec::new();
        let mut item_trigger_map: HashMap<&str, InteractionType> = HashMap::new();

        item_trigger_map.insert("poolEnter", InteractionType::PoolEnter);
        item_trigger_map.insert("poolExit", InteractionType::PoolExit);
        item_trigger_map.insert("poolLift", InteractionType::PoolLift);
        item_trigger_map.insert("poolBooth", InteractionType::PoolBooth);
        item_trigger_map.insert("queue_tile2", InteractionType::QueueTile);
        item_trigger_map.insert("s_queue_tile2", InteractionType::QueueTile);
        item_trigger_map.insert("gamehall_chair_wood", InteractionType::GameTicTacToe);

        if model_id == "hallC" {
            item_trigger_map.insert("gamehall_chair_green", InteractionType::GameChess);
            item_trigger_map.insert("chess_king_chair", InteractionType::GameChess);
        }

        if model_id == "hallB" {
            item_trigger_map.insert("gamehall_chair_green", InteractionType::GameBattleships);
        }

        if model_id == "hallD" {
            item_trigger_map.insert("gamehall_chair_green", InteractionType::GamePoker);
        }

        item_trigger_map.insert("wsJoinQueue", InteractionType::WsJoinQueue);
        item_trigger_map.insert("wsQueueTile", InteractionType::WsQueueTile);
        item_trigger_map.insert("wsTileStart", InteractionType::WsTileStart);

        let mut item_id = 0;

        let mut items = Vec::new();

        for item_data in PublicRoomsDao::get_public_item_data(model_id) {
            let alphabet = "abcdefghijlmnopqrstuvwyz";

            let custom_id = loop {
                let temp = format!(
                    "{}{}",
                    alphabet
                        .chars()
                        .nth(rand::thread_rng().gen_range(0..alphabet.len()))
                        .unwrap(),
                    rand::thread_rng().gen_range(0..999)
                );

                if !random_public_id.contains(&temp) {
                    random_public_id.push(temp.clone());
                    break temp;
                }
            };

            let mut item = Item::new();
            item.set_id(item_id);
            item_id += 1;
            item.set_room_id(room_id);
            item.set_custom_data(&custom_id);
            item.get_definition_mut().set_sprite(item_data.get_sprite());
            item.get_definition_mut().set_top_height(item_data.get_top_height());
            item.get_definition_mut().set_length(item_data.get_length());
            item.get_definition_mut().set_width(item_data.get_width());
            item.set_current_program(item_data.get_current_program());

            if let Some(interaction_type) = item_trigger_map.get(item_data.get_sprite()) {
                item.get_definition_mut()
                    .set_interaction_type(Some(*interaction_type));
            }

            if !item_data.get_behaviour().is_empty() {
                for behaviour in item_data.get_behaviour().split(',') {
                    // Java's `valueOf` throws on an unknown name; unknown
                    // entries are skipped here.
                    if let Some(behaviour) =
                        ItemBehaviour::from_str(behaviour.to_uppercase().as_str())
                    {
                        item.get_definition_mut().add_behaviour(behaviour);
                    }
                }
            }

            if item.get_definition().get_interaction_type().is_none() {
                if item.get_definition().has_behaviour(ItemBehaviour::CanSitOnTop) {
                    item.get_definition_mut()
                        .set_interaction_type(Some(InteractionType::Chair));
                } else {
                    item.get_definition_mut()
                        .set_interaction_type(Some(InteractionType::Default));
                }
            }

            item.get_position_mut().set_x(item_data.get_x());
            item.get_position_mut().set_y(item_data.get_y());
            item.get_position_mut().set_z(item_data.get_z());
            item.get_position_mut().set_rotation(item_data.get_rotation());

            if !item.get_definition().get_sprite().contains("queue_tile2") {
                item.get_definition_mut()
                    .add_behaviour(ItemBehaviour::PublicSpaceObject);
            }

            if item.get_definition().has_behaviour(ItemBehaviour::PrivateFurniture) {
                // Port note: the Java `getDefinitionBySprite(...).getId()`
                // NPE is an early `0` here.
                let definition_id = ItemManager::get_instance()
                    .get_definition_by_sprite(item_data.get_sprite())
                    .map(|definition| definition.get_id())
                    .unwrap_or(0);
                item.set_definition_id(definition_id);
            }

            if item.get_definition().get_sprite() == "poolLift"
                || item.get_definition().get_sprite() == "poolBooth"
            {
                item.show_program(Some("open"));
            }

            if let Some(teleport_to) = item_data.get_teleport_to() {
                let x = teleport_to.first().and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
                let y = teleport_to.get(1).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
                let z = teleport_to.get(2).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
                let rotation = teleport_to.get(3).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);

                let mut position = Position::new(x, y, z as f64);
                position.set_body_rotation(rotation);
                position.set_head_rotation(rotation);

                item.set_teleport_to(Some(position));
            }

            if let Some(swim_to) = item_data.get_swim_to() {
                let x = swim_to.first().and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
                let y = swim_to.get(1).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
                let z = swim_to.get(2).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
                let rotation = swim_to.get(3).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);

                let mut position = Position::new(x, y, z as f64);
                position.set_body_rotation(rotation);
                position.set_head_rotation(rotation);

                item.set_swim_to(Some(position));
            }

            items.push(item);
        }

        items
    }
}

impl Default for PublicItemParser {
    fn default() -> Self {
        Self
    }
}
