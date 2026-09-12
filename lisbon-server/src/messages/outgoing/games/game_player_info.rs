//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.GAMEPLAYERINFO`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game_manager::GameManager;
use crate::game::player::player::Player;
use crate::game::player::statistics::player_statistic::PlayerStatistic;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct GAMEPLAYERINFO {
    players: Vec<Arc<Mutex<Player>>>,
    r#type: GameType,
}

impl GAMEPLAYERINFO {
    /// Mirrors the `GAMEPLAYERINFO(GameType, List<Player>)` constructor.
    pub fn new(r#type: GameType, players: Vec<Arc<Mutex<Player>>>) -> Self {
        Self { players, r#type }
    }
}

impl MessageComposer for GAMEPLAYERINFO {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.players.len() as i32);

        for player in &self.players {
            let player = player.lock();
            // The Java NPEs when the player has no room user.
            response.write_int(
                player
                    .get_room_user()
                    .map_or(0, |room_user| room_user.get_instance_id()),
            );

            if self.r#type == GameType::Battleball {
                response.write_string(
                    player
                        .get_statistic_manager()
                        .get_int_value(PlayerStatistic::BattleballPointsAllTime),
                );
            }

            if self.r#type == GameType::Snowstorm {
                response.write_string(
                    player
                        .get_statistic_manager()
                        .get_int_value(PlayerStatistic::SnowstormPointsAllTime),
                );
            }

            // The Java NPEs when the rank is unset.
            response.write_string(
                GameManager::get_instance()
                    .get_rank_by_points(self.r#type, &player)
                    .map_or("", |rank| rank.get_title()),
            );
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        250 // "Cz"
    }
}
