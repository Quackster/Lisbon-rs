//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.SnowStormMapsManager`.
use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::RwLock;

use crate::game::games::snowstorm::mapping::snow_storm_item::SnowStormItem;
use crate::game::games::snowstorm::mapping::snow_storm_map::SnowStormMap;
use crate::game::games::snowstorm::util::snow_storm_spawn::SnowStormSpawn;
use crate::log::Log;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<SnowStormMapsManager>>> = RwLock::new(None);
}

pub struct SnowStormMapsManager {
    snow_storm_map_maps: HashMap<i32, SnowStormMap>,
}

impl SnowStormMapsManager {
    /// Mirrors the `SnowStormMapsManager` constructor.
    pub fn new() -> Self {
        let mut manager = Self {
            snow_storm_map_maps: HashMap::new(),
        };

        for i in 1..=7 {
            manager.parse_map(i);
        }

        manager
    }

    /// Mirrors `parseMap(int)`.
    fn parse_map(&mut self, map_id: i32) {
        let file_path = format!("tools/snowstorm_maps/arena_{}.dat", map_id);

        if !std::path::Path::new(&file_path).exists() {
            return;
        }

        match std::fs::read_to_string(&file_path) {
            Ok(map_data) => {
                let mut item_list: Vec<SnowStormItem> = Vec::new();

                for item_line in map_data.split('\r') {
                    let item_data: Vec<&str> = item_line.split(' ').collect();

                    if item_data.len() < 6 {
                        continue;
                    }

                    let item = SnowStormItem::new(
                        item_data[0].to_string(),
                        item_data[1].to_string(),
                        item_data[2].parse().unwrap_or(0),
                        item_data[3].parse().unwrap_or(0),
                        item_data[4].parse().unwrap_or(0),
                        item_data[5].parse().unwrap_or(0),
                        self.get_item_height(item_data[1]),
                    );

                    item_list.push(item);
                }

                let snowmachine_data_path = format!(
                    "tools/snowstorm_maps/arena_{}_snowmachines.dat",
                    map_id
                );

                if std::path::Path::new(&snowmachine_data_path).exists() {
                    if let Ok(snowmachine_file_contents) =
                        std::fs::read_to_string(&snowmachine_data_path)
                    {
                        for snowmachine_data in snowmachine_file_contents.split('\r') {
                            let item_data: Vec<&str> = snowmachine_data.split(' ').collect();

                            if item_data.len() < 2 {
                                continue;
                            }

                            let x = item_data[0].parse().unwrap_or(0);
                            let y = item_data[1].parse().unwrap_or(0);

                            item_list.push(SnowStormItem::new(
                                String::new(),
                                "snowball_machine".to_string(),
                                x,
                                y,
                                0,
                                0,
                                1,
                            ));

                            item_list.push(SnowStormItem::new(
                                String::new(),
                                "snowball_machine_hidden".to_string(),
                                x + 1,
                                y,
                                0,
                                0,
                                1,
                            ));

                            item_list.push(SnowStormItem::new(
                                String::new(),
                                "snowball_machine_hidden".to_string(),
                                x + 2,
                                y,
                                0,
                                0,
                                1,
                            ));
                        }
                    }
                }

                let mut spawn_clusters: Vec<SnowStormSpawn> = Vec::new();
                let spawn_cluster_path = format!(
                    "tools/snowstorm_maps/arena_{}_spawn_clusters.dat",
                    map_id
                );

                if std::path::Path::new(&spawn_cluster_path).exists() {
                    if let Ok(contents) = std::fs::read_to_string(&spawn_cluster_path) {
                        for spawn_cluster_data in contents.split('|') {
                            let spawn_data: Vec<&str> = spawn_cluster_data.split(' ').collect();

                            if spawn_data.len() < 4 {
                                continue;
                            }

                            let x = spawn_data[0].parse().unwrap_or(0);
                            let y = spawn_data[1].parse().unwrap_or(0);
                            let radius = spawn_data[2].parse().unwrap_or(0);
                            let min_distance = spawn_data[3].parse().unwrap_or(0);

                            spawn_clusters.push(SnowStormSpawn::new(x, y, radius, min_distance));
                        }
                    }
                }

                self.snow_storm_map_maps.insert(
                    map_id,
                    SnowStormMap::new(
                        map_id,
                        map_data,
                        item_list,
                        self.get_height_map(map_id),
                        spawn_clusters,
                    ),
                );
            }
            Err(ex) => {
                Log::get_error_logger()
                    .error_with(format!("Error when parsing map {}", map_id), ex.to_string());
            }
        }
    }

