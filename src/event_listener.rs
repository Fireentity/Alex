use std::collections::HashMap;
use winit::event;
use winit::event::Event;

pub struct EventListener<'a> {
    events: HashMap<Event<()>, Vec<dyn Fn(&Event<()>)>>,
}

impl EventListener {
    pub fn new() -> Self {
        EventListener { events: HashMap::new() }
    }

    pub fn register_event(&mut self, event: &Event<()>, on_execute: Box<dyn Fn(Event<()>)>) -> void {
        let result = self.events.get(event);
        match result {
            None => self.events.insert(event, vec![on_execute]),
            Some(listeners) => (&listeners).push(on_execute)
        }
    }

    pub fn dispatch(&self, event: &Event<()>) -> void {
        let result = self.events.get(event);
        if result.is_some() {
            for iter in result.unwrap() {
                iter(event);
            }
        }
    }
}