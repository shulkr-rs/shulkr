use crate::{
    command::{
        dispatcher::{CommandError, CommandResult},
        matches::CommandMatches,
    },
    entity::Player,
    event::{Event, player::PlayerEvent},
};

#[derive(Clone)]
pub struct CommandResultEvent {
    player: Player,
    input: String,
    result: CommandResult,
}

impl CommandResultEvent {
    pub fn new(player: Player, input: impl Into<String>, result: CommandResult) -> Self {
        Self {
            player,
            input: input.into(),
            result,
        }
    }

    pub fn input(&self) -> &String {
        &self.input
    }

    pub fn matches(&self) -> Option<&CommandMatches> {
        self.result.as_ref().ok()
    }

    pub fn error(&self) -> Option<&CommandError> {
        self.result.as_ref().err()
    }

    pub fn is_ok(&self) -> bool {
        self.result.is_ok()
    }
}

impl Event for CommandResultEvent {}
impl PlayerEvent for CommandResultEvent {
    fn get_player(&self) -> &Player {
        &self.player
    }
}
