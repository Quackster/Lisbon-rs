//! Mirrors `net.h4bbo.lisbon.game.games.battleball.BattleBallMap`.

use crate::game::games::enums::game_type::GameType;

#[derive(Clone, Debug)]
pub struct BattleBallMap {
    heightmap: String,
    map_id: i32,
    game_type: GameType,
    battleball_tile_map: Vec<Vec<bool>>,
}

impl BattleBallMap {
    /// Mirrors the `BattleBallMap(int, GameType, String)` constructor.
    pub fn new(map_id: i32, game_type: GameType, tile_map: &str) -> Self {
        let heightmap = tile_map.replace('|', "\r");
        let mut instance = Self {
            heightmap,
            map_id,
            game_type,
            battleball_tile_map: Vec::new(),
        };

        instance.parse();

        instance
    }

    fn parse(&mut self) {
        let lines: Vec<&str> = self.heightmap.split('\r').collect();
        let map_size_y = lines.len() as i32;
        let map_size_x = lines.first().map(|line| line.chars().count() as i32).unwrap_or(0);

        self.battleball_tile_map =
            vec![vec![false; map_size_y as usize]; map_size_x as usize];

        for (y, line) in lines.iter().enumerate() {
            for x in 0..map_size_x {
                let tile = line.chars().nth(x as usize);
                let mut value = match tile {
                    Some(char) if char.is_numeric() => char == '1',
                    _ => false,
                };

                // Temporary fix for the two tiles on Sky Peak
                if self.map_id == 1 && ((x == 24 && y == 17) || (x == 24 && y == 18)) {
                    value = true;
                }

                self.battleball_tile_map[x as usize][y] = value;
            }
        }
    }

    /// Mirrors `getGameType()`.
    pub fn get_game_type(&self) -> GameType {
        self.game_type
    }

    /// Mirrors `getMapId()`.
    pub fn get_map_id(&self) -> i32 {
        self.map_id
    }

    /// Mirrors `isGameTile(int, int)` (out-of-bounds reads return false
    /// instead of throwing).
    pub fn is_game_tile(&self, x: i32, y: i32) -> bool {
        self.battleball_tile_map
            .get(x as usize)
            .and_then(|row| row.get(y as usize))
            .copied()
            .unwrap_or(false)
    }
}
