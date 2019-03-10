// rhc 20190118 .. unused use std::borrow::BorrowMut;
// rhc 20190118 .. unused use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{thread, time};

extern crate libloading;
use self::libloading::{Library, Symbol};

use crate::loggable;
use crate::plugin;
use crate::net::router::{Inlet, Outlet};
use crate::timing::LoopTimer;
use crate::Loggable;
use crate::net::{CanHandleMessages, Message, Routable};
use crate::Plugin;

#[derive(Debug)] // rhc 20190118
pub enum RunMode {
    Thread,
    Process,
}

const DEFAULT_RUN_MODE: RunMode = RunMode::Thread;

#[derive(Debug)] // rhc 20190118
pub struct PluginHost {
    library: Option<Library>,
    inner: Arc<Mutex<PluginHostContext>>,
    inlet: Option<Inlet>,
    outlet: Option<Outlet>,
}

impl CanHandleMessages for PluginHost {
    fn can_handle(&self, handle: String) -> bool {
        let mut loc_self = self.inner.lock().unwrap();
        loc_self.can_handle(handle)
    }

    fn handle(&mut self, handle: String, m: Message) -> Option<Message> {
        let mut loc_self = self.inner.lock().unwrap();
        loc_self.handle(handle, m)
    }
}

impl Routable for PluginHost {
    fn set_inlet(&mut self, inlet: Inlet) {
        let mut loc_self = self.inner.lock().unwrap();
        loc_self.set_inlet(inlet);
    }

    fn set_outlet(&mut self, outlet: Outlet) {
        let mut loc_self = self.inner.lock().unwrap();
        loc_self.set_outlet(outlet);
    }

    fn get_handle(&self) -> String {
        let mut loc_self = self.inner.lock().unwrap();
        loc_self.get_handle()
    }
}

impl PluginHost {
    pub fn new() -> PluginHost {
        PluginHost {
            library: None,
            inner: Arc::new(Mutex::new(PluginHostContext::new())),
            inlet: None,
            outlet: None,
        }
    }

    pub fn set_run_mode(&mut self, rm: RunMode) {
        let mut loc_self = self.inner.lock().unwrap();
        loc_self.set_run_mode(rm);
    }

    pub fn init(&mut self) {}

    pub fn start(&mut self) {
        let local_self = self.inner.clone();
        thread::spawn(move || {
            let mut lck = local_self.lock().unwrap();
            lck.main();
        });
    }

    pub fn test(&mut self) {
        let mut loc_self = self.inner.lock().unwrap();
        match &mut loc_self.pg {
            None => (),
            Some(p) => p.test(),
        };
    }

    pub fn load(path: String) -> Result<PluginHost, String> {
        let mut pgh = PluginHost::new();
        unsafe {
            match pgh._load_plugin(path.clone()) {
                Ok(_pg) => Ok(pgh),
                Err(_e) => Err(format!("Unable to load plugin '{}'", path).to_string()),
            }
        }
    }

    unsafe fn _load_plugin(&mut self, path: String) -> Result<(), String> {
        tesys_log!("Loading Plugin: {}", path);

        let lib = Library::new(path);
        match lib {
            Ok(l) => {
                self.library = Some(l);
                match &self.library {
                	None => Err(format!("Unable to get _create_plugin() from plugin: Library property of PluginHost is None. ").to_string()),
                	Some(lib) => match lib.get(plugin::PLUGIN_CREATE_SYMBOL) {
	                    Ok(sym) => {
	                        let func = sym as Symbol<plugin::PluginCreate>;
	                        let pg_raw = func();
							let mut loc_self = self.inner.lock().unwrap();
	                       	loc_self.set_plugin(Box::from_raw(pg_raw));
	                        Ok(())
	                    }
	                    Err(_e) => Err(format!("Unable to load _create symbol: {}", _e).to_string()),
	                }
                }
            }
            Err(_e) => Err(format!("Unable to load library file: {}", _e).to_string()),
        }
    }
}

#[derive(Loggable)]
#[derive(Debug)] // rhc 20190118
struct PluginHostContext {
    run_mode: RunMode,
    pub pg: Option<Box<Plugin>>,
    do_run: bool,
    inlet: Option<Mutex<Inlet>>,
    outlet: Option<Mutex<Outlet>>,
}

impl PluginHostContext {
    pub fn new() -> PluginHostContext {
        PluginHostContext {
            run_mode: DEFAULT_RUN_MODE,
            pg: None,
            do_run: false,
            inlet: None,
            outlet: None,
        }
    }

    pub fn main(&mut self) {
        let mut lt = LoopTimer::new(60); // Target polling rate of 60 iter / sec.

        tesys_log!(Self, "Launched thread.");

        while self.do_run {
            lt.start();
            lt.end();
        }
    }

    pub fn set_run_mode(&mut self, rm: RunMode) {
        self.run_mode = rm;
    }

    pub fn set_plugin(&mut self, pg: Box<Plugin>) {
        self.pg = Some(pg);
    }
}

impl CanHandleMessages for PluginHostContext {
    fn can_handle(&self, handle: String) -> bool {
        match &self.pg {
            Some(pg) => pg.can_handle(handle),
            None => false,
        }
    }

    fn handle(&mut self, handle: String, m: Message) -> Option<Message> {
        match &mut self.pg {
            Some(pg) => pg.handle(handle, m),
            None => None,
        }
    }
}

impl Routable for PluginHostContext {
    fn set_inlet(&mut self, inlet: Inlet) {
        self.inlet = Some(Mutex::new(inlet));
    }

    fn set_outlet(&mut self, outlet: Outlet) {
        self.outlet = Some(Mutex::new(outlet));
    }

    fn get_handle(&self) -> String {
        "plugin".to_string()
    }
}
