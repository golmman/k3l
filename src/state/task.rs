use std::rc::Rc;

use crate::common::MapPoint;
use crate::tile_config::TileConfig;

use super::map::Map;
use super::npc::Npc;
use super::{get_shortest_path, State};

pub trait Action {
    fn execute(&self, npc: &mut Npc, state: &mut State);
}

pub trait Task: TaskClone + Iterator<Item = Box<dyn Action>> {
    fn assign(self: Box<Self>, npc: &mut Npc);
    fn get_name(&self) -> String;
    fn get_priority(&self) -> i32;
}

// Magic necessary for making trait objects clonable
// see: https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
pub trait TaskClone {
    fn clone_box(&self) -> Box<dyn Task>;
}

impl<T> TaskClone for T
where
    T: 'static + Task + Clone,
{
    fn clone_box(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Task> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn Task> {
    fn eq(&self, other: &Self) -> bool {
        self.get_priority() == other.get_priority()
    }
}

impl Eq for Box<dyn Task> {}

impl PartialOrd for Box<dyn Task> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Box<dyn Task> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_priority()
            .cmp(&other.get_priority())
    }
}

#[derive(Debug)]
pub enum TaskKind {
    Cursor,
    Soldier,
    Worker,
}

impl From<&str> for TaskKind {
    fn from(kind: &str) -> Self {
        match kind {
            "cursor" => TaskKind::Cursor,
            "soldier" => TaskKind::Soldier,
            "worker" => TaskKind::Worker,
            _ => panic!("TaskKind '{kind}' unknown."),
        }
    }
}

impl From<String> for TaskKind {
    fn from(kind: String) -> Self {
        TaskKind::from(kind.as_str())
    }
}

pub struct IdleCursorAction {}

impl Action for IdleCursorAction {
    fn execute(&self, npc: &mut Npc, state: &mut State) {}
}

#[derive(Clone)]
pub struct IdleCursorTask {}

impl Task for IdleCursorTask {
    fn assign(self: Box<Self>, npc: &mut Npc) {
        npc.task = self;
    }

    fn get_name(&self) -> String {
        String::from("IdleCursor")
    }

    fn get_priority(&self) -> i32 {
        0
    }
}

impl Iterator for IdleCursorTask {
    type Item = Box<dyn Action>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Box::new(IdleCursorAction {}))
    }
}

pub struct GotoAction {
    next_step: MapPoint,
}

impl Action for GotoAction {
    fn execute(&self, npc: &mut Npc, state: &mut State) {
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
        //let steps = get_shortest_path(start, goal, map, tile_config);

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
