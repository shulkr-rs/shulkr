#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TicketType {
    pub timeout: i64,
    flags: u8,
}

impl TicketType {
    const FLAG_PERSIST: u8 = 1;
    const FLAG_LOADING: u8 = 2;
    const FLAG_SIMULATION: u8 = 4;
    const FLAG_KEEP_DIMENSION_ACTIVE: u8 = 8;
    const FLAG_CAN_EXPIRE_IF_UNLOADED: u8 = 16;

    const fn new(timeout: i64, flags: u8) -> Self {
        Self { timeout, flags }
    }

    pub const PLAYER_SPAWN: TicketType = Self::new(20, 2);
    pub const SPAWN_SEARCH: TicketType = Self::new(1, 2);
    pub const DRAGON: TicketType = Self::new(0, 6);
    pub const PLAYER_LOADING: TicketType = Self::new(0, 2);
    pub const PLAYER_SIMULATION: TicketType = Self::new(0, 12);
    pub const FORCED: TicketType = Self::new(0, 15);
    pub const PORTAL: TicketType = Self::new(300, 15);
    pub const ENDER_PEARL: TicketType = Self::new(40, 14);
    pub const UNKNOWN: TicketType = Self::new(1, 18);

    pub fn persist(&self) -> bool {
        self.flags & Self::FLAG_PERSIST != 0
    }

    pub fn does_load(&self) -> bool {
        self.flags & Self::FLAG_LOADING != 0
    }

    pub fn does_simulate(&self) -> bool {
        self.flags & Self::FLAG_SIMULATION != 0
    }

    pub fn should_keep_dimension_active(&self) -> bool {
        self.flags & Self::FLAG_KEEP_DIMENSION_ACTIVE != 0
    }

    pub fn can_expire_if_unloaded(&self) -> bool {
        self.flags & Self::FLAG_CAN_EXPIRE_IF_UNLOADED != 0
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout != 0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ticket {
    pub ticket_type: TicketType,
    pub level: i32,
    ticks_left: i64,
}

impl Ticket {
    pub fn new(ticket_type: TicketType, level: i32) -> Self {
        Self {
            ticket_type,
            level,
            ticks_left: ticket_type.timeout,
        }
    }

    pub fn reset_ticks_left(&mut self) {
        self.ticks_left = self.ticket_type.timeout;
    }

    pub fn decrease_ticks_left(&mut self) {
        if self.ticket_type.has_timeout() {
            self.ticks_left -= 1;
        }
    }

    pub fn is_timed_out(&self) -> bool {
        self.ticket_type.has_timeout() && self.ticks_left < 0
    }

    pub(crate) fn same_type_and_level(&self, other: &Ticket) -> bool {
        self.ticket_type == other.ticket_type && self.level == other.level
    }
}
