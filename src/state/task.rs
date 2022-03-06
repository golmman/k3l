use std::fmt::Display;

use super::npc::Npc;
use super::State;

pub mod goto;
pub mod idle;

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

#[derive(Debug)]
pub enum TaskKind {
    Cursor,
    Soldier,
    Worker,
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
        Some(self.cmp(other))
    }
}

impl Ord for Box<dyn Task> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_priority()
            .cmp(&other.get_priority())
    }
}

impl Display for Box<dyn Task> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.get_priority(), self.get_name())
    }
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
