//! Mirrors `net.h4bbo.lisbon.game.pathfinder.Rotation`.

use crate::game::pathfinder::position::Position;

pub struct Rotation;

impl Rotation {
    /// Mirrors `calculateHumanDirection(int, int, int, int)`.
    pub fn calculate_human_direction(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
        let mut rotation = 0;

        if x1 > x2 && y1 > y2 {
            rotation = 7;
        } else if x1 < x2 && y1 < y2 {
            rotation = 3;
        } else if x1 > x2 && y1 < y2 {
            rotation = 5;
        } else if x1 < x2 && y1 > y2 {
            rotation = 1;
        } else if x1 > x2 {
            rotation = 6;
        } else if x1 < x2 {
            rotation = 2;
        } else if y1 < y2 {
            rotation = 4;
        }

        rotation
    }

    /// Mirrors `calculateWalkDirection(Position, Position)`.
    pub fn calculate_walk_direction(from: &Position, to: &Position) -> i32 {
        Self::calculate_walk_direction_coords(
            from.get_x(),
            from.get_y(),
            to.get_x(),
            to.get_y(),
        )
    }

    /// Mirrors `calculateWalkDirection(int, int, int, int)`.
    pub fn calculate_walk_direction_coords(x: i32, y: i32, to_x: i32, to_y: i32) -> i32 {
        if x == to_x {
            if y < to_y {
                4
            } else {
                0
            }
        } else if x > to_x {
            if y == to_y {
                6
            } else if y < to_y {
                5
            } else {
                7
            }
        } else {
            if y == to_y {
                2
            } else if y < to_y {
                3
            } else {
                1
            }
        }
    }

    /// Mirrors `getHeadRotation(int, Position, Position)`.
    pub fn get_head_rotation(rotation: i32, position: &Position, towards: &Position) -> i32 {
        let mut head_rotation = rotation;
        let diff = rotation
            - Self::calculate_human_direction(
                position.get_x(),
                position.get_y(),
                towards.get_x(),
                towards.get_y(),
            );

        if position.get_rotation() % 2 == 0 {
            if diff > 0 {
                head_rotation = rotation - 1;
            } else if diff < 0 {
                head_rotation = rotation + 1;
            }
        }

        head_rotation
    }
}
