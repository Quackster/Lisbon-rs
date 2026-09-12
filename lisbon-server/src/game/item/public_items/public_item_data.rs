//! Mirrors `net.h4bbo.lisbon.game.item.public_items.PublicItemData`.
#[derive(Clone, Debug)]
pub struct PublicItemData {
    id: String,
    room_model: String,
    sprite: String,
    x: i32,
    y: i32,
    z: f64,
    rotation: i32,
    top_height: f64,
    length: i32,
    width: i32,
    behaviour: String,
    current_program: String,
    teleport_to: Option<String>,
    swim_to: Option<String>,
}

impl PublicItemData {
    /// Mirrors the 14-arg `PublicItemData` constructor.
    pub fn new(
        id: &str,
        room_model: &str,
        sprite: &str,
        x: i32,
        y: i32,
        z: f64,
        rotation: i32,
        top_height: f64,
        length: i32,
        width: i32,
        behaviour: &str,
        current_program: &str,
        teleport_to: Option<&str>,
        swim_to: Option<&str>,
    ) -> Self {
        Self {
            id: id.to_string(),
            room_model: room_model.to_string(),
            sprite: sprite.to_string(),
            x,
            y,
            z,
            rotation,
            top_height,
            length,
            width,
            behaviour: behaviour.to_string(),
            current_program: current_program.to_string(),
            teleport_to: teleport_to.map(|v| v.to_string()),
            swim_to: swim_to.map(|v| v.to_string()),
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Mirrors `getRoomModel()`.
    pub fn get_room_model(&self) -> &str {
        &self.room_model
    }

    /// Mirrors `getSprite()`.
    pub fn get_sprite(&self) -> &str {
        &self.sprite
    }

    /// Mirrors `getX()`.
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// Mirrors `getY()`.
    pub fn get_y(&self) -> i32 {
        self.y
    }

    /// Mirrors `getZ()`.
    pub fn get_z(&self) -> f64 {
        self.z
    }

    /// Mirrors `getRotation()`.
    pub fn get_rotation(&self) -> i32 {
        self.rotation
    }

    /// Mirrors `getLength()`.
    pub fn get_length(&self) -> i32 {
        self.length
    }

    /// Mirrors `getWidth()`.
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /// Mirrors `getBehaviour()`.
    pub fn get_behaviour(&self) -> &str {
        &self.behaviour
    }

    /// Mirrors `getCurrentProgram()`.
    pub fn get_current_program(&self) -> &str {
        &self.current_program
    }

    /// Mirrors `getTopHeight()`.
    pub fn get_top_height(&self) -> f64 {
        self.top_height
    }

    /// Mirrors `getTeleportTo()`.
    pub fn get_teleport_to(&self) -> Option<Vec<String>> {
        self.teleport_to
            .as_ref()
            .map(|v| v.split(' ').map(|s| s.to_string()).collect())
    }

    /// Mirrors `getSwimTo()`.
    pub fn get_swim_to(&self) -> Option<Vec<String>> {
        self.swim_to
            .as_ref()
            .map(|v| v.split(' ').map(|s| s.to_string()).collect())
    }
}
