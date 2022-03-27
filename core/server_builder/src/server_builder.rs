use std::collections::HashSet;

use feather_plugin::{Plugin, bevy_ecs::prelude::World, ResourceInitializer, TypeInfo};

use crate::BuiltServer;


pub struct ServerBuilder {
    plugins: Vec<Plugin>,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Plugin) -> &mut Self {
        if self.plugins.iter().any(|x| x.name() == plugin.name()) {
            panic!(
                "Cant have two plugins called the same name: {:?}",
                plugin.name()
            )
        }
        self.plugins.push(plugin);
        self
    }

    pub fn build(mut self) -> BuiltServer {
        let mut world = World::new();
        let resource_inits = self.plugins.iter_mut().flat_map(|x| std::mem::take(&mut x.resource_initialisers)).collect::<Vec<_>>();
        let resource_requirements = self.plugins.iter().map(|x| x.resource_requirements.clone()).reduce(|mut x,y| {
            x.extend(y);
            x
        }).unwrap_or_default();
        Self::run_resorce_inits(&resource_inits,&resource_requirements, &mut world);
        BuiltServer::new(world)
    }


    fn run_resorce_inits(resource_inits: &Vec<ResourceInitializer>,resource_requirements: &HashSet<(&str, TypeInfo)>, world: &mut World) {

        // If two plugins initialize the same resource, then it is proably
        // the case that this can be a cause of conflictis.
        // However if it does something like (A,B) -> A, or A -> A,
        // then it proably does not assume A to be in a speciffic state.
        {
            let mut issues = Vec::new();
            for i in 0..resource_inits.len() {
                for u in i + 1..resource_inits.len() {
                    let ri = &resource_inits[i];
                    let ru = &resource_inits[u];
                    issues.extend(ri.init_overlap(ru))
                }
            }

            for (typeinfo, a, b) in issues {
                println!("----------------------------------------");
                println!("POSSIBLE ISSUE BETWEEN PLUGIN {} AND {}.", a, b);
                println!("Overlapping initialization of resource {}", typeinfo.name);
                println!("----------------------------------------");
            }
        }

        let mut created_types = HashSet::with_capacity(resource_inits.len());
        let mut resource_inits: Vec<&ResourceInitializer> = resource_inits.iter().collect();

        while !resource_inits.is_empty() {
            let mut rest = Vec::new();

            for x in &resource_inits {
                if x.all_requires_in(&created_types) {
                    x.call(world);
                    created_types.extend(x.creates_ids())
                } else {
                    rest.push(*x)
                }
            }

            if resource_inits.len() == rest.len() {
                // No change happend
                for init in resource_inits {
                    let missing_requirements = init
                        .required()
                        .into_iter()
                        .filter(|x| !created_types.contains(&x.id))
                        .collect::<Vec<_>>();

                    print!("-------------------------------------------------------------------------------");
                    println!("ERROR IN PLUGIN {}", init.plugin_name);
                    println!("Unable to initialize the resources {:?}, because it was impossible to initialize:\n\t{:?}", init.creates().iter().map(|x| x.name).collect::<Vec<_>>(), missing_requirements.iter().map(|x| x.name).collect::<Vec<_>>());
                    print!("-------------------------------------------------------------------------------");
                }
                panic!(
                    "#{} RESOURCE INIT FUNCTIONS NOT CALLABLE BECAUSE OF MISSING DEPENDENCY!",
                    rest.len()
                );
            }
            resource_inits = rest;
        }

        // Check that all resource requirements are met
        let mut num_missing_resources = 0;
        for (plugin_name, type_info) in resource_requirements {
            if !created_types.contains(&type_info.id) {
                num_missing_resources += 1;
                println!("------------------------------------------------------------------------------------");
                println!("Critical Error, the required resource {} for the plugin {} could not be initialized!",type_info.name, plugin_name);
                println!("------------------------------------------------------------------------------------");
            }
        }
        
        if num_missing_resources > 0 {
            panic!("Unable to start server because {} resource requirements were not met.",num_missing_resources);
        }
    }
}
