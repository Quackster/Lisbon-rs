//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.mapping.SnowStormMap`.
use std::sync::Arc;

use crate::game::games::snowstorm::mapping::snow_storm_item::SnowStormItem;
use crate::game::games::snowstorm::mapping::snow_storm_tile::SnowStormTile;
use crate::game::games::snowstorm::util::snow_storm_spawn::SnowStormSpawn;
use crate::game::pathfinder::position::Position;

#[derive(Clone, Debug)]
pub struct SnowStormMap {
    map_id: i32,
    item_list: Vec<SnowStormItem>,
    spawn_clusters: Vec<SnowStormSpawn>,
    compiled_items: String,
    height_map: String,
    map_size_y: i32,
    map_size_x: i32,
    tiles: Vec<Vec<Option<SnowStormTile>>>,
}

impl SnowStormMap {
    /// Mirrors the `SnowStormMap(int, String, ArrayList<SnowStormItem>,
    /// String, ArrayList<SnowStormSpawn>)` constructor.
    pub fn new(
        map_id: i32,
        compiled_items: String,
        item_list: Vec<SnowStormItem>,
        height_map: String,
        spawn_clusters: Vec<SnowStormSpawn>,
    ) -> Self {
        let mut map = Self {
            map_id,
            item_list,
            spawn_clusters,
            compiled_items,
            height_map,
            map_size_y: 0,
            map_size_x: 0,
            tiles: Vec::new(),
        };

        map.parse_height_map();
        map
    }

    /// Mirrors `parseHeightMap()`.
    pub fn parse_height_map(&mut self) {
        let lines: Vec<&str> = self.height_map.split('|').collect();

        self.map_size_y = lines.len() as i32;
        self.map_size_x = if lines.is_empty() {
            0
        } else {
            lines[0].len() as i32
        };

        self.tiles = vec![vec![None; self.map_size_y as usize]; self.map_size_x as usize];

        let mut temporary_heightmap = String::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, tile_char) in line.chars().enumerate() {
                let position = Position::new_xy(x as i32, y as i32);
                let snow_storm_tile = SnowStormTile::new(
                    x as i32,
                    y as i32,
                    tile_char.to_ascii_lowercase() == 'x',
                    self.item_list
                        .iter()
                        .filter(|item| item.get_x() == position.get_x() && item.get_y() == position.get_y())
                        .map(|item| Arc::new(item.clone()))
                        .collect(),
                );

                self.tiles[x][y] = Some(snow_storm_tile.clone());

                let tile: String = if !snow_storm_tile.is_walkable() {
                    "x".to_string()
                } else {
                    "0".to_string()
                };

                temporary_heightmap.push_str(&tile);
            }

            temporary_heightmap.push('\r');
        }

        self.height_map = temporary_heightmap;
    }

    /// Mirrors `getTile(Position)`.
    pub fn get_tile(&self, position: &Position) -> Option<&SnowStormTile> {
        if position.get_x() < 0 || position.get_y() < 0 {
            return None;
        }

        if position.get_x() >= self.map_size_x || position.get_y() >= self.map_size_y {
            return None;
        }

        self.tiles[position.get_x() as usize][position.get_y() as usize].as_ref()
    }

    /// Mirrors `getMapSizeX()`.
    pub fn get_map_size_x(&self) -> i32 {
        self.map_size_x
    }

    /// Mirrors `getMapSizeY()`.
    pub fn get_map_size_y(&self) -> i32 {
        self.map_size_y
    }

    /// Mirrors `getMapId()`.
    pub fn get_map_id(&self) -> i32 {
        self.map_id
    }

    /// Mirrors `getHeightMap()`.
    pub fn get_height_map(&self) -> &str {
        &self.height_map
    }

    /// Mirrors `getItems()`.
    pub fn get_items(&self) -> &Vec<SnowStormItem> {
        &self.item_list
    }

    /// Mirrors `getSpawnClusters()`.
    pub fn get_spawn_clusters(&self) -> &Vec<SnowStormSpawn> {
        &self.spawn_clusters
    }

    /// Mirrors `getCompiledItems()`.
    pub fn get_compiled_items(&self) -> &str {
        &self.compiled_items
    }
}
