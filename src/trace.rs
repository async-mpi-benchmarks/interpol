use crate::event::{Event, MpiOp, WORLD};

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
}
