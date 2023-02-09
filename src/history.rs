use bevy_codegen::model::{BevyModel, Component, System};
use undo::{Action, History};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PotooEvent {
    //Add
    AddComponent(Component),
    AddStartupSystem(System),
    AddRunTimeSystem(System),
    //AddComponentToEntity(Entity, Component),

    //Remove
    RemoveComponent(Component),
    RemoveStartupSystem(System),
    RemoveRunTimeSystem(System),

    //Update
    UpdateComponent(Component),
    UpdateStartupSystem(System),
    UpdateRunTimeSystem(System),
}

pub enum ReloadType {
    RequireReload,
    None,
}

pub fn reload_get_type(event: PotooEvent) -> ReloadType {
    match event {
        PotooEvent::UpdateRunTimeSystem(_) => ReloadType::None,
        _ => ReloadType::RequireReload,
    }
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
            PotooEvent::RemoveComponent(c) => target.components.retain(|x| x.name != c.name),
            PotooEvent::AddStartupSystem(s) => target.startup_systems.push(s.clone()),
            PotooEvent::AddRunTimeSystem(s) => target.systems.push(s.clone()),
            PotooEvent::RemoveStartupSystem(s) => {
                target.startup_systems.retain(|x| x.name != s.name)
            }
            PotooEvent::RemoveRunTimeSystem(s) => target.systems.retain(|x| x.name != s.name),
            PotooEvent::UpdateComponent(c) => {
                let index = target
                    .components
                    .iter()
                    .position(|x| *x.name == c.name)
                    .expect("Component with name found");
                let tmp = target.components.remove(index);
                target.components.insert(index, tmp);
            }
            PotooEvent::UpdateStartupSystem(s) => {
                let index = target
                    .startup_systems
                    .iter()
                    .position(|x| *x.name == s.name)
                    .expect("Startup system with name found");
                let tmp = target.startup_systems.remove(index);
                target.startup_systems.insert(index, tmp);
            }
            PotooEvent::UpdateRunTimeSystem(s) => {
                let index = target
                    .systems
                    .iter()
                    .position(|x| *x.name == s.name)
                    .expect("Runtime system with name found");
                let tmp = target.systems.remove(index);
                target.systems.insert(index, tmp);
            }
        };
    }

    fn undo(&mut self, target: &mut Self::Target) -> Self::Output {
        match &self.0 {
            PotooEvent::AddComponent(c) => target.components.retain(|x| x.name != c.name),
            PotooEvent::RemoveComponent(c) => {
                target.components.push(c.clone());
            }
            PotooEvent::AddStartupSystem(s) => target.startup_systems.retain(|x| x.name != s.name),
            PotooEvent::AddRunTimeSystem(s) => target.systems.retain(|x| x.name != s.name),
            PotooEvent::RemoveStartupSystem(s) => target.startup_systems.push(s.clone()),
            PotooEvent::RemoveRunTimeSystem(s) => target.systems.push(s.clone()),
            PotooEvent::UpdateComponent(c) => {
                let index = target
                    .components
                    .iter()
                    .position(|x| *x.name == c.name)
                    .expect("Component with name found");
                let tmp = target.components.remove(index);
                target.components.insert(index, tmp);
            }
            PotooEvent::UpdateStartupSystem(s) => {
                let index = target
                    .startup_systems
                    .iter()
                    .position(|x| *x.name == s.name)
                    .expect("Startup system with name found");
                let tmp = target.startup_systems.remove(index);
                target.startup_systems.insert(index, tmp);
            }
            PotooEvent::UpdateRunTimeSystem(s) => {
                let index = target
                    .systems
                    .iter()
                    .position(|x| *x.name == s.name)
                    .expect("Runtime system with name found");
                let tmp = target.systems.remove(index);
                target.systems.insert(index, tmp);
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
