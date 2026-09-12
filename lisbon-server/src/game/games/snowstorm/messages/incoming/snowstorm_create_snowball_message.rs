//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.messages.incoming.SnowstormCreateSnowballMessage`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::snowstorm::events::snow_storm_create_snowball_event::SnowStormCreateSnowballEvent;
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::game::games::snowstorm::util::snow_storm_activity_state::SnowStormActivityState;
use crate::game::games::snowstorm::util::snow_storm_message::SnowStormMessage;
use crate::server::netty::streams::NettyRequest;

pub struct SnowstormCreateSnowballMessage;

impl SnowStormMessage for SnowstormCreateSnowballMessage {
    /// Mirrors `handle(NettyRequest, SnowStormGame, GamePlayer)`.
    fn handle(
        &self,
        _request: &mut NettyRequest,
        snow_storm_game: &Arc<SnowStormGame>,
        game_player: &Arc<Mutex<GamePlayer>>,
    ) {
        let (walkable, snowballs, walking) = {
            let game_player = game_player.lock();
            let attrs = game_player.get_snow_storm_attributes();
            (
                attrs.is_walkable(),
                attrs.get_snowballs(),
                attrs.is_walking(),
            )
        };

        if !walkable {
            return;
        }

        if snowballs >= 5 {
            return;
        }

        let player_arc = Arc::clone(game_player);

        if walking {
            player_arc.lock().get_snow_storm_attributes_mut().set_walking(false);
        }

        let object_id = player_arc.lock().get_object_id();

        if let Some(update_task) = snow_storm_game.get_update_task() {
            update_task.send_queue(
                0,
                1,
                Arc::new(SnowStormCreateSnowballEvent::new(object_id)),
            );
        }

        let arg_arc = Arc::clone(&player_arc);
        let cb_arc = Arc::clone(&player_arc);
        player_arc
            .lock()
            .get_snow_storm_attributes_mut()
            .set_activity_state(
                arg_arc,
                SnowStormActivityState::ActivityStateCreating,
                Some(Box::new(
                    move || {
                        let (walkable, health) = {
                            let p = cb_arc.lock();
                            let attrs = p.get_snow_storm_attributes();
                            (attrs.is_walkable(), attrs.get_health())
                        };

                        if !walkable || health == 0 {
                            return;
                        }

                        cb_arc
                            .lock()
                            .get_snow_storm_attributes()
                            .increment_snowballs();
                    },
                )),
            );
    }
}
