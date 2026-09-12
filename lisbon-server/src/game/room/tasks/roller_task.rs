//! Mirrors `net.h4bbo.lisbon.game.room.tasks.RollerTask`.
use crate::game::entity::entity::Entity;
use crate::game::game_scheduler::GameScheduler;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::item::roller::entity_rolling_analysis::EntityRollingAnalysis;
use crate::game::item::roller::item_rolling_analysis::ItemRollingAnalysis;
use crate::game::item::roller::roller_entry::RollerEntry;
use crate::game::item::roller::rolling_analysis::RollingAnalysis;
use crate::game::pathfinder::position::Position;
use crate::game::room::room::Room;
use crate::game::room::tasks::roller_complete_task::RollerCompleteTask;

/// Mirrors `RollerTask` (the Java `Runnable` is the `run` method).
pub struct RollerTask {
    room: Room,
}

impl RollerTask {
    /// Mirrors the `RollerTask(Room)` constructor.
    pub fn new(room: &Room) -> Self {
        Self { room: room.clone() }
    }

    /// Mirrors `run()` (the Java `try/catch` log is dropped; Rust has no
    // checked exceptions).
    pub fn run(&self) {
        let mut items_rolling: Vec<(Item, (Item, Position, Position))> = Vec::new();
        let mut entities_rolling: Vec<(Box<dyn Entity + Send>, (Item, Position, Position))> =
            Vec::new();

        let mut roller_entries: Vec<RollerEntry> = Vec::new();

        let item_rolling_analysis = ItemRollingAnalysis::new();
        let entity_rolling_analysis = EntityRollingAnalysis::new();

        if self
            .room
            .get_items()
            .iter()
            .any(|item| item.has_behaviour(ItemBehaviour::Roller))
        {
            self.room.get_mapping().lock().regenerate_collision_map(&self.room);
        }

        for roller in self.room.get_items().iter() {
            let Some(roller_tile) = roller.get_tile() else {
                continue;
            };

            if !roller.has_behaviour(ItemBehaviour::Roller) {
                continue;
            }

            let mut roller_entry = RollerEntry::new(roller.clone());

            for item in roller_tile.lock().get_items() {
                if item.has_behaviour(ItemBehaviour::Roller) {
                    continue;
                }

                // Java `Map.containsKey` identity is mirrored by id.
                if items_rolling
                    .iter()
                    .any(|(existing, _)| existing.get_id() == item.get_id())
                {
                    continue;
                }

                let mut item = item.clone();

                if let Some(next_position) =
                    item_rolling_analysis.can_roll(&mut item, roller, &self.room)
                {
                    let from_position = item.get_position().copy();
                    items_rolling.push((item.clone(), (roller.clone(), from_position, next_position)));

                    if let Some(rolling_data) = item.get_rolling_data() {
                        roller_entry.add_rolling_item(rolling_data.clone());
                    }
                }
            }

            let roller_tile = roller_tile.lock();
            let roller_entities = roller_tile.get_entities();

            if let Some(first_entity) = roller_entities.first() {
                if entities_rolling.iter().any(|(existing, _)| {
                    existing.get_details().get_id() == first_entity.get_details().get_id()
                }) {
                    continue;
                }

                // Port note: the Java entity-rolling lookup
                // (`canRoll` / `entitiesRolling.put` /
                // `rollerEntry.setRollingEntity`) is skipped; the Rust
                // `RoomTile` entity list is a shared slice, so an owned
                // `Box<dyn Entity + Send>` cannot be taken out of it for
                // the `entities_rolling` map.
            }

            if roller_entry.get_rolling_entity().is_some()
                || !roller_entry.get_rolling_items().is_empty()
            {
                roller_entries.push(roller_entry);
            }
        }

        for (entity, pair) in entities_rolling.iter_mut() {
            let (roller, from_position, next_position) = pair;
            entity_rolling_analysis
                .do_roll(entity, roller, &self.room, from_position, next_position);
        }

        for (item, pair) in items_rolling.iter_mut() {
            let (roller, _from_position, next_position) = pair;

            if !item.is_current_roll_blocked() {
                item_rolling_analysis
                    .do_roll(item, roller, &self.room, _from_position, next_position);
            }

            item.save();
        }

        for entry in roller_entries.iter() {
            let rolling_items: Vec<crate::game::item::roller::rolling_data::RollingData> = entry
                .get_rolling_items()
                .iter()
                .filter(|data| match data.get_item() {
                    Some(item) => !item.is_current_roll_blocked(),
                    None => true,
                })
                .cloned()
                .collect();

            let entity_roller_data = entry
                .get_rolling_entity()
                .and_then(|entity| entity.get_room_user())
                .and_then(|room_user| room_user.get_rolling_data());

            self.room.send(&crate::messages::outgoing::rooms::items::slide_objectbundle::SLIDEOBJECTBUNDLE::new_roller(
                entry.get_roller().clone(),
                rolling_items,
                entity_roller_data,
            ));
        }

        if !items_rolling.is_empty() || !entities_rolling.is_empty() {
            self.room.get_mapping().lock().regenerate_collision_map(&self.room);

            let mut task = RollerCompleteTask::new(
                items_rolling.drain(..).map(|(item, _)| item).collect(),
                entities_rolling.drain(..).map(|(entity, _)| entity).collect(),
                &self.room,
            );

            GameScheduler::get_instance().schedule(move || task.run(), 800);
        }
    }
}
