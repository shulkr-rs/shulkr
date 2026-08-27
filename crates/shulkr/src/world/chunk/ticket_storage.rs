use super::ticket::Ticket;
use std::collections::HashMap;

pub struct TicketStorage {
    tickets: HashMap<(i32, i32), Vec<Ticket>>,
    max_level: i32,
    loading_listener: Option<Box<dyn FnMut((i32, i32), i32, bool) + Send>>,
    simulation_listener: Option<Box<dyn FnMut((i32, i32), i32, bool) + Send>>,
}

impl TicketStorage {
    pub fn new(max_level: i32) -> Self {
        Self {
            tickets: HashMap::new(),
            max_level,
            loading_listener: None,
            simulation_listener: None,
        }
    }

    pub fn set_loading_chunk_updated_listener(
        &mut self,
        listener: impl FnMut((i32, i32), i32, bool) + Send + 'static,
    ) {
        self.loading_listener = Some(Box::new(listener));
    }

    pub fn set_simulation_chunk_updated_listener(
        &mut self,
        listener: impl FnMut((i32, i32), i32, bool) + Send + 'static,
    ) {
        self.simulation_listener = Some(Box::new(listener));
    }

    pub fn has_tickets(&self) -> bool {
        !self.tickets.is_empty()
    }

    pub fn get_tickets(&self, pos: (i32, i32)) -> &[Ticket] {
        self.tickets.get(&pos).map_or(&[], Vec::as_slice)
    }

    pub fn add_ticket(&mut self, pos: (i32, i32), ticket: Ticket) -> bool {
        let existing = self.tickets.entry(pos).or_default();
        if let Some(same) = existing.iter_mut().find(|t| t.same_type_and_level(&ticket)) {
            same.reset_ticks_left();
            return false;
        }

        let old_simulation_level = Self::ticket_level_in(existing, true, self.max_level);
        let old_loading_level = Self::ticket_level_in(existing, false, self.max_level);
        existing.push(ticket);

        if ticket.ticket_type.does_simulate() && ticket.level < old_simulation_level {
            if let Some(listener) = &mut self.simulation_listener {
                listener(pos, ticket.level, true);
            }
        }
        if ticket.ticket_type.does_load() && ticket.level < old_loading_level {
            if let Some(listener) = &mut self.loading_listener {
                listener(pos, ticket.level, true);
            }
        }

        true
    }

    pub fn remove_ticket(&mut self, pos: (i32, i32), ticket: Ticket) -> bool {
        let Some(existing) = self.tickets.get_mut(&pos) else {
            return false;
        };

        let Some(index) = existing.iter().position(|t| t.same_type_and_level(&ticket)) else {
            return false;
        };
        existing.remove(index);

        if existing.is_empty() {
            self.tickets.remove(&pos);
        }

        let remaining = self.tickets.get(&pos).map_or(&[][..], Vec::as_slice);

        if ticket.ticket_type.does_simulate() {
            let new_level = Self::ticket_level_in(remaining, true, self.max_level);
            if let Some(listener) = &mut self.simulation_listener {
                listener(pos, new_level, false);
            }
        }
        if ticket.ticket_type.does_load() {
            let new_level = Self::ticket_level_in(remaining, false, self.max_level);
            if let Some(listener) = &mut self.loading_listener {
                listener(pos, new_level, false);
            }
        }

        true
    }

    pub fn ticket_level_at(&self, pos: (i32, i32), simulation: bool) -> i32 {
        Self::ticket_level_in(self.get_tickets(pos), simulation, self.max_level)
    }

    fn lowest_ticket(tickets: &[Ticket], simulation: bool) -> Option<&Ticket> {
        tickets
            .iter()
            .filter(|t| {
                if simulation {
                    t.ticket_type.does_simulate()
                } else {
                    t.ticket_type.does_load()
                }
            })
            .min_by_key(|t| t.level)
    }

    fn ticket_level_in(tickets: &[Ticket], simulation: bool, max_level: i32) -> i32 {
        Self::lowest_ticket(tickets, simulation).map_or(max_level, |t| t.level)
    }
}

#[cfg(test)]
mod tests {
    use super::super::ticket::TicketType;
    use super::*;
    use std::sync::{Arc, Mutex};

    const MAX_LEVEL: i32 = 34;

    fn recording_listener() -> (
        impl FnMut((i32, i32), i32, bool) + Send,
        Arc<Mutex<Vec<((i32, i32), i32, bool)>>>,
    ) {
        let log = Arc::new(Mutex::new(Vec::new()));
        let recorder = log.clone();
        let listener = move |pos, level, only_decreased| {
            recorder.lock().unwrap().push((pos, level, only_decreased));
        };
        (listener, log)
    }

    #[test]
    fn add_ticket_fires_loading_listener_only_when_it_lowers_the_level() {
        let mut storage = TicketStorage::new(MAX_LEVEL);
        let (listener, log) = recording_listener();
        storage.set_loading_chunk_updated_listener(listener);

        storage.add_ticket((0, 0), Ticket::new(TicketType::PLAYER_LOADING, 10));
        assert_eq!(*log.lock().unwrap(), vec![((0, 0), 10, true)]);

        storage.add_ticket((0, 0), Ticket::new(TicketType::PLAYER_LOADING, 20));
        assert_eq!(log.lock().unwrap().len(), 1);

        storage.add_ticket((0, 0), Ticket::new(TicketType::PLAYER_LOADING, 3));
        assert_eq!(
            *log.lock().unwrap(),
            vec![((0, 0), 10, true), ((0, 0), 3, true)]
        );
    }

    #[test]
    fn remove_ticket_recomputes_and_reports_the_new_minimum() {
        let mut storage = TicketStorage::new(MAX_LEVEL);
        let (listener, log) = recording_listener();
        storage.set_loading_chunk_updated_listener(listener);

        let low = Ticket::new(TicketType::PLAYER_LOADING, 3);
        let high = Ticket::new(TicketType::PLAYER_LOADING, 10);
        storage.add_ticket((0, 0), low);
        storage.add_ticket((0, 0), high);
        log.lock().unwrap().clear();

        storage.remove_ticket((0, 0), low);
        assert_eq!(*log.lock().unwrap(), vec![((0, 0), 10, false)]);

        storage.remove_ticket((0, 0), high);
        assert_eq!(
            *log.lock().unwrap(),
            vec![((0, 0), 10, false), ((0, 0), MAX_LEVEL, false)]
        );
        assert!(storage.get_tickets((0, 0)).is_empty());
    }

    #[test]
    fn ticket_level_at_ignores_the_wrong_kind_of_ticket() {
        let mut storage = TicketStorage::new(MAX_LEVEL);
        storage.add_ticket((0, 0), Ticket::new(TicketType::PLAYER_SIMULATION, 0));
        assert_eq!(storage.ticket_level_at((0, 0), false), MAX_LEVEL);
        assert_eq!(storage.ticket_level_at((0, 0), true), 0);
    }

    #[test]
    fn readding_the_same_type_and_level_just_resets_timeout_and_reports_no_change() {
        let mut storage = TicketStorage::new(MAX_LEVEL);
        let (listener, log) = recording_listener();
        storage.set_loading_chunk_updated_listener(listener);

        let ticket = Ticket::new(TicketType::PORTAL, 5);
        assert!(storage.add_ticket((0, 0), ticket));
        assert!(
            !storage.add_ticket((0, 0), ticket),
            "same type+level should not be added twice"
        );
        assert_eq!(
            log.lock().unwrap().len(),
            1,
            "re-adding an identical ticket should not re-notify"
        );
    }
}
