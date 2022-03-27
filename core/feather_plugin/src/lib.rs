mod resources;
mod system_location;
mod type_info;

pub use bevy_ecs;
pub use type_info::TypeInfo;
pub use resources::ResourceInitializer;

use std::collections::HashSet;

use bevy_ecs::schedule::{
    ExclusiveSystemDescriptorCoercion, IntoSystemDescriptor, ParallelSystemDescriptorCoercion,
    SystemDescriptor,
};

use crate::{
    resources::ResourceInit,
    system_location::SystemLocation,
};


pub type PluginName = &'static str;

pub struct Plugin {
    name: PluginName,
    pub resource_initialisers: Vec<ResourceInitializer>,
    pub resource_requirements: HashSet<(PluginName,TypeInfo)>,
    pub systems: Vec<(SystemDescriptor, SystemLocation)>,
    pub used_system_labels: HashSet<&'static str>,
}

impl Plugin {
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Panics if name does not follow naming convention.
    /// Adding a plugins to server also panics if the name
    /// is already taken.
    pub fn new(name: &'static str) -> Self {
        if name.chars().any(char::is_whitespace) {
            panic!("The name of a plugin is not allowed to have a space in it.")
        }
        Self {
            name,
            resource_initialisers: Default::default(),
            resource_requirements: Default::default(),
            systems: Default::default(),
            used_system_labels: Default::default(),
        }
    }

    pub fn add_system<Param>(
        mut self,
        label: &'static str,
        location: impl Into<SystemLocation>,
        system: impl IntoSystemDescriptor<Param>,
    ) ->  Self {
        if !label.starts_with(format!("{}::", self.name()).as_str()) {
            panic!("The system label must begin with \"{}::\", aka plugin name followed by :: as a seperator.", self.name());
        }
        if self.used_system_labels.contains(label) {
            panic!("Two systems can't have the same label: {} !",label);
        }
        self.used_system_labels.insert(label);
        let descriptor = match system.into_descriptor() {
            SystemDescriptor::Parallel(x) => SystemDescriptor::Parallel(x.label(label)),
            SystemDescriptor::Exclusive(x) => SystemDescriptor::Exclusive(x.label(label)),
        };  
        self.systems.push((descriptor, location.into()));
        self
    }

    /// This method is how you are suposed to add resources to the becy_ecs::World.
    /// https://bevyengine.org/learn/book/getting-started/resources/
    /// 
    /// Adding resource is done by providing a closure.
    /// ```
    /// pub struct MyResource;
    /// 
    /// let plugin = Plugin::new("my_plugin")
    ///                 .add_resource(|| MyResource)
    /// ```
    /// If you need the contents of one resource to create your resource, then 
    /// you can do the following. 
    /// ```
    /// pub struct Resource1{
    ///     pub a: i32
    /// };
    /// pub struct Resource2 {
    ///     b: i32
    /// };
    /// let plugin = Plugin::new("my_plugin")
    ///             .add_resource(|r1: &Resource1| Resource2{b: r1.a + 1})
    ///             .add_resource(|| Resource1{a : 1});
    /// ```
    /// Notice the & on Resource1, and also notice how the order the resources
    /// are added does not matter. They are actually initialized in an "arbitrary"
    /// orderd by the server once the server starts. 
    /// 
    pub fn add_resource<Params, Result>(
        mut self,
        init: impl ResourceInit<Params, Result>,
    ) -> Self {
        self.resource_initialisers.push(init.to_box(self.name()));
        self
    }

    /// Add this to your plugin if it needs a speciffic resource to function.
    /// but you dont want to be responsible for initializing it. 
    /// Anytime you do world.get_resource::<T>().unwrap(), you should
    /// have a add_resource_rquirement<T>. 
    pub fn add_resource_requirement<T>(mut self)-> Self where T: 'static {
        self.resource_requirements.insert((self.name, TypeInfo::of::<T>()));
        self
    }

    pub fn add_event<E>(&mut self) -> &mut Self {
        todo!()
    }
}

