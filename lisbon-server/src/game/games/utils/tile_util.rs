//! Mirrors `net.h4bbo.lisbon.game.games.utils.TileUtil`.
use crate::game::entity::entity::Entity;
use crate::game::games::battleball::enums::battle_ball_colour_state::BattleBallColourState;
use crate::game::games::battleball::battle_ball_tile::BattleBallTile;
use crate::game::games::battleball::enums::battle_ball_tile_state::BattleBallTileState;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::room::mapping::room_tile::RoomTile;

pub struct TileUtil;

impl TileUtil {
    /// Mirrors `undoTileAttributes(BattleBallTile, Game)` (the `Game`
    // parameter is unused in the Java source).
    pub fn undo_tile_attributes(tile: &mut BattleBallTile) -> bool {
        let state = tile.get_state();
        let colour = tile.get_colour();

        if colour == BattleBallColourState::Default || state == BattleBallTileState::Default {
            return false;
        }

        tile.clear_points_referece();

        tile.set_colour(BattleBallColourState::Default);
        tile.set_state(BattleBallTileState::Default);

        /* The point-removal block (and its score loop) is commented out in
        the Java source. */

        true
    }

    /// Mirrors `isValidGameTile(GamePlayer, BattleBallTile, boolean)`.
    pub fn is_valid_game_tile(
        game_player: &GamePlayer,
        tile: Option<&BattleBallTile>,
        check_entities: bool,
    ) -> bool {
        let Some(tile) = tile else {
            // && tile.getColour() != BattleBallColourState.DISABLED; is
            // commented out in the Java source.
            return false;
        };

        let Some(game) = game_player.get_game() else {
            return false;
        };

        let room = game.get_room();

        let player = game_player.get_player().lock();
        let entity: Option<&(dyn Entity + Send)> =
            if check_entities { Some(&*player) } else { None };

        RoomTile::is_valid_tile(&room, entity, tile.get_position())
    }
}
