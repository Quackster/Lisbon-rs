//! Mirrors `net.h4bbo.lisbon.dao.mysql.PetDao`.

use rand::Rng;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::pets::pet_details::PetDetails;
use crate::util::date_util::DateUtil;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct PetDao;

impl PetDao {
    /// Mirrors `createPet(long, String, String, int, String)`.
    pub fn create_pet(database_id: i64, name: &str, type_: &str, race: i32, colour: &str) {
        let now = DateUtil::get_current_time_seconds() as i64;
        let mut rng = rand::thread_rng();

        let nature_positive = rng.gen_range(0..7);
        let nature_negative = rng.gen_range(0..7);
        let name = escape(name);
        let type_ = escape(type_);
        let colour = escape(colour);

        Storage::get_storage().execute(&format!(
            "INSERT INTO items_pets (item_id, name, type, race, colour, nature_positive, nature_negative, born, last_kip, last_eat, last_drink, last_playtoy, last_playuser) VALUES ({database_id}, '{name}', '{type_}', {race}, '{colour}', {nature_positive}, {nature_negative}, {now}, {now}, {now}, {now}, {now}, {now})"
        ));
    }

    /// Mirrors `getPetDetails(int)`.
    pub fn get_pet_details(item_id: i32) -> Option<PetDetails> {
        let mut pet_details = None;

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM items_pets WHERE item_id = {item_id}"))
        {
            if let (Some(id), Some(item_id), Some(name), Some(pet_type), Some(race), Some(colour), Some(nature_positive), Some(nature_negative), Some(friendship), Some(born), Some(last_kip), Some(last_eat), Some(last_drink), Some(last_playtoy), Some(last_playuser), Some(x), Some(y), Some(rotation)) = (
                row.i32("id"),
                row.i32("item_id"),
                row.str("name"),
                row.str("type"),
                row.str("race"),
                row.str("colour"),
                row.i32("nature_positive"),
                row.i32("nature_negative"),
                row.f64("friendship").map(|v| v as f32),
                row.i64("born"),
                row.i64("last_kip"),
                row.i64("last_eat"),
                row.i64("last_drink"),
                row.i64("last_playtoy"),
                row.i64("last_playuser"),
                row.i32("x"),
                row.i32("y"),
                row.i32("rotation"),
            ) {
                pet_details = Some(PetDetails::new(
                    id,
                    item_id,
                    &name,
                    &pet_type,
                    &race,
                    &colour,
                    nature_positive,
                    nature_negative,
                    friendship,
                    born,
                    last_kip,
                    last_eat,
                    last_drink,
                    last_playtoy,
                    last_playuser,
                    x,
                    y,
                    rotation,
                ));
            }
        }

        pet_details
    }

    /// Mirrors `saveCoordinates(int, int, int, int)`.
    /// Mirrors `saveDetails(int, PetDetails)`.
    pub fn save_details(id: i32, pet_details: &PetDetails) {
        Storage::get_storage().execute(&format!(
            "UPDATE items_pets SET last_kip = {}, last_eat = {}, last_drink = {}, last_playtoy = {}, last_playuser = {} WHERE id = {}",
            pet_details.get_last_kip(),
            pet_details.get_last_eat(),
            pet_details.get_last_drink(),
            pet_details.get_last_play_toy(),
            pet_details.get_last_play_user(),
            id
        ));
    }

    pub fn save_coordinates(id: i32, x: i32, y: i32, rotation: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE items_pets SET x = {x}, y = {y}, rotation = {rotation} WHERE id = {id}"
        ));
    }
}
