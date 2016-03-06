use std::collections::HashMap;
extern crate sdl2;

use sdl2::event::Event;

//type Event = sdl2::Event;
type Callback<'a> = Box<(Fn(&'a mut Event,) -> Result<()> + 'static)>;

fn mk_callback<'a, F>(f: F) -> Callback<'a>
    where F: Fn(&'a mut Event,) -> Result<()> + 'static {
        Box::new(f) as Callback
    }

pub struct Event_Dispatcher {
    events: HashMap<Event,Callback>,
}

impl Event_Dispatcher{
    pub fn register<'a, F>(&mut self, id: Event, fun: F)
        where F: Fn(&'a mut Event,) -> Result<()> + 'static {
            self.events.insert(id, mk_callback(fun));
    }
    pub fn handle(&self, evt: Event) -> Result<()>{
        match self.events.get(evt) {
            Some(&fun) => {
                fun(evt);
                Ok()
            },
            None => Err(format!("Event {} not handled", evt))
        }
    }
}
