use std::collections::HashSet;
use std::rc::Rc;

use self::map::Map;
use self::map::TilePos;
use self::npc::Npc;
use self::npc::NpcClass;
use self::selection::Selection;
use self::task::idle::IdleCursorTask;
use self::task::Task;
use crate::common::MapPoint;
use crate::common::ScreenPoint;
use crate::common::TILE_SIZE;
use crate::npc_config::NpcConfig;
use crate::renderer::debug_info::DEBUG_INFO_PAGE_TOTAL;
use crate::screen::Pixel;
use crate::screen::Sprite;
use crate::tile_config::BaseTile;
use crate::tile_config::TileConfig;

mod map;
pub mod npc;
pub mod selection;
pub mod task;

pub struct State {
    pub astar_start: MapPoint,
    pub astar_goal: MapPoint,
    pub astar_path: Vec<MapPoint>,

    pub selection: Selection,
    pub dig_selection: HashSet<MapPoint>,

    pub debug_info_page: i32,

    pub cursor_pos: MapPoint,
    pub elapsed_time: u64,
    pub map: Rc<Map>,
    pub map_pos: MapPoint,

    pub npcs: Vec<Npc>,

    pub cursor_tasks: Vec<Box<dyn Task>>,
    pub soldier_tasks: Vec<Box<dyn Task>>,
    pub worker_tasks: Vec<Box<dyn Task>>,

    // TODO: state should ideally only contain the information needed for a savefile
    pub npc_config: Rc<NpcConfig>,
    pub tile_config: Rc<TileConfig>,

    pub screen_size: MapPoint,
}

impl State {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let tile_config = Rc::new(TileConfig::from_file("tile_config.toml"));
        let npc_config = Rc::new(NpcConfig::from_file("npc_config.toml"));
        let elapsed_time = 0;
        let map = Rc::new(Map::from_file("example_map.toml", &tile_config));
        let map_pos = MapPoint::new(24, 1);

        let npcs = vec![Npc {
            npc_id: String::from("follower"),
            pos: MapPoint::new(10, 10),
            task: Box::new(IdleCursorTask {}),
        }];

        let cursor_tasks = Vec::new();
        let soldier_tasks = Vec::new();
        let worker_tasks = Vec::new();

        Self {
            astar_start: MapPoint::new(0, 0),
            astar_goal: MapPoint::new(0, 0),
            astar_path: Vec::new(),

            selection: Selection::new(),
            dig_selection: HashSet::new(),

            debug_info_page: 1,

            cursor_pos: MapPoint::new(0, 0),
            elapsed_time,
            map,
            map_pos,

            npcs,

            cursor_tasks,
            soldier_tasks,
            worker_tasks,

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

    // TODO: is it possible to prevent npc cloning here?
    pub fn update_npcs(&mut self) {
        self.cursor_tasks.as_mut_slice().sort();

        for i in 0..self.npcs.len() {
            let mut npc_clone = self.npcs[i].clone();

            match self
                .npc_config
                .get(&npc_clone.npc_id)
                .npc_class
            {
                NpcClass::Debug => {
                    State::assign_appropriate_task(&mut npc_clone, &mut self.cursor_tasks)
                }
                NpcClass::Soldier => {
                    State::assign_appropriate_task(&mut npc_clone, &mut self.soldier_tasks)
                }
                NpcClass::Worker => todo!(),
            }

            npc_clone.execute_next_action(self);

            self.npcs[i] = npc_clone;
        }
    }

    fn assign_appropriate_task(npc: &mut Npc, tasks: &mut Vec<Box<dyn Task>>) {
        for i in 0..tasks.len() {
            if tasks[i].get_priority() > npc.task.get_priority() {
                let task = tasks.swap_remove(i);
                npc.assign(task);
                return;
            }
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

    pub fn debug_info_next_page(&mut self) {
        self.debug_info_page += 1;

        if self.debug_info_page > DEBUG_INFO_PAGE_TOTAL {
            self.debug_info_page = 0;
        }
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

    pub fn toggle_selection(&mut self) {
        if self.selection.pos.is_none() && self.selection.size.is_none() {
            self.selection.pos = Some(&self.cursor_pos - &self.map_pos);
        } else if self.selection.pos.is_some() && self.selection.size.is_some() {
            self.selection.pos = Some(&self.cursor_pos - &self.map_pos);
            self.selection.size = None;
        } else if self.selection.pos.is_some() && self.selection.size.is_none() {
            let pos = self.selection.pos.as_ref().unwrap();
            self.selection.size = Some(&(&self.cursor_pos - pos) - &self.map_pos);
            self.selection.normalize();
            self.selection.size =
                Some(self.selection.size.as_ref().unwrap() + &MapPoint::new(1, 1));

            self.set_dig_selection();
        } else {
            panic!("State selection pos was none but size was some, this should not be possible.");
        }
    }

    fn set_dig_selection(&mut self) {
        if let Selection {
            pos: Some(pos),
            size: Some(size),
        } = &self.selection
        {
            for y in pos.y..(pos.y + size.y) {
                for x in pos.x..(pos.x + size.x) {
                    let base_tile = self.get_base_tile_at(&MapPoint::new(x, y));
                    let minable = base_tile
                        .map(|b| b.minable)
                        .unwrap_or(false);

                    if minable {
                        self.dig_selection
                            .insert(MapPoint::new(x, y));
                    }
                }
            }
        }
    }
}

pub fn get_shortest_path(
    start: &MapPoint,
    goal: &MapPoint,
    map: &Map,
    tile_config: &TileConfig,
) -> Vec<MapPoint> {
    let path = pathfinding::prelude::astar(
        start,
        |p| successors(p, map, tile_config),
        |p| heuristic(p, goal),
        |p| p == goal,
    );

    path.map_or(Vec::new(), |p| p.0)
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
