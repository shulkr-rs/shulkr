use serde::{Deserialize, Serialize};
use shulkr_macros::Enumeration;
use uuid::Uuid;

use crate::util::{Either, Key};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameProfile {
    #[serde(alias = "id")]
    pub uuid: Uuid,
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvableProfile {
    pub kind: ProfileKind,
    pub unpack: Either<PartialProfile, GameProfile>,
    pub body: Option<Key>,
    pub cape: Option<Key>,
    pub elytra: Option<Key>,
    pub model: Option<PlayerModel>,
}

impl ResolvableProfile {
    pub const fn empty() -> Self {
        ResolvableProfile {
            kind: ProfileKind::Partial,
            unpack: Either::Left(PartialProfile {
                username: None,
                uuid: None,
                properties: Vec::new(),
            }),
            body: None,
            cape: None,
            elytra: None,
            model: None,
        }
    }

    pub fn kind(&self) -> ProfileKind {
        self.kind
    }

    pub fn profile(&self) -> &Either<PartialProfile, GameProfile> {
        &self.unpack
    }

    pub fn partial_profile(&self) -> Option<&PartialProfile> {
        match &self.unpack {
            Either::Left(profile) => Some(profile),
            _ => None,
        }
    }

    pub fn complete_profile(&self) -> Option<&GameProfile> {
        match &self.unpack {
            Either::Right(profile) => Some(profile),
            _ => None,
        }
    }

    pub fn body(&self) -> Option<&Key> {
        self.body.as_ref()
    }

    pub fn cape(&self) -> Option<&Key> {
        self.cape.as_ref()
    }

    pub fn elytra(&self) -> Option<&Key> {
        self.elytra.as_ref()
    }

    pub fn player_model(&self) -> Option<&PlayerModel> {
        self.model.as_ref()
    }
}

#[derive(Enumeration)]
pub enum ProfileKind {
    Partial,
    Complete,
}

#[derive(Debug, Clone)]
pub struct PartialProfile {
    pub username: Option<String>,
    pub uuid: Option<Uuid>,
    pub properties: Vec<Property>,
}

#[derive(Enumeration)]
pub enum PlayerModel {
    Wide,
    Slim,
}
