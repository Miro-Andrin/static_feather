use std::{any::TypeId, collections::HashSet};
use crate::{PluginName, type_info::TypeInfo};
use bevy_ecs::prelude::World;

// A boxed version of ResouceInit trait, that removes the need
// for generics.
pub struct ResourceInitializer {
    pub plugin_name: PluginName,
    func: Box<dyn Fn(&mut bevy_ecs::world::World)>,
    requires: Vec<TypeInfo>,
    creates: Vec<TypeInfo>,
}

impl ResourceInitializer {
    // If two resource initializers create the same type, then this is possibly
    // a conflic between plugins that we want to detect. However if we have the Fn(&A,&B) -> A
    // case where the output is part of the input, then we assume this to be fine.
    pub fn init_overlap(&self, other: &Self) -> Vec<(TypeInfo, PluginName, PluginName)> {
        let mut result = Vec::new();
        for x in &self.requires {
            for y in &other.requires {
                if x == y {
                    if !(self.creates.contains(x) || other.creates.contains(x)) {
                        result.push((x.clone(), self.plugin_name, other.plugin_name))
                    }
                }
            }
        }
        result
    }

    pub fn all_requires_in(&self, set: &HashSet<TypeId>) -> bool {
        self.requires.iter().all(|x| set.contains(&x.id))
    }
    
    pub fn call(&self, world: &mut World) {
        (self.func)(world)
    }

    pub fn required(&self) -> Vec<TypeInfo> {
        self.requires.clone()
    }

    pub fn creates(&self) -> Vec<TypeInfo> {
        self.creates.clone()
    }
    
    pub fn creates_ids(&self) -> Vec<TypeId> {
        self.creates.iter().map(|x| x.id.clone()).collect()
    }
}

pub trait ResourceInit<Params, Result> {
    fn init(&self, world: &mut World);
    fn consumes() -> Vec<TypeInfo>;
    fn produces() -> Vec<TypeInfo>;
    fn to_box(self, name: &'static str) -> ResourceInitializer;
}

impl<F, A, B, R> ResourceInit<(A, B), R> for F
where
    F: (Fn(&A, &B) -> R) + 'static,
    A: 'static + Send + Sync,
    B: 'static + Send + Sync,
    R: 'static + Send + Sync,
{
    fn init(&self, world: &mut World) {
        let a = world.get_resource().unwrap();
        let b = world.get_resource().unwrap();
        let r = (self)(a, b);
        world.insert_resource(r);
    }

    fn consumes() -> Vec<TypeInfo> {
        vec![TypeInfo::of::<A>(), TypeInfo::of::<B>()]
    }

    fn produces() -> Vec<TypeInfo> {
        vec![TypeInfo::of::<R>()]
    }

    fn to_box(self, name: &'static str) -> ResourceInitializer {
        ResourceInitializer {
            plugin_name: name,
            func: Box::new(move |x| self.init(x)),
            requires: Self::consumes(),
            creates: Self::produces(),
        }
    }
}

impl<F, A, B, C, R> ResourceInit<(A, B, C), R> for F
where
    F: 'static + Fn(&A, &B, &C) -> R,
    A: 'static + Send + Sync,
    B: 'static + Send + Sync,
    C: 'static + Send + Sync,
    R: 'static + Send + Sync,
{
    fn init(&self, world: &mut World) {
        let a = world.get_resource().unwrap();
        let b = world.get_resource().unwrap();
        let c = world.get_resource().unwrap();
        let r = (self)(a, b, c);
        world.insert_resource(r);
    }

    fn consumes() -> Vec<TypeInfo> {
        vec![
            TypeInfo::of::<A>(),
            TypeInfo::of::<B>(),
            TypeInfo::of::<C>(),
        ]
    }

    fn produces() -> Vec<TypeInfo> {
        vec![TypeInfo::of::<R>()]
    }

    fn to_box(self, name: &'static str) -> ResourceInitializer {
        ResourceInitializer {
            plugin_name: name,
            func: Box::new(move |x| self.init(x)),
            requires: Self::consumes(),
            creates: Self::produces(),
        }
    }
}

impl<F, A, R> ResourceInit<(A,), R> for F
where
    F: 'static + Fn(&A) -> R,
    A: 'static + Send + Sync,
    R: 'static + Send + Sync,
{
    fn init(&self, world: &mut World) {
        let a = world.get_resource().unwrap();
        let r = (self)(a);
        world.insert_resource(r);
    }

    fn consumes() -> Vec<TypeInfo> {
        vec![TypeInfo::of::<A>()]
    }

    fn produces() -> Vec<TypeInfo> {
        vec![TypeInfo::of::<R>()]
    }

    fn to_box(self, name: &'static str) -> ResourceInitializer {
        ResourceInitializer {
            plugin_name: name,
            func: Box::new(move |x| self.init(x)),
            requires: Self::consumes(),
            creates: Self::produces(),
        }
    }
}
impl<F, R> ResourceInit<(), R> for F
where
    F: 'static + Fn() -> R,
    R: 'static + Send + Sync,
{
    fn init(&self, world: &mut World) {
        let r = (self)();
        world.insert_resource(r);
    }

    fn consumes() -> Vec<TypeInfo> {
        vec![]
    }

    fn produces() -> Vec<TypeInfo> {
        vec![TypeInfo::of::<R>()]
    }

    fn to_box(self, name: &'static str) -> ResourceInitializer {
        ResourceInitializer {
            plugin_name: name,
            func: Box::new(move |x| self.init(x)),
            requires: Self::consumes(),
            creates: Self::produces(),
        }
    }
}
