use crate::common::MapPoint;
use crate::tile_config::TileConfig;

use super::map::Map;
use super::npc::Npc;
use super::{get_shortest_path, State};

pub trait Action {
    fn execute(&self, npc: &mut Npc, state: &mut State);
}

pub trait Task: TaskClone + Iterator<Item = Box<dyn Action>> {
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

struct GotoAction {
    next_step: MapPoint,
}

impl Action for GotoAction {
    fn execute(&self, npc: &mut Npc, state: &mut State) {
        npc.pos = self.next_step.clone();
    }
}

#[derive(Clone)]
struct GotoTask {
    steps: Vec<MapPoint>,
    step_index: usize,
}

impl GotoTask {
    pub fn new(start: &MapPoint, goal: &MapPoint, map: &Map, tile_config: &TileConfig) -> Self {
        let steps = get_shortest_path(start, goal, map, tile_config);

        Self {
            steps,
            step_index: 0,
        }
    }
}

impl Task for GotoTask {
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
