use bevy_codegen::model::{BevyModel, Component};
use undo::{Action, History};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PotooEvent {
    AddComponent(Component),
    RemoveComponent(Component),
}

#[derive(Clone, Debug)]
pub struct PotooEvents(pub PotooEvent);

impl ProjectModel {
    pub fn apply(&mut self, event: PotooEvents) {
        self.history.apply(&mut self.model, event);
    }

    pub fn redo(&mut self) {
        self.history.redo(&mut self.model);
    }

    pub fn undo(&mut self) {
        self.history.undo(&mut self.model);
    }
}

impl Action for PotooEvents {
    type Target = BevyModel;
    type Output = ();

    fn apply(&mut self, target: &mut Self::Target) -> Self::Output {
        match &self.0 {
            PotooEvent::AddComponent(c) => target.components.push(c.clone()),
            PotooEvent::RemoveComponent(_) => {
                target.components.pop();
            }
        };
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        match &self.0 {
            PotooEvent::AddComponent(_) => target.components.pop(),
            PotooEvent::RemoveComponent(c) => {
                target.components.push(c.clone());
                Some(c.clone())
            }
        };
    }

    fn redo(&mut self, target: &mut Self::Target) -> Self::Output {
        self.apply(target)
    }
}

#[derive(Debug)]
pub struct ProjectModel {
    pub model: BevyModel,
    pub history: History<PotooEvents>,
}
