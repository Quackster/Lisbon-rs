//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.GAMESTATUS`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::battleball::battle_ball_game::BattleBallGame;
use crate::game::games::battleball::battle_ball_tile::BattleBallTile;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game_event::GameEvent;
use crate::game::games::game_object::GameObject;
use crate::game::games::player::game_team::GameTeam;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct GAMESTATUS {
    game: Arc<BattleBallGame>,
    game_teams: Vec<Arc<Mutex<GameTeam>>>,
    objects: Vec<Box<dyn GameObject>>,
    events: Vec<Box<dyn GameEvent>>,
    update_tiles: Vec<Arc<Mutex<BattleBallTile>>>,
    fill_tiles: Vec<Arc<Mutex<BattleBallTile>>>,
}

impl GAMESTATUS {
    /// Mirrors the `GAMESTATUS(Game, Collection<GameTeam>,
    // List<GameObject>, List<GameEvent>, List<BattleBallTile>,
    // List<BattleBallTile>)` constructor.
    pub fn new(
        game: Arc<BattleBallGame>,
        game_teams: Vec<Arc<Mutex<GameTeam>>>,
        objects: Vec<Box<dyn GameObject>>,
        events: Vec<Box<dyn GameEvent>>,
        update_tiles: Vec<Arc<Mutex<BattleBallTile>>>,
        fill_tiles: Vec<Arc<Mutex<BattleBallTile>>>,
    ) -> Self {
        Self {
            game,
            game_teams,
            objects,
            events,
            update_tiles,
            fill_tiles,
        }
    }
}

impl MessageComposer for GAMESTATUS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        // TODO: Handle more than just objects events (power ups, etc)
        // (inherited from the Java).
        response.write_int(self.objects.len() as i32);

        for object in &self.objects {
            response.write_int(object.get_game_object_type().get_object_id());
            object.serialise_object(response);
        }

        if self.game.get_game_type() == GameType::Battleball {
            response.write_int(self.update_tiles.len() as i32);

            for tile in &self.update_tiles {
                let tile = tile.lock();
                let position = tile.get_position();
                response.write_int(position.get_x());
                response.write_int(position.get_y());
                response.write_int(tile.get_colour().get_colour_id());
                response.write_int(tile.get_state().get_tile_state_id());
            }

            response.write_int(self.fill_tiles.len() as i32);

            for tile in &self.fill_tiles {
                let tile = tile.lock();
                let position = tile.get_position();
                response.write_int(position.get_x());
                response.write_int(position.get_y());
                response.write_int(tile.get_colour().get_colour_id());
                response.write_int(tile.get_state().get_tile_state_id());
            }
        }

        response.write_int(self.game_teams.len() as i32);

        for team in &self.game_teams {
            response.write_int(team.lock().get_points());
        }

        response.write_int(1);
        response.write_int(self.events.len() as i32);

        for event in &self.events {
            response.write_int(event.get_game_event_type().get_event_id());
            event.serialise_event(response);
        }
    }

    fn get_header(&self) -> i16 {
        // "Ct"
        244
    }
}
