

use feather_server_builder::ServerBuilder;
use feather_plugin::Plugin;

pub fn init_plugin() -> Plugin {
    Plugin::new("world_edit")
        .add_resource(|x:&usize| -> i32 {(x+1) as i32})
        .add_resource(|| -> usize {1})
        .add_resource(|x:&usize| -> i32 {(x+1) as i32})
        .add_resource_requirement::<usize>()
        // .add_system("world_edit::test_system", location, system)
}

fn main() {
    let mut app = ServerBuilder::new();
    app.add_plugin(init_plugin());

    let x = app.build();
    let value = x.world.get_resource::<usize>();
    assert_eq!(Some(&1), value);

    let value = x.world.get_resource::<i32>();
    assert_eq!(Some(&2), value);
}