    /// Mirrors `getItemHeight(String)`.
    fn get_item_height(&self, sprite_name: &str) -> i32 {
        match sprite_name {
            "sw_tree1" | "sw_tree2" | "sw_tree3" | "sw_tree4" | "block_basic3"
            | "obst_snowman" | "block_arch1" | "block_arch3" | "block_arch1b"
            | "block_arch3b" => 3,
            "block_basic2" | "block_ice2" => 2,
            "block_basic" | "obst_duck" | "sw_fence" | "block_ice" => 1,
            _ => 0,
        }
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<SnowStormMapsManager> {
        if let Some(existing) = INSTANCE.read().as_ref() {
            return existing.clone();
        }

        let instance = Arc::new(Self::new());
        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Mirrors `reset()`.
    pub fn reset() {
        INSTANCE.write().take();
        Self::get_instance();
    }

    /// Mirrors `getHeightMap(int)`.
    pub fn get_height_map(&self, map_id: i32) -> String {
        if map_id == 1 {
            return "xxxxxxxxxxxxxxxxxxx00000xxxxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxx0000000xxxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxx000000000xxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxx00000000000xxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxx0000000000000xxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxx000000000000000xxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxx00000000000000000xxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxx0000000000000000000xxxxxxxxxxxxxxxxxxx|xxxxxxxxxxx000000000000000000000xxxxxxxxxxxxxxxxxx|xxxxxxxxxx00000000000000000000000xxxxxxxxxxxxxxxxx|xxxxxxxxx0000000000000000000000000xxxxxxxxxxxxxxxx|xxxxxxxx000000000000000000000000000xxxxxxxxxxxxxxx|xxxxxxx00000000000000000000000000000xxxxxxxxxxxxxx|xxxxxx0000000000000000000000000000000xxxxxxxxxxxxx|xxxxx000000000000000000000000000000000xxxxxxxxxxxx|xxxx00000000000000000000000000000000000xxxxxxxxxxx|xxx0000000000000000000000000000000000000xxxxxxxxxx|xx000000000000000000000000000000000000000xxxxxxxxx|x00000000000000000000000000000000000000000xxxxxxxx|00000000000000000000xxxx0xxxxxx000000000000xxxxxxx|00000000000000000000xxxx0xxxxxxx000000000000xxxxxx|00000000000000000000xxxx0xxxxxxx0000000000000xxxxx|00000000000000000000xxx000000xxx00000000000000xxxx|x0000000000000000000xxx000000xxx000000000000000xxx|xx000000000000000000xxx0000000000000000000000000xx|xxx00000000000000000xxx000000xxx00000000000000000x|xxxx0000000000000000000000000xxx000000000000000000|xxxxx000000000000000xxx000000xxx000000000000000000|xxxxxx00000000000000xxxxxxx0xxxx000000000000000000|xxxxxxx0000000000000xxxxxxx0xxxx000000000000000000|xxxxxxxx0000000000000xxxxxx0xxx0000000000000000000|xxxxxxxxx00000000000000000000000000000000000000000|xxxxxxxxxx000000000000000000000000000000000000000x|xxxxxxxxxxx0000000000000000000000000000000000000xx|xxxxxxxxxxxx00000000000000000000000000000000000xxx|xxxxxxxxxxxxx000000000000000000000000000000000xxxx|xxxxxxxxxxxxxx0000000000000000000000000000000xxxxx|xxxxxxxxxxxxxxx00000000000000000000000000000xxxxxx|xxxxxxxxxxxxxxxx000000000000000000000000000xxxxxxx|xxxxxxxxxxxxxxxxx0000000000000000000000000xxxxxxxx|xxxxxxxxxxxxxxxxxx00000000000000000000000xxxxxxxxx|xxxxxxxxxxxxxxxxxxx000000000000000000000xxxxxxxxxx|xxxxxxxxxxxxxxxxxxxx0000000000000000000xxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxx00000000000000000xxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxx000000000000000xxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxx0000000000000xxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxx00000000000xxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxx000000000xxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxxx0000000xxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxxxx00000xxxxxxxxxxxxxxxxxx|".to_string();
        }

        if map_id == 2 {
            return "xxxxxxxxxxxxxxxxxxx00000xxxxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxx0000000xxxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxx000000000xxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxx00000000000xxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxx0000000000000xxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxx000000000000000xxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxx00000000000000000xxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxx0000000000000000000xxxxxxxxxxxxxxxxxxx|xxxxxxxxxxx000000000000000000000xxxxxxxxxxxxxxxxxx|xxxxxxxxxx00000000000000000000000xxxxxxxxxxxxxxxxx|xxxxxxxxx0000000000000000000000000xxxxxxxxxxxxxxxx|xxxxxxxx000000000000000000000000000xxxxxxxxxxxxxxx|xxxxxxx00000000000000000000000000000xxxxxxxxxxxxxx|xxxxxx0000000000000000000000000000000xxxxxxxxxxxxx|xxxxx000000000000000000000000000000000xxxxxxxxxxxx|xxxx00000000000000000000000000000000000xxxxxxxxxxx|xxx0000000000000000000000000000000000000xxxxxxxxxx|xx000000000000000000000000000000000000000xxxxxxxxx|x00000000000000000000000000000000000000000xxxxxxxx|0000000000000000000000000000000000000000000xxxxxxx|0000000000000000000xxxxxxxxxx0xxxxxx00000000xxxxxx|0000000000000000000xxxxxxxxxx0xxxxxxx00000000xxxxx|0000000000000000000xxxxxxxxxx0xxxxxxx000000000xxxx|x000000000000000000xxx000000000000xxx0000000000xxx|xx00000000000000000xxx000000000000xxx00000000000xx|xxx0000000000000000xxx000000000000xxx000000000000x|xxxx000000000000000xxx000000000000xxx0000000000000|xxxxx00000000000000000000000000000xxx0000000000000|xxxxxx0000000000000xxx000000000000xxxx000000000000|xxxxxxx000000000000xxx000000000000xxxxxxxxxx0xxxxx|xxxxxxxx00000000000xxx000000000000xxxxxxxxxx0xxxxx|xxxxxxxxx0000000000xxx0000000000000xxxxxxxxx0xxxxx|xxxxxxxxxx000000000xxxxxxxx0000000000000000000000x|xxxxxxxxxxx00000000xxxxxxxxx00000000000000000000xx|xxxxxxxxxxxx00000000xxxxxxxx0000000000000000000xxx|xxxxxxxxxxxxx000000000000xxx000000000000000000xxxx|xxxxxxxxxxxxxx00000000000xxx00000000000000000xxxxx|xxxxxxxxxxxxxxx0000000000xxx0000000000000000xxxxxx|xxxxxxxxxxxxxxxx000000000xxx000000000000000xxxxxxx|xxxxxxxxxxxxxxxxx00000000xxx00000000000000xxxxxxxx|xxxxxxxxxxxxxxxxxx0000000xxx0000000000000xxxxxxxxx|xxxxxxxxxxxxxxxxxxx000000xxx000000000000xxxxxxxxxx|xxxxxxxxxxxxxxxxxxxx00000xxx00000000000xxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxx0000xxx0000000000xxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxx000xxx000000000xxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxx00xxx00000000xxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxx0xxx0000000xxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxxxxx000000xxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxxxxx00000xxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxxxxx0000xxxxxxxxxxxxxxxxxx|".to_string();
        }

        if map_id == 3 {
            return "xxxxxxxxxxxxxxxxxxx00000xxxxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxx0000000xxxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxx000000000xxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxx00000000000xxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxx000000000000xxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxx0000000000000xxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxx00000000000xxxxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxx00000000000000xxxx0xxxxxxxxxxxxxxxxxxx|xxxxxxxxxxx0000000000000000xxx00xxxxxxxxxxxxxxxxxx|xxxxxxxxxx00000000000000000xxx0000xxxxxxxxxxxxxxxx|xxxxxxxxx000000000000000000xxx00000xxxxxxxxxxxxxxx|xxxxxxxx0000000000000000000xxx00000xxxxxxxxxxxxxxx|xxxxxxx00000000000000000000xxx000000xxxxxxxxxxxxxx|xxxxxx000000000000000000000xxx0000000xxxxxxxxxxxxx|xxxxx0000000000000000000000xxx00000000xxxxxxxxxxxx|xxxx00000000000000000000000xxx000000000xxxxxxxxxxx|xxx000000000000000000000000xxx0000000000xxxxxxxxxx|xx0000000000000000000000000xxx00000000000xxxxxxxxx|x00000000000000000000000000xxx000000000000xxxxxxxx|000000000000000000000000000xxx0000000000000xxxxxxx|000000000000000000000000000xxx00000000000000xxxxxx|000000000000000000000000000xxx000000000000000xxxxx|0000000000000000000000000000000000000000000000xxxx|x0000000000000000000000000000000000000000000000xxx|xx00000000000000000000000000x0000000000000000000xx|xxx000000000000000000000000xxx0000000000000000000x|xxxx00000000000000000000000x0000000000000000000000|xxxxx000000000000000000000000000000000000000000000|xxxxxx00000000000000000000000000000000000000000000|xxxxxxx00000000000000000000xxx00000000000000000000|xxxxxxxx0000000000000000000xxx00000000000000000000|xxxxxxxxx000000000000000000xxx00000000000000000000|xxxxxxxxxx00000000000000000xxx0000000000000000000x|xxxxxxxxxxx0000000000000000xxx000000000000000000xx|xxxxxxxxxxxx000000000000000xxx00000000000000000xxx|xxxxxxxxxxxxx00000000000000xxx0000000000000000xxxx|xxxxxxxxxxxxxx0000000000000xxx000000000000000xxxxx|xxxxxxxxxxxxxxx000000000000xxx00000000000000xxxxxx|xxxxxxxxxxxxxxxx00000000000xxx0000000000000xxxxxxx|xxxxxxxxxxxxxxxxx0000000000xxx000000000000xxxxxxxx|xxxxxxxxxxxxxxxxxx000000000xxx00000000000xxxxxxxxx|xxxxxxxxxxxxxxxxxxx00000000xxx0000000000xxxxxxxxxx|xxxxxxxxxxxxxxxxxxxx0000000xxx000000000xxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxx000000xxx00000000xxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxx00000xxx0000000xxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxx0000xxx000000xxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxx000xxx00000xxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxx00xxx0000xxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxxx0xxx000xxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|".to_string();
        }

        if map_id == 7 {
            return "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxx0xxxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxx00xxxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxx000xxxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxx0000xxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxx0xxxxxxx00000xxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxx00xxxxxxx000000xxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxx000xxxxxxx0000000xxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxx0000xxxxxxx00000000xxxxxxxxxxxxxxxxxx|xxxxxxxxxxxx00000xxxxxxx000000000xxxxxxxxxxxxxxxxx|xxxxxxxxxxx000000xxxxxxx0000000000xxxxxxxxxxxxxxxx|xxxxxxxxxx0000000xxxxxxx00000000000xxxxxxxxxxxxxxx|xxxxxxxxx00000000xxxxxxx00000000000xxxxxxxxxxxxxxx|xxxxxxxx000000000xxxxxxx00000000000xxxxxxxxxxxxxxx|xxxxxxx0000000000xxxxxxx00000000000xxxxxxxxxxxxxxx|xxxxxx00000000000xxxxxxx00000000000xxx0xxxxxxxxxxx|xxxxx000000000000xxxxxxx00000000000xxx00xxxxxxxxxx|xxxx0000000000000xxxxxxx00000000000xxx000xxxxxxxxx|xxx00000000000000000000000000000000xxx0000xxxxxxxx|0x00000000000000000000000000000000000000000xxxxxxx|x0000000000000000xxx0xxx00000000000xxx000000xxxxxx|00000000000000000xx000xx00000000000xxx0000000xxxxx|00000000000000000000x00000000000000xxx00000000xxxx|x000000000000000000xxx0000000000000xxx000000000xxx|xx000000000000000000000000000000000xxx0000000000xx|xxx00000000000000000000000000000000xxx00000000000x|xxxx000000000000000x0x0000000000000xxx000000000000|xxxxx00000000000000xxx0000000000000xxx000000000000|xxxxxx0000000000000xxx0000000xxx0xxxxx000000000000|xxxxxxx000000000000xxx000000xxxx0xxxxx000000000000|xxxxxxxx00000000000xxx000000xxxx0xxxx0000000000000|xxxxxxxxx0000000000xxx000000xxx0000000000000000000|xxxxxxxxxx000000000xxx000000xxx000000000000000000x|xxxxxxxxxxx00000000xxx000000xxx00000000000000000xx|xxxxxxxxxxxx0000000xxx000000xxx0000000000000000xxx|xxxxxxxxxxxxx000000xxxxxxxxxxxx000000000000000xxxx|xxxxxxxxxxxxxx00000xxxxxxxxxxxx00000000000000xxxxx|xxxxxxxxxxxxxxx00000xxxxxxxxxxx0000000000000xxxxxx|xxxxxxxxxxxxxxxx000000000000xxx000000000000xxxxxxx|xxxxxxxxxxxxxxxxx0000000000000000000000000xxxxxxxx|xxxxxxxxxxxxxxxxxx00000000000000000000000xxxxxxxxx|xxxxxxxxxxxxxxxxxxx0000000000x0000000000xxxxxxxxxx|xxxxxxxxxxxxxxxxxxxx00000000xxx00000000xxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxx0000000xxx0000000xxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxx000000x00000000xxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxx0000000000000xxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxx00000000000xxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxx000xx0000xxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxxx00xxxxxxxxxxxxxxxxxxxxxx|xxxxxxxxxxxxxxxxxxxxxxxxxxx0xxxxxxxxxxxxxxxxxxxxxx|".to_string();
        }

        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|".to_string()
            + "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxx00000000xxxxxxxxxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxx00000000000xxxxxxxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxxxxxx000000000000000xxxxxxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxxxxx00000000000000000xxxxxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxxxx0000000000000000000xxxxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxxx000000000000000000000xxxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxx00000000000000000000000xxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxx0000000000000000000000000xxxxxxxxxxxxxxxx|"
            + "xxxxxxxx000000000000000000000000000xxxxxxxxxxxxxxx|"
            + "xxxxxxx00000000000000000000000000000xxxxxxxxxxxxxx|"
            + "xxxxxx0000000000000000000000000000000xxxxxxxxxxxxx|"
            + "xxxxx000000000000000000000000000000000xxxxxxxxxxxx|"
            + "xxxxx0000000000000000000000000000000000xxxxxxxxxxx|"
            + "xxxxx00000000000000000000000000000000000xxxxxxxxxx|"
            + "xxxxx000000000000000000000000000000000000xxxxxxxxx|"
            + "xxxx00000000000000000000000000000000000000xxxxxxxx|"
            + "xxxx000000000000000000000000000000000000000xxxxxxx|"
            + "xxxx0000000000000000000000000000000000000000xxxxxx|"
            + "xxxx00000000000000000000000000000000000000000xxxxx|"
            + "0xxx000000000000000000000000000000000000000000xxxx|"
            + "xxxx000000000000000000000000000000000000000000xxxx|"
            + "xxxx0000000000000000000000000000000000000000000xxx|"
            + "xxxx0000000000000000000000000000000000000000000xxx|"
            + "xxxx0000000000000000000000000000000000000000000xxx|"
            + "xxxxx000000000000000000000000000000000000000000xxx|"
            + "xxxxxx00000000000000000000000000000000000000000xxx|"
            + "xxxxxxx0000000000000000000000000000000000000000xxx|"
            + "xxxxxxxx000000000000000000000000000000000000000xxx|"
            + "xxxxxxxxx0000000000000000000000000000000000000xxxx|"
            + "xxxxxxxxxx000000000000000000000000000000000000xxxx|"
            + "xxxxxxxxxxx0000000000000000000000000000000000xxxxx|"
            + "xxxxxxxxxxxx00000000000000000000000000000000xxxxxx|"
            + "xxxxxxxxxxxxx000000000000000000000000000000xxxxxxx|"
            + "xxxxxxxxxxxxxx0000000000000000000000000000xxxxxxxx|"
            + "xxxxxxxxxxxxxxx00000000000000000000000000xxxxxxxxx|"
            + "xxxxxxxxxxxxxxxx0000000000000000000000000xxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxx00000000000000000000000xxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxx0000000000000000000000xxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxxx00000000000000000000xxxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxxxx000000000000000000xxxxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxxxxx0000000000000000xxxxxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxxxxxxx0000000000000xxxxxxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxxxxxxxx000000000xxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxxxxxxxxxx000000xxxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|"
            + "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx|"
    }


    /// Mirrors `getMap(int)`.
    pub fn get_map(&self, map_id: i32) -> Option<SnowStormMap> {
        self.snow_storm_map_maps.get(&map_id).cloned()
    }
}
