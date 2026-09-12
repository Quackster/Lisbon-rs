//! Mirrors `net.h4bbo.lisbon.messages.incoming.recycler.CONFIRM_FURNI_RECYCLING`.
use crate::game::entity::entity::Entity;
use crate::dao::mysql::recycler_dao::RecyclerDao;
use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::recycler::start_recycling_result::START_RECYCLING_RESULT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::date_util::DateUtil;

#[allow(non_camel_case_types)]
pub struct CONFIRM_FURNI_RECYCLING;

impl MessageEvent for CONFIRM_FURNI_RECYCLING {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(recycler_session) = RecyclerDao::get_session(player.get_details().get_id()) else {
            return Ok(());
        };

        let is_cancel = !reader.read_boolean();

        if !is_cancel {
            if !recycler_session.is_recycling_done() || recycler_session.has_timeout() {
                return Ok(());
            }
        }

        let Some(inventory) = player.get_inventory() else {
            return Ok(());
        };

        for item_id in recycler_session.get_items() {
            let Some(mut item) = inventory.get_item(item_id) else {
                continue;
            };

            if is_cancel {
                item.set_hidden(false);
                item.save();

                inventory.add_item(&item);
            } else {
                inventory.remove_item(&item);
                item.delete();
            }
        }

        RecyclerDao::delete_session(player.get_details().get_id());

        if !is_cancel {
            if let Some(reward) = recycler_session.get_recycler_reward() {
                if let Some(catalogue_item) = reward.get_catalogue_item() {
                    let item_list = CatalogueManager::get_instance().purchase(
                        player.get_details(),
                        catalogue_item,
                        None,
                        None,
                        DateUtil::get_current_time_seconds() as i64,
                    );

                    if !item_list.is_empty() {
                        inventory.view(player, "new");
                    }
                }
            }
        }

        player.send(&START_RECYCLING_RESULT::new(true));
        inventory.view(player, "new");

        Ok(())
    }
}
