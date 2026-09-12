//! Mirrors `net.h4bbo.lisbon.messages.incoming.navigator.GETUSERFLATCATS`.
use crate::game::entity::entity::Entity;
use crate::game::navigator::navigator_manager::NavigatorManager;
use crate::game::player::player::Player;
use crate::game::player::player_rank::PlayerRank;
use crate::messages::outgoing::navigator::userflatcats::USERFLATCATS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETUSERFLATCATS;

impl MessageEvent for GETUSERFLATCATS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let rank = player
            .get_details()
            .get_rank()
            .unwrap_or(PlayerRank::Normal)
            .rank_id();

        let category_list: Vec<_> = NavigatorManager::get_instance()
            .get_categories()
            .values()
            .filter(|category| !category.is_public_spaces())
            .filter(|category| category.get_minimum_role_access().rank_id() <= rank)
            .cloned()
            .collect();

        player.send(&USERFLATCATS::new(category_list));

        Ok(())
    }
}
