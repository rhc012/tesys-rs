extern crate slotmap;
// use slotmap::{SlotMap, DefaultKey};
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::net::{Message, Routable};

pub type Inlet = Receiver<Message>;
pub type Outlet = Sender<Message>;

#[derive(Debug)] // rhc 20190118
pub struct Router {
    _receiver: Inlet,
    _sender: Outlet,
    outlets: HashMap<String, Outlet>,
    root: Option<Inlet>,
}

impl Router {
    pub fn new() -> Router {
        let (sender, receiver) = channel();
        Router {
            _sender: sender,
            _receiver: receiver,
            outlets: HashMap::new(),
            root: None,
        }
    }

    pub fn register<T: Routable>(&mut self, item: &mut T) {
        // First we spawn a new channel and pass the Receiver to to the Routable object.
        let (sender, receiver) = channel();
        item.set_inlet(receiver);

        // Next we insert the outlet from the router into our outlets HashMap.
        self.outlets.insert(item.get_handle(), sender);
    }

    pub fn set_root(&mut self, root_inlet: Inlet) {
        self.root = Some(root_inlet);
    }

    pub fn poll(&mut self) {
        let mut queue = VecDeque::<Message>::new();

        // First we collect all of the messages waiting for dispatch
        for inl in self._receiver.try_iter() {
            queue.push_back(inl);
        }

        // Now we dispatch them
        for m in queue.iter() {
            self.dispatch(m.clone());
        }
    }

    pub fn dispatch(&mut self, m: Message) {
        println!("{:?}", m);
    }
}
