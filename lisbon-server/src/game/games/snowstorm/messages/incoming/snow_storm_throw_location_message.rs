//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.messages.incoming.SnowStormThrowLocationMessage`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::snowstorm::events::snow_storm_launch_snowball_event::SnowStormLaunchSnowballEvent;
use crate::game::games::snowstorm::events::snow_storm_throw_event::SnowStormThrowEvent;
use crate::game::games::snowstorm::objects::snowball_object::SnowballObject;
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::game::games::snowstorm::util::snow_storm_message::SnowStormMessage;
use crate::game::pathfinder::position::Position;
use crate::game::pathfinder::rotation::Rotation;
use crate::server::netty::streams::NettyRequest;

pub struct SnowStormThrowLocationMessage;

impl SnowStormMessage for SnowStormThrowLocationMessage {
    /// Mirrors `handle(NettyRequest, SnowStormGame, GamePlayer)`.
    fn handle(
        &self,
        reader: &mut NettyRequest,
        snow_storm_game: &Arc<SnowStormGame>,
        game_player: &Arc<Mutex<GamePlayer>>,
    ) {
        let (walkable, last_throw, snowballs) = {
            let game_player = game_player.lock();
            let attrs = game_player.get_snow_storm_attributes();
            (
                attrs.is_walkable(),
                attrs.get_last_throw(),
                attrs.get_snowballs(),
            )
        };

        if !walkable {
            return;
        }

        if (last_throw + 300) > chrono::Utc::now().timestamp_millis() {
            return;
        }

        let x = reader.read_int();
        let y = reader.read_int();
        let trajectory = reader.read_int();

        if trajectory != 2 && trajectory != 1 {
            return;
        }

        if snowballs <= 0 {
            return;
        }

        let object_id = snow_storm_game.create_object_id();

        let (from_x, from_y, direction) = {
            let game_player = game_player.lock();
            let Some(current_position) = game_player
                .get_snow_storm_attributes()
                .get_current_position()
            else {
                // Java `NullPointerException` equivalent.
                return;
            };

            let target_x = SnowStormGame::convert_to_game_coordinate(x);
            let target_y = SnowStormGame::convert_to_game_coordinate(y);
            let direction = Rotation::calculate_walk_direction(
                &current_position,
                &Position::new_xy(target_x, target_y),
            );
            (current_position.get_x(), current_position.get_y(), direction)
        };

        let snowball = Arc::new(Mutex::new(SnowballObject::new(
            object_id,
            Arc::clone(snow_storm_game),
            Arc::clone(game_player),
            from_x,
            from_y,
            SnowStormGame::convert_to_game_coordinate(x),
            SnowStormGame::convert_to_game_coordinate(y),
            trajectory,
            direction,
        )));

        game_player
            .lock()
            .get_snow_storm_attributes()
            .decrement_snowballs();

        let visibility_path = snowball.lock().get_path();
        let last_tile_position = visibility_path.last().cloned();

        // Reconsider velocity/time to live and recalculate since it's
        // blocked
        if let Some(last_tile_position) = last_tile_position {
            if last_tile_position != Position::new_xy(snowball.lock().get_target_x(), snowball.lock().get_target_y()) {
                let mut snowball = snowball.lock();
                snowball.set_target_x(last_tile_position.get_x());
                snowball.set_target_y(last_tile_position.get_y());
                snowball.set_blocked(true);
            }
        }

        let (target_x, target_y) = {
            let snowball = snowball.lock();
            (
                SnowStormGame::convert_to_world_coordinate(snowball.get_target_x()),
                SnowStormGame::convert_to_world_coordinate(snowball.get_target_y()),
            )
        };

        if let Some(update_task) = snow_storm_game.get_update_task() {
            update_task.send_queue(
                0,
                1,
                Arc::new(SnowStormThrowEvent::new(
                    game_player.lock().get_object_id(),
                    target_x,
                    target_y,
                    trajectory,
                )),
            );
            update_task.send_queue(
                0,
                1,
                Arc::new(SnowStormLaunchSnowballEvent::new(
                    object_id,
                    game_player.lock().get_object_id(),
                    target_x,
                    target_y,
                    trajectory,
                )),
            );
        }

        game_player
            .lock()
            .get_snow_storm_attributes()
            .set_last_throw(chrono::Utc::now().timestamp_millis());

        SnowballObject::schedule_movement_task(&snowball);
    }
}
