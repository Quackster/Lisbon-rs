//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.messages.incoming.SnowStormWalkMessage`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::game::games::snowstorm::util::snow_storm_message::SnowStormMessage;
use crate::game::pathfinder::position::Position;
use crate::server::netty::streams::NettyRequest;

pub struct SnowStormWalkMessage;

impl SnowStormMessage for SnowStormWalkMessage {
    /// Mirrors `handle(NettyRequest, SnowStormGame, GamePlayer)`.
    fn handle(
        &self,
        reader: &mut NettyRequest,
        _snow_storm_game: &Arc<SnowStormGame>,
        game_player: &Arc<Mutex<GamePlayer>>,
    ) {
        // Port note: the Java `RoomEntity.isWalkingAllowed()` check has
        // no Rust equivalent (the `RoomEntity` stub only exposes
        // `set_walking_allowed`); it is treated as allowed.
        let walkable = game_player
            .lock()
            .get_snow_storm_attributes()
            .is_walkable();

        if !walkable {
            return;
        }

        let x = reader.read_int();
        let y = reader.read_int();

        let new_x = SnowStormGame::convert_to_game_coordinate(x);
        let new_y = SnowStormGame::convert_to_game_coordinate(y);

        let same_position = {
            let game_player = game_player.lock();
            game_player
                .get_snow_storm_attributes()
                .get_current_position()
                .map(|c| *c == Position::new_xy(new_x, new_y))
                .unwrap_or(false)
        };

        if same_position {
            return;
        }

        let mut game_player = game_player.lock();
        game_player
            .get_snow_storm_attributes_mut()
            .set_goal_world_coordinates(Some([x, y]));
        game_player.get_snow_storm_attributes_mut().set_walking(true);
        game_player
            .get_snow_storm_attributes_mut()
            .set_walk_goal(Some(Position::new_xy(new_x, new_y)));
    }
}
