use crate::common::MapPoint;
use crate::npc_config::{BaseNpc, NpcConfig, NpcId};

use super::task::{IdleCursorTask, Task};
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
            return;
        }

        match state
            .npc_config
            .get(&self.npc_id)
            .task_kind
        {
            super::task::TaskKind::Cursor => self.set_idle_cursor_task(),
            super::task::TaskKind::Soldier => todo!(),
            super::task::TaskKind::Worker => todo!(),
        }
    }

    pub fn assign(&mut self, task: Box<dyn Task>) {
        task.assign(self);
    }

    fn set_idle_cursor_task(&mut self) {
        self.task = Box::new(IdleCursorTask {});
    }
}
