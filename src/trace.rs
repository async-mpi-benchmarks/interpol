use crate::event::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Trace {
    events: Vec<Event>,
}

impl Trace {
    pub fn new() -> Self {
        Trace { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }
}

#[cfg(test)]
mod tests {
    use crate::WORLD;
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn builds() {
        let t = Trace::new();
        assert_eq!(t.events, vec![]);
    }

    #[test]
    fn adds_event() {
        let mut t = Trace::new();
        let e = Event::Irecv(NonBlocking::new(1, 0, WORLD, 0, 0, 69, 420));

        t.add_event(e.clone());
        assert_eq!(t.events[0], e);
    }

    #[test]
    fn serialize_to_file() {
        let mut t = Trace::new();
        let e0 = Event::Init(Init::new(34, 0.78));
        let e1 = Event::Isend(NonBlocking::new(0, 1, WORLD, 0, 0, 9, 19));
        let e2 = Event::Wait(Wait::new(0, WORLD, 0, 20, 27));
        let e3 = Event::Irecv(NonBlocking::new(0, 1, WORLD, 1, 1, 69, 420));
        let e4 = Event::Wait(Wait::new(0, WORLD, 1, 555, 567));
        let e5 = Event::Finalize(Finalize::new(0, 978, 1024f64));

        t.add_event(e0);
        t.add_event(e1);
        t.add_event(e2);
        t.add_event(e3);
        t.add_event(e4);
        t.add_event(e5);

        let serialized = serde_json::to_string_pretty(&t).expect("Failed to serialize");
        let mut file = File::create("./target/test.json").unwrap();
        write!(file, "{}", serialized).unwrap();

        assert_eq!(std::path::Path::new("./target/test.json").exists(), true);
    }
}
