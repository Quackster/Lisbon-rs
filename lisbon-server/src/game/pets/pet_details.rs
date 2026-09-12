//! Mirrors `net.h4bbo.lisbon.game.pets.PetDetails`.
// Port note: the Java class extends `PlayerDetails`; the Rust `PlayerDetails`
// base is not mirrored (composition is used instead).
#[derive(Clone, Debug)]
pub struct PetDetails {
    id: i32,
    item_id: i32,
    name: String,
    pet_type: String,
    race: String,
    colour: String,
    nature_positive: i32,
    nature_negative: i32,
    friendship: f32,
    born: i64,
    last_kip: i64,
    last_eat: i64,
    last_drink: i64,
    last_play_toy: i64,
    last_play_user: i64,
    x: i32,
    y: i32,
    rotation: i32,
}

impl PetDetails {
    /// Mirrors the 18-arg `PetDetails` constructor.
    pub fn new(
        id: i32,
        item_id: i32,
        name: &str,
        pet_type: &str,
        race: &str,
        colour: &str,
        nature_positive: i32,
        nature_negative: i32,
        friendship: f32,
        born: i64,
        last_kip: i64,
        last_eat: i64,
        last_drink: i64,
        last_play_toy: i64,
        last_play_user: i64,
        x: i32,
        y: i32,
        rotation: i32,
    ) -> Self {
        Self {
            id,
            item_id,
            name: name.to_string(),
            pet_type: pet_type.to_string(),
            race: race.to_string(),
            colour: colour.to_string(),
            nature_positive,
            nature_negative,
            friendship,
            born,
            last_kip,
            last_eat,
            last_drink,
            last_play_toy,
            last_play_user,
            x,
            y,
            rotation,
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getItemId()`.
    pub fn get_item_id(&self) -> i32 {
        self.item_id
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `getType()`.
    pub fn get_type(&self) -> &str {
        &self.pet_type
    }

    /// Mirrors `getRace()`.
    pub fn get_race(&self) -> &str {
        &self.race
    }

    /// Mirrors `getColour()`.
    pub fn get_colour(&self) -> &str {
        &self.colour
    }

    /// Mirrors `getNaturePositive()`.
    pub fn get_nature_positive(&self) -> i32 {
        self.nature_positive
    }

    /// Mirrors `getNatureNegative()`.
    pub fn get_nature_negative(&self) -> i32 {
        self.nature_negative
    }

    /// Mirrors `getFriendship()`.
    pub fn get_friendship(&self) -> f32 {
        self.friendship
    }

    /// Mirrors `getBorn()`.
    pub fn get_born(&self) -> i64 {
        self.born
    }

    /// Mirrors `getLastKip()`.
    pub fn get_last_kip(&self) -> i64 {
        self.last_kip
    }

    /// Mirrors `getLastEat()`.
    pub fn get_last_eat(&self) -> i64 {
        self.last_eat
    }

    /// Mirrors `getLastDrink()`.
    pub fn get_last_drink(&self) -> i64 {
        self.last_drink
    }

    /// Mirrors `getLastPlayToy()`.
    pub fn get_last_play_toy(&self) -> i64 {
        self.last_play_toy
    }

    /// Mirrors `getLastPlayUser()`.
    pub fn get_last_play_user(&self) -> i64 {
        self.last_play_user
    }

    pub fn set_last_kip(&mut self, last_kip: i64) {
        self.last_kip = last_kip;
    }

    pub fn set_last_eat(&mut self, last_eat: i64) {
        self.last_eat = last_eat;
    }

    pub fn set_last_drink(&mut self, last_drink: i64) {
        self.last_drink = last_drink;
    }

    pub fn set_last_play_toy(&mut self, last_play_toy: i64) {
        self.last_play_toy = last_play_toy;
    }

    pub fn set_last_play_user(&mut self, last_play_user: i64) {
        self.last_play_user = last_play_user;
    }

    /// Mirrors `getX()`.
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// Mirrors `getY()`.
    pub fn get_y(&self) -> i32 {
        self.y
    }

    /// Mirrors `getRotation()`.
    pub fn get_rotation(&self) -> i32 {
        self.rotation
    }
}
