use crate::event::{Event, MpiOp, WORLD};
use serde::{Serialize, Deserialize};

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
        let e = Event::new(
            MpiOp::Irecv {
                current_rank: 1,
                partner_rank: 0,
                tag: 0,
                req: 0,
            },
            Some(WORLD),
            69,
            420,
        );

        t.add_event(e.clone());
        assert_eq!(t.events[0], e);
    }

    #[test]
    fn serialize_to_file() {
        let mut t = Trace::new();
        let e0 = Event::new(MpiOp::Init, None, 3, 7);
        let e1 = Event::new(
            MpiOp::Isend {
                current_rank: 0,
                partner_rank: 1,
                tag: 0,
                req: 0,
            },
            Some(WORLD),
            9,
            19,
        );
        let e2 = Event::new(MpiOp::Wait { current_rank: 0, req: 0 }, Some(WORLD), 20, 27);
        let e3 = Event::new(
            MpiOp::Irecv {
                current_rank: 0,
                partner_rank: 1,
                tag: 1,
                req: 1,
            },
            Some(WORLD),
            69,
            420,
        );
        let e4 = Event::new(MpiOp::Wait { current_rank: 0, req: 1, }, Some(WORLD), 555, 567);
        let e5 = Event::new(MpiOp::Finalize { current_rank: 0 }, Some(WORLD), 978, 1024);

        t.add_event(e0);
        t.add_event(e1);
        t.add_event(e2);
        t.add_event(e3);
        t.add_event(e4);
        t.add_event(e5);

        let serialized = serde_json::to_string_pretty(&t).expect("Failed to serialize");
        let mut file = File::create("/tmp/test.json").unwrap();
        write!(file, "{}", serialized);

        assert_eq!(std::path::Path::new("/tmp/test.json").exists(), true);
    }
}
