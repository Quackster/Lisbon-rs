//! Mirrors `net.h4bbo.lisbon.game.room.models.triggers.FlatTrigger`.
use std::any::Any;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::dao::mysql::pet_dao::PetDao;
use crate::dao::mysql::room_visits_dao::RoomVisitsDao;
use crate::game::achievements::achievement_manager::AchievementManager;
use crate::game::achievements::achievement_type::AchievementType;
use crate::game::entity::entity::Entity;
use crate::game::guides::guide_manager::GuideManager;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::item::interactors::types::pet_nest_interactor::PetNestInteractor;
use crate::game::pathfinder::position::Position;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::triggers::generic_trigger::Trigger;
use crate::messages::outgoing::rooms::items::place_flooritem::PLACE_FLOORITEM;
use crate::messages::outgoing::rooms::items::stuff_data_update::STUFFDATAUPDATE;


pub struct FlatTrigger;

impl Trigger for FlatTrigger {
    /// Mirrors `onRoomEntry(Entity, Room, boolean, Object...)`.
    fn on_room_entry(
        &self,
        entity: &dyn Entity,
        room: &Room,
        first_entry: bool,
        _custom_args: &[Box<dyn Any>],
    ) {
        let Some(player) = entity.as_player() else {
            return;
        };

        RoomVisitsDao::add_visit(player.get_details().get_id(), room.get_id());

        AchievementManager::get_instance()
            .try_progress(&AchievementType::RoomEntry, player);

        if player.get_guide_manager().is_guide() && player.get_guide_manager().get_invited_by() > 0 {
            let invited_by = player.get_guide_manager().get_invited_by();

            if room.get_data().get_owner_id() == invited_by {
                let mut found_newb: Option<Arc<Mutex<Player>>> = None;
                for candidate in room.get_entity_manager().get_players() {
                    if candidate.lock().get_details().get_id() == invited_by {
                        found_newb = Some(candidate);
                        break;
                    }
                }

                if let Some(newb) = found_newb {
                    GuideManager::get_instance().tutor_enter_room(player, &newb.lock());
                }

                player.get_guide_manager().set_invited_by(0);
            }
        }

        if first_entry {
            for item in room
                .get_item_manager()
                .get_floor_items()
                .iter()
                .filter(|item| {
                    item.get_definition().get_interaction_type() == Some(InteractionType::PetNest)
                })
            {
                if let Some(nest_box) = InteractionType::PetNest.get_trigger() {
                    if let Some(interactor) = nest_box.downcast_ref::<PetNestInteractor>() {
                        if let Some(pet_details) = PetDao::get_pet_details(item.get_id()) {
                            let mut position = Position::new_xy(
                                pet_details.get_x(),
                                pet_details.get_y(),
                            );
                            position.set_rotation(pet_details.get_rotation());

                            interactor.add_pet(room, &pet_details, &position);
                        }
                    }
                }
            }
        }

        for item in room
            .get_item_manager()
            .get_floor_items()
            .iter()
            .filter(|item| {
                item.get_definition().get_interaction_type() == Some(InteractionType::PetWaterBowl)
            })
        {
            let item = item.clone();
            player.send(&PLACE_FLOORITEM::new(Box::new(item.clone())));
            player.send(&STUFFDATAUPDATE::new(Box::new(item)));
        }
    }

    /// Mirrors `onRoomLeave(Entity, Room, Object...)`.
    fn on_room_leave(&self, _entity: &dyn Entity, _room: &Room, _custom_args: &[Box<dyn Any>]) {}
}

