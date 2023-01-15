use bevy::ecs::component;
use bevy_codegen::model::{BevyModel, Component};
use undo::{Action, History};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PotooEvent {
    Component(Component),
}

#[derive(Clone, Debug)]
pub struct PotooEvents(pub PotooEvent);

impl ProjectModel {
    pub fn apply(&mut self, add: PotooEvents) {
        self.history.apply(&mut self.model, add);
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
            PotooEvent::Component(component) => {
                target.components.push(component.clone());
            }
        };
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        match self.0 {
            PotooEvent::Component(_) => target.components.pop(),
        };
    }
}

#[derive(Debug)]
pub struct ProjectModel {
    pub model: BevyModel,
    pub history: History<PotooEvents>,
}
