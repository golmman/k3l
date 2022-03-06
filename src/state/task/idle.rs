use super::Action;
use super::Task;
use crate::state::npc::Npc;
use crate::state::State;

#[derive(Clone)]
pub struct IdleCursorTask {}

pub struct IdleCursorAction {}

impl Action for IdleCursorAction {
    fn execute(&self, _npc: &mut Npc, _state: &mut State) {}
}

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
