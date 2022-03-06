use std::rc::Rc;

use super::Action;
use super::Task;
use crate::common::MapPoint;
use crate::state::get_shortest_path;
use crate::state::map::Map;
use crate::state::npc::Npc;
use crate::state::State;
use crate::tile_config::TileConfig;

pub struct GotoAction {
    next_step: MapPoint,
}

impl Action for GotoAction {
    fn execute(&self, npc: &mut Npc, _state: &mut State) {
        npc.pos = self.next_step.clone();
    }
}

#[derive(Clone)]
pub struct GotoTask {
    goal: MapPoint,
    tile_config: Rc<TileConfig>,
    map: Rc<Map>,

    steps: Vec<MapPoint>,
    step_index: usize,
}

impl GotoTask {
    pub fn new(goal: MapPoint, map: Rc<Map>, tile_config: Rc<TileConfig>) -> Self {
        Self {
            goal,
            tile_config,
            map,
            steps: Vec::new(),
            step_index: 0,
        }
    }
}

impl Task for GotoTask {
    fn assign(mut self: Box<Self>, npc: &mut Npc) {
        self.steps = get_shortest_path(&npc.pos.clone(), &self.goal, &self.map, &self.tile_config);

        npc.task = self;
    }

    fn get_name(&self) -> String {
        String::from("Goto")
    }

    fn get_priority(&self) -> i32 {
        1
    }
}

impl Iterator for GotoTask {
    type Item = Box<dyn Action>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step_index >= self.steps.len() {
            return None;
        }

        let next_step = self.steps[self.step_index].clone();
        self.step_index += 1;

        Some(Box::new(GotoAction { next_step }))
    }
}
