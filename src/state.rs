use self::map::Map;
use self::map::TilePos;
use crate::common::MapPoint;
use crate::common::ScreenPoint;
use crate::common::TILE_SIZE;
use crate::npc_config::NpcConfig;
use crate::screen::Pixel;
use crate::screen::Sprite;
use crate::tile_config::BaseTile;
use crate::tile_config::TileConfig;

mod map;

pub struct State {
    pub astar_start: MapPoint,
    pub astar_goal: MapPoint,
    pub astar_path: Option<Vec<MapPoint>>,

    pub show_debug_info: bool,

    pub cursor_pos: MapPoint,
    pub elapsed_time: u64,
    pub map: Map,
    pub map_pos: MapPoint,

    pub npc_config: NpcConfig,
    pub tile_config: TileConfig,

    pub screen_size: MapPoint,
}

impl State {
    pub fn new() -> Self {
        let elapsed_time = 0;
        let tile_config = TileConfig::from_file("tile_config.toml");
        let npc_config = NpcConfig::from_file("npc_config.toml");
        let map = Map::from_file("example_map.toml", &tile_config);
        let map_pos = MapPoint::new(24, 1);

        Self {
            astar_start: MapPoint::new(0, 0),
            astar_goal: MapPoint::new(0, 0),
            astar_path: None,

            show_debug_info: true,

            cursor_pos: MapPoint::new(0, 0),
            elapsed_time,
            map,
            map_pos,

            npc_config,
            tile_config,

            screen_size: MapPoint::new(0, 0),
        }
    }

    pub fn get_map_sprite(&self) -> Sprite {
        let mut pixels = Vec::new();
        let width = TILE_SIZE.width() * self.map.size.width();
        let height = self.map.size.height();

        for tile in &self.map.tiles {
            let tile_id = tile.tile_id;
            let tile_string_alternative_id = tile.tile_string_alternative_id;

            let tile_frames = &self
                .tile_config
                .get(tile_id)
                .tile_strings[tile_string_alternative_id]
                .frames;
            let frame = (self.elapsed_time % tile_frames.len() as u64) as usize;

            let tile_str = &tile_frames[frame];
            let color = self.tile_config.get(tile_id).color;

            for ch in tile_str.chars() {
                pixels.push(Pixel { ch, color });
            }
        }

        Sprite {
            pixels,
            size: ScreenPoint::new(width, height),
        }
    }

    pub fn resize(&mut self, screen_size: &MapPoint) {
        self.screen_size = screen_size.clone();
    }

    pub fn elapse_time(&mut self) {
        self.elapsed_time += 1;
    }

    pub fn get_base_tile_at(&self, point: &MapPoint) -> Option<&BaseTile> {
        if let Some(tile) = self.map.get_tile(point) {
            return Some(self.tile_config.get(tile.tile_id));
        }
        None
    }

    pub fn toggle_debug_info(&mut self) {
        self.show_debug_info = !self.show_debug_info;
    }

    pub fn set_astar_start(&mut self) {
        self.astar_start.x = self.cursor_pos.x - self.map_pos.x;
        self.astar_start.y = self.cursor_pos.y - self.map_pos.y;
    }

    pub fn set_astar_goal(&mut self) {
        self.astar_goal.x = self.cursor_pos.x - self.map_pos.x;
        self.astar_goal.y = self.cursor_pos.y - self.map_pos.y;
    }

    pub fn move_map_left(&mut self) {
        self.map_pos.x -= 1;
    }

    pub fn move_map_right(&mut self) {
        self.map_pos.x += 1;
    }

    pub fn move_map_up(&mut self) {
        self.map_pos.y -= 1;
    }

    pub fn move_map_down(&mut self) {
        self.map_pos.y += 1;
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor_pos.x -= 1;

        if self.cursor_pos.x <= 0 {
            self.cursor_pos.x = 0;
            self.move_map_right();
        }
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor_pos.x += 1;

        if self.cursor_pos.x >= self.screen_size.width() - 1 {
            self.cursor_pos.x = self.screen_size.width() - 1;
            self.move_map_left();
        }
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor_pos.y -= 1;

        if self.cursor_pos.y <= 0 {
            self.cursor_pos.y = 0;
            self.move_map_down();
        }
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor_pos.y += 1;

        if self.cursor_pos.y >= self.screen_size.height() - 1 {
            self.cursor_pos.y = self.screen_size.height() - 1;
            self.move_map_up();
        }
    }
}

pub fn get_shortest_path(
    start: &MapPoint,
    goal: &MapPoint,
    map: &Map,
    tile_config: &TileConfig,
) -> Option<Vec<MapPoint>> {
    let path = pathfinding::prelude::astar(
        start,
        |p| successors(p, map, tile_config),
        |p| heuristic(p, goal),
        |p| p == goal,
    );

    path.map(|p| p.0)
}

fn successors(point: &MapPoint, map: &Map, tile_config: &TileConfig) -> Vec<(MapPoint, u32)> {
    let neigh_tiles: Vec<TilePos> = map
        .get_neighborhood4(point)
        .filter_traversable(tile_config)
        .into();

    neigh_tiles
        .iter()
        .map(|t| (t.pos.clone(), 1))
        .collect()
}

fn heuristic(point: &MapPoint, goal: &MapPoint) -> u32 {
    (pathfinding::prelude::absdiff(point.x, goal.x)
        + pathfinding::prelude::absdiff(point.y, goal.y)) as u32
}
