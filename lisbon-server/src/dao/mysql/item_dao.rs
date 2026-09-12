//! Mirrors `net.h4bbo.lisbon.dao.mysql.ItemDao`.

use std::collections::HashMap;

use sqlx::mysql::MySqlRow;
use sqlx::Row;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::item::base::item_definition::ItemDefinition;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct ItemDao;

impl ItemDao {
    /// Mirrors `saveTradeBanExpire(int, long)`.
    pub fn save_trade_ban_expire(user_id: i32, trade_ban_expiration: i64) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET trade_ban_expiration = {trade_ban_expiration} WHERE id = {user_id}"
        ));
    }

    /// Mirrors `newItem(Item)` (the Java `getGeneratedKeys()` write-back is
    /// applied through the `execute_insert` generated key).
    pub fn new_item(item: &mut Item) {
        match Storage::get_storage().execute_insert(&format!(
            "INSERT INTO items (user_id, definition_id, custom_data) VALUES ({}, {}, '{}')",
            item.get_owner_id(),
            item.get_definition().get_id(),
            escape(item.get_custom_data())
        )) {
            Some(id) => item.set_id(id as i32),
            None => {}
        }
    }

    /// Mirrors `redeemCreditItem(int, int, int)`.
    pub fn redeem_credit_item(amount: i32, item_id: i32, user_id: i32) -> i32 {
        let storage = Storage::get_storage();

        let fetched = storage.runtime().block_on(async {
            // The delete and the credit increase share one atomic
            // transaction (Java disables autocommit for this).
            let mut transaction = match storage.pool().begin().await {
                Ok(transaction) => transaction,
                Err(err) => {
                    Storage::log_error(err.to_string());
                    return None;
                }
            };

            if let Err(err) =
                sqlx::query(&format!("DELETE FROM items WHERE id = {item_id}"))
                    .execute(&mut *transaction)
                    .await
            {
                Storage::log_error(err.to_string());
                let _ = transaction.rollback().await;
                return None;
            }

            if let Err(err) = sqlx::query(&format!(
                "UPDATE users SET credits = credits + {amount} WHERE id = {user_id}"
            ))
            .execute(&mut *transaction)
            .await
            {
                Storage::log_error(err.to_string());
                let _ = transaction.rollback().await;
                return None;
            }

            // Java fetches the increased amount before the commit.
            match sqlx::query(&format!(
                "SELECT credits FROM users WHERE id = {user_id}"
            ))
            .fetch_all(&mut *transaction)
            .await
            {
                Ok(rows) => {
                    if let Err(err) = transaction.commit().await {
                        Storage::log_error(err.to_string());
                        return None;
                    }
                    rows.into_iter()
                        .next()
                        .and_then(|row| row.try_get::<i32, _>("credits").ok())
                }
                Err(err) => {
                    Storage::log_error(err.to_string());
                    let _ = transaction.rollback().await;
                    None
                }
            }
        });

        fetched.unwrap_or(-1)
    }

    /// Mirrors `updateItems(List<Item>)`.
    pub fn update_items(items: &[Item]) {
        for item in items {
            Self::update_item(item);
        }
    }

    /// Mirrors `updateItem(Item)`.
    pub fn update_item(item: &Item) {
        let position = item.get_position();

        Storage::get_storage().execute(&format!(
            "UPDATE items SET user_id = {}, room_id = {}, definition_id = {}, x = {}, y = {}, z = {}, rotation = {}, wall_position = '{}', custom_data = '{}', order_id = {}, is_hidden = {} WHERE id = {}",
            item.get_owner_id(),
            item.get_room_id(),
            item.get_definition().get_id(),
            position.get_x(),
            position.get_y(),
            position.get_z(),
            position.get_rotation(),
            escape(item.get_wall_position()),
            escape(item.get_custom_data()),
            item.get_order_id(),
            if item.is_hidden() { 1 } else { 0 },
            item.get_id()
        ));
    }

    /// Mirrors `getItemDefinitions()`.
    pub fn get_item_definitions() -> HashMap<i32, ItemDefinition> {
        let mut definitions = HashMap::new();

        for row in Storage::get_storage().query_all("SELECT * FROM items_definitions") {
            if let (Some(id), Some(sprite), Some(name), Some(description), Some(behaviour), Some(interactor), Some(top_height), Some(length), Some(width), Some(colour), Some(drink_ids), Some(is_recyclable)) = (
                row.i32("id"),
                row.str("sprite"),
                row.str("name"),
                row.str("description"),
                row.str("behaviour"),
                row.str("interactor"),
                row.f64("top_height"),
                row.i32("length"),
                row.i32("width"),
                row.str("colour"),
                row.str("drink_ids"),
                row.bool("is_recyclable"),
            ) {
                definitions.insert(
                    id,
                    ItemDefinition::with_data(
                        id,
                        &sprite,
                        &name,
                        &description,
                        &behaviour,
                        &interactor,
                        top_height,
                        length,
                        width,
                        &colour,
                        &drink_ids,
                        is_recyclable,
                    ),
                );
            }
        }

        definitions
    }

    /// Mirrors `getUserItemsByDefinition(int, ItemDefinition)`.
    pub fn get_user_items_by_definition(user_id: i32, definition: &ItemDefinition) -> Vec<Item> {
        let mut items = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT * FROM items WHERE user_id = {user_id} AND definition_id = {}",
            definition.get_id()
        )) {
            let mut item = Item::new();
            Self::fill(&mut item, &row);
            items.push(item);
        }

        items
    }

    /// Mirrors `getItem(int)`.
    pub fn get_item(item_id: i32) -> Option<Item> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM items WHERE id = {item_id}"))
        {
            let mut item = Item::new();
            Self::fill(&mut item, &row);
            return Some(item);
        }

        None
    }

    /// Mirrors `getInventory(int)`.
    pub fn get_inventory(user_id: i32) -> Vec<Item> {
        let mut items = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT * FROM items WHERE user_id = {user_id} AND room_id = 0 ORDER BY order_id ASC"
        )) {
            let mut item = Item::new();
            Self::fill(&mut item, &row);
            items.push(item);
        }

        items
    }

    /// Mirrors `getRoomItems(RoomData)`.
    pub fn get_room_items(room_data: &crate::game::room::room_data::RoomData) -> Vec<Item> {
        let mut items = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT * FROM items WHERE room_id = {}",
            room_data.get_id()
        )) {
            let mut item = Item::new();
            Self::fill(&mut item, &row);
            items.push(item);
        }

        items
    }

    /// Mirrors `deleteItems(List<Integer>)`.
    pub fn delete_items(items: &[i32]) {
        for item_id in items {
            Storage::get_storage().execute(&format!("DELETE FROM items WHERE id = {item_id}"));
        }
    }

    /// Mirrors the private `fill(Item, ResultSet)`.
    fn fill(item: &mut Item, row: &MySqlRow) {
        if let Some(id) = row.i32("id") {
            item.set_id(id);
        }
        if let Some(order_id) = row.i32("order_id") {
            item.set_order_id(order_id);
        }
        if let Some(user_id) = row.i32("user_id") {
            item.set_owner_id(user_id);
        }
        if let Some(room_id) = row.i32("room_id") {
            item.set_room_id(room_id);
        }
        if let Some(definition_id) = row.i32("definition_id") {
            item.set_definition_id(definition_id);
        }

        let x = row.i32("x").unwrap_or(0);
        let y = row.i32("y").unwrap_or(0);
        let z = row.f64("z").unwrap_or(0.0);
        let rotation = row.i32("rotation").unwrap_or(0);
        item.set_position(Position::with_rotations(x, y, z, rotation, rotation));

        if let Some(wall_position) = row.str("wall_position") {
            item.set_wall_position(&wall_position);
        }
        if let Some(custom_data) = row.str("custom_data") {
            item.set_custom_data(&custom_data);
        }
        if let Some(is_hidden) = row.bool("is_hidden") {
            item.set_hidden(is_hidden);
        }
    }
}
