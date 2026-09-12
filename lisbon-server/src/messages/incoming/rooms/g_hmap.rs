//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.G_HMAP`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::games::fullgamestatus::FULLGAMESTATUS;
use crate::messages::outgoing::rooms::heightmap::HEIGHTMAP;
use crate::messages::outgoing::rooms::objects_world::OBJECTS_WORLD;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct G_HMAP;

impl MessageEvent for G_HMAP {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        let Some(game_player_arc) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_game_player())
        else {
            // No game player: send the height map from the room model.
            if let Some(model) = room.get_model() {
                player.send(&HEIGHTMAP::from_room_model(model));
            }
            return Ok(());
        };

        let game_player = game_player_arc.lock();
        let game = game_player.get_game();
        let snow_game = game
            .as_ref()
            .and_then(|game| game.as_snow_storm());

        if snow_game.is_some() {
            // The Java NPEs when the map is gone.
            if let Some(map) = snow_game.as_ref().and_then(|snow_game| snow_game.get_map()) {
                player.send(&HEIGHTMAP::new(map.get_height_map()));
            }
        } else if let Some(model) = room.get_model() {
            player.send(&HEIGHTMAP::from_room_model(model));
        }

        if let Some(game) = game {
            player.send(&FULLGAMESTATUS::new(game));

            // The Java NPEs when the map is gone.
            if let Some(map) = snow_game.as_ref().and_then(|snow_game| snow_game.get_map()) {
                player.send(&OBJECTS_WORLD::from_data(map.get_compiled_items()));
            }
        }

        Ok(())
    }
}
