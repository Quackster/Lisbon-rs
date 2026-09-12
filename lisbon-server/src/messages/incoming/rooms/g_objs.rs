//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.G_OBJS`.
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::active_objects::ACTIVE_OBJECTS;
use crate::messages::outgoing::rooms::objects_world::OBJECTS_WORLD;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct G_OBJS;

impl MessageEvent for G_OBJS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        // `getPublicItems` approximation: the `PUBLIC_SPACE_OBJECT` items that
        // are neither invisible nor private furniture.
        let public_items: Vec<crate::game::item::item::Item> = room
            .get_items()
            .into_iter()
            .filter(|item| {
                item.has_behaviour(ItemBehaviour::PublicSpaceObject)
                    && !item.has_behaviour(ItemBehaviour::Invisible)
                    && !item.has_behaviour(ItemBehaviour::PrivateFurniture)
            })
            .collect();

        player.send_queued(&OBJECTS_WORLD::new(public_items));

        // Can't interact on load; `FlatTrigger` is the workaround.
        let active_objects: Vec<crate::game::item::item::Item> = room
            .get_item_manager()
            .get_floor_items()
            .into_iter()
            .filter(|item| {
                item.get_definition().get_interaction_type() != Some(InteractionType::PetWaterBowl)
            })
            .collect();

        player.send_queued(&ACTIVE_OBJECTS::new(active_objects));

        player.flush();

        if let Some(messenger) = player.get_messenger() {
            messenger.send_status_update();
        }

        Ok(())
    }
}
