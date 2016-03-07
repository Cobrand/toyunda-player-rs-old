use std::collections::HashMap;

use sdl2::event::Event;

//type Event = sdl2::Event;
type Callback = Box<(Fn(&mut Event) -> Result<(), String>)>;

//fn mk_callback<F>(f: &F) -> Callback
//    where F: Fn(&mut Event) -> Result<(), i32> {
//        Box::new(*f) as Callback
//    }

pub struct Event_Dispatcher {
    events: HashMap<Event, Callback>,
}

impl Event_Dispatcher {
    pub fn register<F>(&mut self, id: Event, fun: Callback) {
            self.events.insert(id, fun);
    }
    pub fn handle(&self, evt: Event) -> Result<(), String>{
        match self.events.get(evt) {
            Some(&fun) => {
                fun(evt);
                Ok(())
            },
            None => Err(format!("Event {:?} not handled", evt))
        }
    }
}
