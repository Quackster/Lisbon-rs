//! Mirrors `net.h4bbo.lisbon.game.room.models.RoomModel`.

use rand::Rng;

use crate::game::pathfinder::position::Position;
use crate::game::room::mapping::room_tile_state::RoomTileState;
use crate::game::room::models::room_model_trigger_type::RoomModelTriggerType;

#[derive(Clone, Debug)]
pub struct RoomModel {
    model_id: String,
    model_name: String,
    door_x: i32,
    door_y: i32,
    door_z: f64,
    door_rotation: i32,
    map_size_x: i32,
    map_size_y: i32,
    heightmap: String,
    tile_states: Vec<Vec<RoomTileState>>,
    tile_heights: Vec<Vec<f64>>,
    model_trigger: Option<RoomModelTriggerType>,
}

impl RoomModel {
    /// Mirrors the 8-arg `RoomModel` constructor.
    pub fn new(
        model_id: &str,
        model_name: &str,
        door_x: i32,
        door_y: i32,
        door_z: f64,
        door_rotation: i32,
        heightmap: &str,
        trigger_class: Option<&str>,
    ) -> Self {
        let mut model = Self {
            model_id: model_id.to_string(),
            model_name: model_name.to_string(),
            door_x,
            door_y,
            door_z,
            door_rotation,
            map_size_x: 0,
            map_size_y: 0,
            heightmap: heightmap.to_string(),
            tile_states: Vec::new(),
            tile_heights: Vec::new(),
            model_trigger: None,
        };

        if let Some(trigger_class) = trigger_class {
            if !trigger_class.trim().is_empty() {
                // Java `valueOf` throws on an unknown name; the port skips
                // the trigger instead.
                model.model_trigger = RoomModelTriggerType::from_str(&trigger_class.to_uppercase());
            }
        }

        model.parse();
        model
    }

    /// Parse the heightmap, add invalid tiles and the tile heights used
    /// for walking, stairs, etc.
    fn parse(&mut self) {
        let lines: Vec<&str> = self.heightmap.split('|').collect();

        self.map_size_y = lines.len() as i32;
        self.map_size_x = lines.first().map(|line| line.chars().count() as i32).unwrap_or(0);

        self.tile_states = vec![
            vec![RoomTileState::Closed; self.map_size_x as usize];
            self.map_size_y as usize
        ];
        self.tile_heights = vec![vec![0.0; self.map_size_x as usize]; self.map_size_y as usize];

        let mut temporary_heightmap = String::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                if x >= self.map_size_x as usize {
                    break;
                }

                let tile = tile.to_string();

                // `StringUtils.isNumeric` — all ASCII digits.
                if !tile.is_empty() && tile.chars().all(|c| c.is_ascii_digit()) {
                    self.tile_states[y][x] = RoomTileState::Open;
                    // Java `Double.parseDouble` throws on bad input; the
                    // digit check above makes the parse total here.
                    self.tile_heights[y][x] = tile.parse::<f64>().unwrap_or(0.0);
                } else {
                    self.tile_states[y][x] = RoomTileState::Closed;
                    self.tile_heights[y][x] = 0.0;
                }

                if x as i32 == self.door_x && y as i32 == self.door_y {
                    self.tile_states[y][x] = RoomTileState::Open;
                    self.tile_heights[y][x] = self.door_z;
                }

                temporary_heightmap.push_str(&tile);
            }

            temporary_heightmap.push('\r');
        }

        self.heightmap = temporary_heightmap;
    }

    /// Get the tile state by the given coordinates. This doesn't include
    /// room furniture.
    pub fn get_tile_state(&self, x: i32, y: i32) -> RoomTileState {
        if x < 0 || y < 0 {
            return RoomTileState::Closed;
        }

        if x >= self.map_size_x || y >= self.map_size_y {
            return RoomTileState::Closed;
        }

        self.tile_states[y as usize][x as usize]
    }

    /// Get the tile height, this doesn't include furniture heights.
    pub fn get_tile_height(&self, x: i32, y: i32) -> f64 {
        if x < 0 || y < 0 {
            return 0.0;
        }

        if x >= self.map_size_x || y >= self.map_size_y {
            return 0.0;
        }

        self.tile_heights[y as usize][x as usize]
    }

    /// Mirrors `getId`.
    pub fn get_id(&self) -> &str {
        &self.model_id
    }

    /// Mirrors `getName`.
    pub fn get_name(&self) -> &str {
        &self.model_name
    }

    /// Mirrors `getDoorLocation`.
    pub fn get_door_location(&self) -> Position {
        Position::with_rotations(self.door_x, self.door_y, self.door_z, self.door_rotation, self.door_rotation)
    }

    /// Mirrors `getMapSizeX`.
    pub fn get_map_size_x(&self) -> i32 {
        self.map_size_x
    }

    /// Mirrors `getMapSizeY`.
    pub fn get_map_size_y(&self) -> i32 {
        self.map_size_y
    }

    /// Mirrors `getHeightmap`.
    pub fn get_heightmap(&self) -> &str {
        &self.heightmap
    }

    /// Mirrors `getModelTrigger`.
    pub fn get_model_trigger(&self) -> Option<RoomModelTriggerType> {
        self.model_trigger
    }

    /// Mirrors `getRoomTrigger`.
    pub fn get_room_trigger(&self) -> Option<crate::game::room::models::triggers::RoomTrigger> {
        self.model_trigger.and_then(|trigger| trigger.get_room_trigger())
    }

    /// Mirrors `getRandomBound`.
    pub fn get_random_bound(&self, bound_id: i32) -> i32 {
        if bound_id == 0 {
            return rand::thread_rng().gen_range(0..self.map_size_x);
        }

        if bound_id == 1 {
            return rand::thread_rng().gen_range(0..self.map_size_y);
        }

        -1
    }
}
