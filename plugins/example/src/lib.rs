#[macro_use]
extern crate tesys;
#[macro_use]
extern crate tesys_derive;
use tesys::Loggable;
use tesys::astrometry::SkyCoordinate;
use tesys::astrometry::frames::ICRS;
use tesys::loggable;
use tesys::{Plugin,Message,CanHandleMessages,Routable};
use tesys::codegen::*;

// We call into the macros to write the extern C functions
// which allow us to easily create and destroy our Rust
// plugin struct. 
tesys_plugin_create!(ExamplePlugin);
tesys_plugin_destroy!(ExamplePlugin);

tesys_plugin!(ExamplePlugin {
    label: String,
    coord: SkyCoordinate<ICRS>,
    handlers: Vec<&'static StaticHandlerInfo<ExamplePlugin>>,
});

impl Plugin for ExamplePlugin {
    tesys_plugin_new!(
        label: "".to_string(),
        coord: SkyCoordinate::<ICRS>::new(0.0, 0.0),
        handlers: vec!(&test_handler_handler),
    );

    fn test(&mut self) {
        tesys_log!(Self, "Test. ");
        tesys_warn!(Self, "{}", self.coord);
        self.coord.coords[0] += 137.6;
        self.coord.coords[1] += 86.3;
        tesys_warn!(Self, "{}", self.coord);
    }
}

impl CanHandleMessages for ExamplePlugin {
    fn can_handle(&self, handle: String) -> bool {
        true
    }

    fn handle(&mut self, handle: String, m: Message) -> Option<Message> {
        None
    }
}

// Start of the handler setup for plugins. 
static test_handler_handler: StaticHandlerInfo<ExamplePlugin>  = StaticHandlerInfo {
    name: "test_handler",
    handler: test_handler,
};

// Experimenting with the handle attribute 
#[handle("test", return=String)]
fn test_handler(_pg: Box<ExamplePlugin>) {
    println!("Testing function call.");
}