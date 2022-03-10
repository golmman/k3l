use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use crate::color::Color;
use crate::screen::Animation;
use crate::state::npc::NpcClass;

pub type NpcId = String;

#[derive(Debug)]
pub struct BaseNpc {
    pub color: Color,
    pub id: NpcId,
    pub key: String,
    pub name: String,
    pub npc_class: NpcClass,
    pub walk_delay: i32,
    pub animation_idle: Vec<Animation>,
    pub animation_walk: Vec<Animation>,
}

#[derive(Debug)]
pub struct NpcConfig {
    npcs: HashMap<NpcId, BaseNpc>,
}

impl NpcConfig {
    pub fn get(&self, npc_id: &NpcId) -> &BaseNpc {
        self.npcs.get(npc_id).unwrap()
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let mut npcs = HashMap::new();

        // TODO: replace all unwraps with expect
        let tile_config_string = read_to_string(path).unwrap();
        let tile_config: toml::value::Value = toml::from_str(&tile_config_string).unwrap();
        let tile_confg_table = tile_config.as_table().unwrap();

        for (key, t) in tile_confg_table {
            let base = t["base"].as_table().unwrap();

            let bg_color = base["bg_color"]
                .as_integer()
                .map(|c| c as u8);
            let fg_color = base["fg_color"]
                .as_integer()
                .map(|c| c as u8);
            let color = Color { bg_color, fg_color };

            let id = base["id"]
                .as_str()
                .map(String::from)
                .unwrap();

            let name = base["name"]
                .as_str()
                .map(String::from)
                .unwrap();

            let walk_delay = base["walk_delay"]
                .as_integer()
                .map(|i| i as i32)
                .unwrap();

            let npc_class = NpcClass::from(base["npc_class"].as_str().unwrap());

            let animation = t["animation"].as_table().unwrap();

            let animation_idle = animation["idle"]
                .as_array()
                .unwrap()
                .iter()
                .map(Animation::from)
                .collect();

            let animation_walk = animation["walk"]
                .as_array()
                .unwrap()
                .iter()
                .map(Animation::from)
                .collect();

            npcs.insert(
                id.clone(),
                BaseNpc {
                    color,
                    id,
                    key: key.to_string(),
                    name,
                    npc_class,
                    walk_delay,
                    animation_idle,
                    animation_walk,
                },
            );
        }

        Self { npcs }
    }
}
