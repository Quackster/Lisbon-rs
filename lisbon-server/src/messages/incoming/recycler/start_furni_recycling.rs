//! Mirrors `net.h4bbo.lisbon.messages.incoming.recycler.START_FURNI_RECYCLING`.
use crate::game::entity::entity::Entity;
use crate::dao::mysql::recycler_dao::RecyclerDao;
use crate::game::item::item::Item;
use crate::game::player::player::Player;
use crate::game::recycler::recycler_manager::RecyclerManager;
use crate::messages::outgoing::recycler::recycler_status::RECYCLER_STATUS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct START_FURNI_RECYCLING;

impl MessageEvent for START_FURNI_RECYCLING {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let mut items: Vec<Item> = Vec::new();
        let mut can_recycle = false;

        for _ in 0..2 {
            let count = reader.read_int();

            for _ in 0..count {
                let item_id = reader.read_int();

                let Some(item) = player
                    .get_inventory()
                    .and_then(|inventory| inventory.get_item(item_id))
                else {
                    continue;
                };

                if !item.get_definition().is_recyclable() {
                    continue;
                }

                items.push(item);
            }
        }

        let recycler_reward = RecyclerManager::get_instance()
            .get_recycler_rewards()
            .into_iter()
            .find(|reward| reward.get_item_cost() == items.len() as i32);

        if recycler_reward.is_some() {
            if RecyclerDao::get_session(player.get_details().get_id()).is_none() {
                can_recycle = true;
            }
        }

        if can_recycle {
            for item in items.iter_mut() {
                item.set_hidden(true);
                item.save();
            }

            if let Some(inventory) = player.get_inventory() {
                inventory.view(player, "new");
            }

            let items_str = items
                .iter()
                .map(|item| item.get_id().to_string())
                .collect::<Vec<_>>()
                .join(",");

            let session = RecyclerDao::create_session(
                player.get_details().get_id(),
                recycler_reward.as_ref().unwrap().get_id(),
                &items_str,
            );

            player.send(&RECYCLER_STATUS::new(
                RecyclerManager::get_instance().is_recycler_enabled(),
                Some(session),
            ));
        }

        Ok(())
    }
}
