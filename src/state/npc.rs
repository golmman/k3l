use crate::common::MapPoint;
use crate::npc_config::{BaseNpc, NpcConfig, NpcId};

use super::task::Task;
use super::State;

#[derive(Clone)]
pub struct Npc {
    pub npc_id: NpcId,
    pub pos: MapPoint,
    pub task: Box<dyn Task>,
}

impl Npc {
    pub fn execute_next_action(&mut self, state: &mut State) {
        if let Some(action) = self.task.next() {
            action.execute(self, state);
        }
    }

    pub fn assign(&mut self, task: Box<dyn Task>) {
        task.assign(self);
    }
}
