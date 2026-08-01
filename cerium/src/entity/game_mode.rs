#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl From<GameMode> for u8 {
    fn from(value: GameMode) -> Self {
        match value {
            GameMode::Survival => 0,
            GameMode::Creative => 1,
            GameMode::Adventure => 2,
            GameMode::Spectator => 3,
        }
    }
}

impl TryFrom<i32> for GameMode {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let this = match value {
            0 => GameMode::Survival,
            1 => GameMode::Creative,
            2 => GameMode::Adventure,
            3 => GameMode::Spectator,
            _ => return Err(()),
        };
        Ok(this)
    }
}
