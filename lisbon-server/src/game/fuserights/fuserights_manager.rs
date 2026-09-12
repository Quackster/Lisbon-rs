//! Mirrors `net.h4bbo.lisbon.game.fuserights.FuserightsManager`.
use std::sync::OnceLock;

use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_rank::PlayerRank;

pub struct FuserightsManager {
    fuserights: Vec<Fuseright>,
}

impl FuserightsManager {
    /// Mirrors the `FuserightsManager` constructor.
    pub fn new() -> Self {
        Self {
            fuserights: Fuseright::ALL.to_vec(),
        }
    }

    /// Mirrors `getFuserightsForRank(PlayerRank)`.
    pub fn get_fuserights_for_rank(&self, minimum_rank: PlayerRank) -> Vec<Fuseright> {
        self.fuserights
            .iter()
            .copied()
            .filter(|fuseright| {
                match fuseright.minimum_rank() {
                    Some(minimum) if !fuseright.is_club_only() => {
                        minimum_rank.rank_id() >= minimum.rank_id()
                    }
                    _ => false,
                }
            })
            .collect()
    }

    /// Mirrors `getClubFuserights()`.
    pub fn get_club_fuserights(&self) -> Vec<Fuseright> {
        self.fuserights
            .iter()
            .copied()
            .filter(|fuseright| fuseright.is_club_only())
            .collect()
    }

    /// Mirrors `hasFuseright(Fuseright, PlayerDetails)`.
    pub fn has_fuseright(&self, fuse: Fuseright, details: &PlayerDetails) -> bool {
        let rank_id = details.get_rank().map(|rank| rank.rank_id()).unwrap_or(0);

        for fuseright in &self.fuserights {
            if let Some(minimum) = fuseright.minimum_rank() {
                if rank_id >= minimum.rank_id() && *fuseright == fuse {
                    return true;
                }
            }
        }

        if details.has_club_subscription() {
            for fuseright in &self.fuserights {
                if fuseright.is_club_only() && *fuseright == fuse {
                    return true;
                }
            }
        }

        false
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> &'static FuserightsManager {
        static INSTANCE: OnceLock<FuserightsManager> = OnceLock::new();
        INSTANCE.get_or_init(FuserightsManager::new)
    }
}
