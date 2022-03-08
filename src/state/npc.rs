use super::task::idle::IdleCursorTask;
use super::task::Task;
use super::State;
use crate::common::MapPoint;
use crate::npc_config::NpcId;

#[derive(Clone)]
pub struct Npc {
    pub animation: NpcAnimationId,
    pub npc_id: NpcId,
    pub pos: MapPoint,
    pub task: Box<dyn Task>,
}

#[derive(Clone)]
pub enum NpcAnimationId {
    Idle,
    Walk,
    Run,
    Attack,
}

#[derive(Debug)]
pub enum NpcClass {
    Debug,
    Soldier,
    Worker,
}

impl From<&str> for NpcClass {
    fn from(kind: &str) -> Self {
        match kind {
            "debug" => NpcClass::Debug,
            "soldier" => NpcClass::Soldier,
            "worker" => NpcClass::Worker,
            _ => panic!("TaskKind '{kind}' unknown."),
        }
    }
}

impl From<String> for NpcClass {
    fn from(kind: String) -> Self {
        NpcClass::from(kind.as_str())
    }
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
            .npc_class
        {
            NpcClass::Debug => self.set_idle_cursor_task(),
            NpcClass::Soldier => todo!(),
            NpcClass::Worker => todo!(),
        }
    }

    pub fn assign(&mut self, task: Box<dyn Task>) {
        task.assign(self);
    }

    fn set_idle_cursor_task(&mut self) {
        self.task = Box::new(IdleCursorTask {});
    }
}
