use feather_plugin::Plugin;



pub fn init_plugin() -> Plugin {
    Plugin::new("feather_core_protocol_connection")
    .add_resource(|| PacketReciver())
    .add_resource(|| PacketSender())
}

pub struct PacketReciver();
pub struct PacketSender();