use feather_plugin::bevy_ecs::{prelude::World};

pub struct BuiltServer {
    pub world: World,
}


impl BuiltServer {

    pub fn new(world: World) -> Self {
        Self {
            world
        }
    } 

    pub fn run_tick(self) {
        self.world;
    }

}


