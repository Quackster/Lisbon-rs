//! Mirrors `net.h4bbo.lisbon.game.pathfinder.AffectedTile`.
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;

pub struct AffectedTile;

impl AffectedTile {
    /// Mirrors `getAffectedTiles(Item)`.
    pub fn get_affected_tiles(item: &Item) -> Vec<Position> {
        Self::get_affected_tiles_impl(
            item.get_definition().get_length(),
            item.get_definition().get_width(),
            item.get_position().get_x(),
            item.get_position().get_y(),
            item.get_position().get_rotation(),
        )
    }

    /// Mirrors `getAffectedTiles(Item, int, int, int)`.
    pub fn get_affected_tiles_ext(
        item: &Item,
        x: i32,
        y: i32,
        rotation: i32,
    ) -> Vec<Position> {
        Self::get_affected_tiles_impl(
            item.get_definition().get_length(),
            item.get_definition().get_width(),
            x,
            y,
            rotation,
        )
    }

    /// Mirrors `getAffectedTiles(int, int, int, int, int)`.
    pub fn get_affected_tiles_impl(
        mut length: i32,
        mut width: i32,
        x: i32,
        y: i32,
        rotation: i32,
    ) -> Vec<Position> {
        let mut points = Vec::new();

        if length != width && (rotation == 0 || rotation == 4) {
            let l = length;
            length = width;
            width = l;
        }

        for new_x in x..x + width {
            for new_y in y..y + length {
                points.push(Position::new_xy(new_x, new_y));
            }
        }

        points
    }
}
