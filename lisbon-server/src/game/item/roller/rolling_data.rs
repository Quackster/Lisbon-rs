//! Mirrors `net.h4bbo.lisbon.game.item.roller.RollingData`.
use crate::game::entity::entity::Entity;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;

#[derive(Debug)]
pub struct RollingData {
    // The `Box` indirection breaks the `Item` / `RollingData` size
    // recursion (`Item.rolling_data`).
    roller: Box<Item>,
    item: Option<Box<Item>>,
    // The Java `entity` reference cannot be stored from the
    // `&mut Box<dyn Entity + Send>` borrow the rolling analysis passes in;
    // the room-user instance id it exposes to the `SLIDEOBJECTBUNDLE`
    // packet is kept instead.
    entity_instance_id: i32,
    from_position: Position,
    next_position: Position,
    display_height: f64,
    height_update: f64,
}

impl Clone for RollingData {
    fn clone(&self) -> Self {
        Self {
            roller: self.roller.clone(),
            item: self.item.clone(),
            entity_instance_id: self.entity_instance_id,
            from_position: self.from_position.clone(),
            next_position: self.next_position.clone(),
            display_height: self.display_height,
            height_update: self.height_update,
        }
    }
}

impl RollingData {
    /// Mirrors the `RollingData(Entity, Item, Position, Position)`
    /// constructor.
    pub fn new_entity(
        entity: &(dyn Entity + Send),
        roller: &Item,
        from_position: &Position,
        next_position: &Position,
    ) -> Self {
        Self {
            roller: Box::new(roller.clone()),
            item: None,
            entity_instance_id: entity
                .get_room_user()
                .map_or(0, |room_user| room_user.get_instance_id()),
            from_position: from_position.clone(),
            next_position: next_position.clone(),
            display_height: 0.0,
            height_update: -1.0,
        }
    }

    /// Mirrors the `RollingData(Item, Item, Position, Position)`
    /// constructor.
    pub fn new_item(
        item: &Item,
        roller: &Item,
        from_position: &Position,
        next_position: &Position,
    ) -> Self {
        Self {
            roller: Box::new(roller.clone()),
            item: Some(Box::new(item.clone())),
            entity_instance_id: 0,
            from_position: from_position.clone(),
            next_position: next_position.clone(),
            display_height: 0.0,
            height_update: -1.0,
        }
    }

    /// Mirrors `getItem`.
    pub fn get_item(&self) -> Option<&Item> {
        self.item.as_deref()
    }

    /// Mirrors `getRoller`.
    pub fn get_roller(&self) -> &Item {
        &self.roller
    }

    /// Mirrors `getHeightUpdate`.
    pub fn get_height_update(&self) -> f64 {
        self.height_update
    }

    /// Mirrors `setHeightUpdate`.
    pub fn set_height_update(&mut self, height_update: f64) {
        self.height_update = height_update;
    }

    /// Mirrors `getNextPosition`.
    pub fn get_next_position(&self) -> Position {
        self.next_position.clone()
    }

    /// Mirrors `getFromPosition`.
    pub fn get_from_position(&self) -> Position {
        self.from_position.clone()
    }

    /// Mirrors `getDisplayHeight`.
    pub fn get_display_height(&self) -> f64 {
        self.display_height
    }

    /// Mirrors `setDisplayHeight`.
    pub fn set_display_height(&mut self, display_height: f64) {
        self.display_height = display_height;
    }

    /// Mirrors `getEntity().getRoomUser().getInstanceId()` (the `Entity`
    /// reference is mirrored by the instance id, see the struct docs).
    pub fn get_entity_instance_id(&self) -> i32 {
        self.entity_instance_id
    }
}
