//! Mirrors `net.h4bbo.lisbon.game.games.battleball.events.PlayerMoveEvent`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::enums::game_event_type::GameEventType;
use crate::game::games::game_event::GameEvent;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::pathfinder::position::Position;
use crate::server::netty::streams::NettyResponse;

pub struct PlayerMoveEvent {
    game_player: Arc<Mutex<GamePlayer>>,
    next_position: Position,
}

impl PlayerMoveEvent {
    /// Mirrors the `PlayerMoveEvent(GamePlayer, Position)` constructor.
    pub fn new(game_player: Arc<Mutex<GamePlayer>>, next_position: Position) -> Self {
        Self {
            game_player,
            next_position,
        }
    }

    /// Mirrors `getGamePlayer()`.
    pub fn get_game_player(&self) -> &Arc<Mutex<GamePlayer>> {
        &self.game_player
    }

    /// Mirrors `getNextPosition()`.
    pub fn get_next_position(&self) -> &Position {
        &self.next_position
    }
}

impl GameEvent for PlayerMoveEvent {
    /// Mirrors `serialiseEvent(NettyResponse)`.

    fn serialise_event(&self, response: &mut NettyResponse) {
        let game_player = self.game_player.lock();
        let instance_id = game_player
            .get_player()
            .lock()
            .get_room_user()
            .map(|room_user| room_user.get_instance_id())
            .unwrap_or(0);

        response.write_int(instance_id);
        response.write_int(self.next_position.get_x());
        response.write_int(self.next_position.get_y());
    }

    /// Mirrors `getGameEventType()`.
    fn get_game_event_type(&self) -> GameEventType {
        GameEventType::BattleballPlayerEvent
    }
}
