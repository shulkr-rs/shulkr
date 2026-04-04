use serde::Serialize;
use uuid::Uuid;

use crate::{
    entity::{EntityLike, Player},
    text::TextComponent,
};

/// Represents the full response sent during a Minecraft server list ping.
#[derive(Debug, Serialize)]
pub struct PingResponse {
    version: Version,
    players: Players,
    description: TextComponent,
    #[serde(rename = "enforcesSecureChat")]
    enforces_secure_chat: bool,
}

impl PingResponse {
    /// Creates a new [`Builder`] for constructing a [`PingResponse`].
    ///
    /// # Example
    /// ```
    /// let response = PingResponse::builder(version)
    ///     .with_max_players(100)
    ///     .with_online_players(5)
    ///     .build();
    /// ```
    pub fn builder(version: Version) -> Builder {
        Builder::new(version)
    }
}

/// A builder for constructing a [`PingResponse`].
pub struct Builder {
    version: Version,
    online_players: i32,
    max_players: i32,
    description: TextComponent,
    players: Vec<SamplePlayer>,
}

impl Builder {
    pub(crate) fn new(version: Version) -> Self {
        Self {
            version,
            online_players: 0,
            max_players: 0,
            players: Vec::new(),
            description: TextComponent::new(),
        }
    }

    /// Sets the server version.
    pub fn with_version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }

    /// Sets the maximum number of players shown in the server list.
    ///
    /// Note: This is purely cosmetic and does not enforce an actual limit.
    pub fn with_max_players(mut self, max_players: i32) -> Self {
        self.max_players = max_players;
        self
    }

    /// Sets the number of currently online players.
    pub fn with_online_players(mut self, online_players: i32) -> Self {
        self.online_players = online_players;
        self
    }

    /// Sets the server description.
    pub fn with_description(mut self, description: TextComponent) -> Self {
        self.description = description;
        self
    }

    /// Adds a single sample player to the response.
    ///
    /// Sample players may be displayed in the server list hover tooltip.
    pub fn with_player(mut self, player: SamplePlayer) -> Self {
        self.players.push(player);
        self
    }
    /// Adds multiple sample players to the response.
    ///
    /// Sample players may be displayed in the server list hover tooltip.
    pub fn with_players(mut self, players: impl IntoIterator<Item = SamplePlayer>) -> Self {
        self.players.extend(players);
        self
    }

    /// Builds the [`PingResponse`].
    pub fn build(self) -> PingResponse {
        PingResponse {
            version: self.version,
            players: Players {
                max: self.max_players,
                online: self.online_players,
                sample: self.players,
            },
            description: self.description,
            enforces_secure_chat: true,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Version {
    protocol: i32,
    name: String,
}

impl Version {
    pub fn new(protocol: i32, name: impl ToString) -> Self {
        Self {
            protocol,
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SamplePlayer {
    name: String,
    #[serde(rename = "id")]
    uuid: Uuid,
}

impl<S> From<(S, Uuid)> for SamplePlayer
where
    S: ToString,
{
    fn from(value: (S, Uuid)) -> Self {
        Self {
            name: value.0.to_string(),
            uuid: value.1,
        }
    }
}

impl From<Player> for SamplePlayer {
    fn from(value: Player) -> Self {
        Self {
            name: value.name().clone(),
            uuid: value.uuid(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Players {
    max: i32,
    online: i32,
    sample: Vec<SamplePlayer>,
}
