use std::collections::HashMap;

use sdl2::event::Event;
#[feature(core_intrinsics)]
use std::intrinsics;

type Callback = Box<(Fn(&Event) -> Result<(), String>)>;

pub fn get_type<T>(_: &T) -> String {
    unsafe { intrinsics::type_name::<T>() }.into()
}

pub struct Event_Type {
    evt: Event,
}

pub struct Event_Dispatcher {
    events: HashMap<String, Callback>,
}

impl Event_Dispatcher {
    pub fn register<F>(&mut self, id: Event, fun: Callback) {
        self.events.insert(get_type(&id), fun);
    }
    pub fn handle(&self, evt: Event) -> Result<(), String> {
        match self.events.get(&get_type(&evt)) {
            Some(ref fun) => {
                fun(&evt);
                Ok(())
            }
            None => Err(format!("Event {:?} not handled", evt)),
        }
    }
}
