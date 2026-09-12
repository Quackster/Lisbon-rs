//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.objects.SnowStormAvatarObject`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::server::netty::streams::NettyResponse;

pub struct SnowStormAvatarObject {
    p: Arc<Mutex<GamePlayer>>,
}

impl SnowStormAvatarObject {
    /// Mirrors the `SnowStormAvatarObject(GamePlayer)` constructor.
    pub fn new(game_player: Arc<Mutex<GamePlayer>>) -> Self {
        Self { p: game_player }
    }

    /// Mirrors the `p` field access (`getPlayer()`).
    pub fn get_player(&self) -> &Arc<Mutex<GamePlayer>> {
        &self.p
    }
}

impl GameObject for SnowStormAvatarObject {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        let p = self.p.lock();
        let attrs = p.get_snow_storm_attributes();

        response.write_int(GameObjectType::SnowwarAvatarObject.get_object_id());
        response.write_int(p.get_object_id());
        // Port note: the Java `NullPointerException`s if the
        // `currentPosition` is unset; `0` is used.
        response.write_int(SnowStormGame::convert_to_world_coordinate(
            attrs.get_current_position().map(|c| c.get_x()).unwrap_or(0),
        ));
        response.write_int(SnowStormGame::convert_to_world_coordinate(
            attrs.get_current_position().map(|c| c.get_y()).unwrap_or(0),
        ));
        response.write_int(attrs.get_rotation());
        response.write_int(attrs.get_health());
        response.write_int(attrs.get_snowballs());
        response.write_int(0);
        response.write_int(attrs.get_activity_timer());
        // Port note: the Java `NullPointerException`s if the
        // `activityState` is unset; `0` is used.
        response.write_int(attrs.get_activity_state().map(|s| s.get_state_id()).unwrap_or(0));
        let next_goal = attrs.get_next_goal().cloned();
        response.write_int(next_goal.as_ref().map(|g| g.get_x()).unwrap_or_else(|| {
            attrs.get_current_position().map(|c| c.get_x()).unwrap_or(0)
        }));
        response.write_int(next_goal.as_ref().map(|g| g.get_y()).unwrap_or_else(|| {
            attrs.get_current_position().map(|c| c.get_y()).unwrap_or(0)
        }));
        // Port note: the Java `NullPointerException`s if `isWalking` is
        // true and the `walkGoal` is unset; the current position is used.
        let walk_target_x = if attrs.is_walking() {
            attrs
                .get_walk_goal()
                .map(|g| g.get_x())
                .unwrap_or_else(|| attrs.get_current_position().map(|c| c.get_x()).unwrap_or(0))
        } else {
            attrs.get_current_position().map(|c| c.get_x()).unwrap_or(0)
        };
        let walk_target_y = if attrs.is_walking() {
            attrs
                .get_walk_goal()
                .map(|g| g.get_y())
                .unwrap_or_else(|| attrs.get_current_position().map(|c| c.get_y()).unwrap_or(0))
        } else {
            attrs.get_current_position().map(|c| c.get_y()).unwrap_or(0)
        };
        response.write_int(SnowStormGame::convert_to_world_coordinate(walk_target_x));
        response.write_int(SnowStormGame::convert_to_world_coordinate(walk_target_y));
        response.write_int(attrs.get_score());
        let player = p.get_player().lock();
        let details = player.get_details();
        response.write_int(details.get_id());
        response.write_int(p.get_team_id());
        response.write_int(p.get_object_id());
        response.write_string(details.get_name());
        response.write_string(details.get_motto());
        response.write_string(details.get_figure());
        response.write_string(details.get_sex());
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::SnowwarAvatarObject
    }

    /// Mirrors `getId()`.
    // Port note: the Java passes `gamePlayer.getObjectId()` to `super`;
    // the value is read from the locked `GamePlayer`.
    fn get_id(&self) -> i32 {
        self.p.lock().get_object_id()
    }
}
