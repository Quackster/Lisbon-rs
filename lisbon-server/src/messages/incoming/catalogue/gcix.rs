//! Mirrors `net.h4bbo.lisbon.messages.incoming.catalogue.GCIX`.
use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::catalogue::catalogue_pages::CATALOGUE_PAGES;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GCIX;

impl MessageEvent for GCIX {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(rank) = player.get_details().get_rank() else {
            return Ok(());
        };

        player.send(&CATALOGUE_PAGES::new(
            CatalogueManager::get_instance()
                .get_pages_for_rank(rank, player.get_details().has_club_subscription()),
        ));

        Ok(())
    }
}
