#![allow(ambiguous_glob_reexports)]

pub mod client {
    pub mod handshake {
        #[allow(clippy::module_inception)]
        mod handshake;

        pub use handshake::HandshakePacket;
    }

    pub mod status {
        mod ping_request;
        mod status_request;

        pub use ping_request::PingRequestPacket;
        pub use status_request::StatusRequestPacket;
    }

    pub mod login {
        mod encryption_response;
        mod login_acknowledged;
        mod login_plugin_response;
        mod login_start;

        pub use encryption_response::EncryptionResponsePacket;
        pub use login_acknowledged::LoginAcknowledgePacket;
        pub use login_plugin_response::LoginPluginResponsePacket;
        pub use login_start::LoginStartPacket;
    }

    pub mod config {
        mod acknowledge_finish_config;
        mod client_info;
        mod known_packs;
        mod plugin_message;

        pub use acknowledge_finish_config::AcknowledgeFinishConfigPacket;
        pub use client_info::ClientInfoPacket;
        pub use known_packs::KnownPacksPacket;
        pub use plugin_message::PluginMessagePacket;
    }

    pub mod play {
        mod change_recipe_book_settings;
        mod chat_command;
        mod chat_message;
        mod chunk_batch_received;
        mod click_container;
        mod client_tick_end;
        mod close_container;
        mod confirm_teleportation;
        mod interact;
        mod keep_alive;
        mod pick_block;
        mod pick_entity;
        mod ping_request;
        mod player_abilities;
        mod player_action;
        mod player_command;
        mod player_input;
        mod player_loaded;
        mod player_movement_flags;
        mod player_position;
        mod player_position_and_rotation;
        mod player_rotation;
        mod player_session;
        mod plugin_message;
        mod request_game_mode;
        mod seen_advancements;
        mod set_creative_mode_slot;
        mod set_held_item;
        mod swing_arm;
        mod use_item;
        mod use_item_on;

        pub use change_recipe_book_settings::ChangeRecipeBookSettingsPacket;
        pub use chat_command::ChatCommandPacket;
        pub use chat_message::ChatMessagePacket;
        pub use chunk_batch_received::ChunkBatchReceivedPacket;
        pub use click_container::ClickContainerPacket;
        pub use client_tick_end::ClientTickEndPacket;
        pub use close_container::CloseContainerPacket;
        pub use confirm_teleportation::ConfirmTeleportationPacket;
        pub use interact::InteractPacket;
        pub use keep_alive::KeepAlivePacket;
        pub use pick_block::PickItemFromBlockPacket;
        pub use pick_entity::PickItemFromEntityPacket;
        pub use ping_request::PingRequestPacket;
        pub use player_abilities::PlayerAbilitiesPacket;
        pub use player_action::*;
        pub use player_command::*;
        pub use player_input::*;
        pub use player_loaded::PlayerLoadedPacket;
        pub use player_movement_flags::PlayerMovementFlagsPacket;
        pub use player_position::PlayerPositionPacket;
        pub use player_position_and_rotation::PlayerPositionAndRotationPacket;
        pub use player_rotation::PlayerRotationPacket;
        pub use player_session::PlayerSessionPacket;
        pub use plugin_message::PluginMessagePacket;
        pub use request_game_mode::PlayerRequestGameModePacket;
        pub use seen_advancements::SeenAdvancementsPacket;
        pub use set_creative_mode_slot::SetCreativeModeSlotPacket;
        pub use set_held_item::SetHeldItemPacket;
        pub use swing_arm::SwingArmPacket;
        pub use use_item::UseItemPacket;
        pub use use_item_on::UseItemOnPacket;
    }

    pub use config::*;
    pub use handshake::*;
    pub use login::*;
    pub use play::*;
    pub use status::*;
}

pub mod server {

    pub mod handshake {
        // Empty
    }

    pub mod status {
        mod pong_response;
        mod status_response;

        pub use pong_response::PongResponsePacket;
        pub use status_response::StatusResponsePacket;
    }

    pub mod login {
        mod encryption_request;
        mod login_disconnect;
        mod login_plugin_request;
        mod login_success;
        mod set_compression;

        pub use encryption_request::*;
        pub use login_disconnect::LoginDisconnectPacket;
        pub use login_plugin_request::LoginPluginRequestPacket;
        pub use login_success::*;
        pub use set_compression::SetCompressionPacket;
    }

    pub mod config {
        mod feature_flags;
        mod finish_config;
        mod known_packs;
        mod plugin_message;
        mod registry_data;
        mod update_tags;

        pub use feature_flags::FeatureFlagsPacket;
        pub use finish_config::FinishConfigPacket;
        pub use known_packs::KnownPacksPacket;
        pub use plugin_message::PluginMessagePacket;
        pub use registry_data::*;
        pub use update_tags::*;
    }

    pub mod play {
        mod acknowledge_block_change;
        mod block_update;
        mod chat_message;
        mod chunk_batch_finished;
        mod chunk_batch_start;
        mod chunk_data_and_update_light;
        mod close_container;
        mod commands;
        mod disconnect;
        mod display_objective;
        mod entity_animation;
        mod entity_event;
        mod entity_position;
        mod entity_position_rotation;
        mod entity_rotation;
        mod game_event;
        mod keep_alive;
        mod login;
        mod open_screen;
        mod player_abilities;
        mod player_info_remove;
        mod player_info_update;
        mod remove_entities;
        mod reset_score;
        mod respawn;
        mod set_block_destroy_stage;
        mod set_center_chunk;
        mod set_container_content;
        mod set_container_slot;
        mod set_cursor_item;
        mod set_entity_metadata;
        mod set_head_rotation;
        mod set_held_item;
        mod set_tablist_header_footer;
        mod spawn_entity;
        mod sync_player_position;
        mod system_chat_message;
        mod unload_chunk;
        mod update_advancements;
        mod update_objectives;
        mod update_score;
        mod update_teams;
        mod world_event;

        pub use acknowledge_block_change::AcknowledgeBlockChangePacket;
        pub use block_update::BlockUpdatePacket;
        pub use chat_message::PlayerChatMessagePacket;
        pub use chunk_batch_finished::ChunkBatchFinishedPacket;
        pub use chunk_batch_start::ChunkBatchStartPacket;
        pub use chunk_data_and_update_light::*;
        pub use close_container::CloseContainerPacket;
        pub use commands::CommandsPacket;
        pub use disconnect::DisconnectPacket;
        pub use display_objective::DisplayObjectivePacket;
        pub use entity_animation::EntityAnimationPacket;
        pub use entity_event::EntityEventPacket;
        pub use entity_position::EntityPositionPacket;
        pub use entity_position_rotation::EntityPositionRotationPacket;
        pub use entity_rotation::EntityRotationPacket;
        pub use game_event::GameEventPacket;
        pub use keep_alive::KeepAlivePacket;
        pub use login::LoginPacket;
        pub use open_screen::OpenScreenPacket;
        pub use player_abilities::*;
        pub use player_info_remove::PlayerInfoRemovePacket;
        pub use player_info_update::*;
        pub use remove_entities::RemoveEntitiesPacket;
        pub use reset_score::ResetScorePacket;
        pub use respawn::*;
        pub use set_block_destroy_stage::SetBlockDestroyStagePacket;
        pub use set_center_chunk::SetCenterChunkPacket;
        pub use set_container_content::SetContainerContentPacket;
        pub use set_container_slot::SetContainerSlotPacket;
        pub use set_cursor_item::SetCursorItemPacket;
        pub use set_entity_metadata::SetEntityMetadataPacket;
        pub use set_head_rotation::SetHeadRotationPacket;
        pub use set_held_item::SetHeldItemPacket;
        pub use set_tablist_header_footer::SetTablistHeaderFooterPacket;
        pub use spawn_entity::SpawnEntityPacket;
        pub use sync_player_position::SyncPlayerPositionPacket;
        pub use system_chat_message::SystemChatMessagePacket;
        pub use unload_chunk::UnloadChunkPacket;
        pub use update_advancements::UpdateAdvancementsPacket;
        pub use update_objectives::{UpdateObjectivesAction, UpdateObjectivesPacket};
        pub use update_score::UpdateScorePacket;
        pub use update_teams::*;
        pub use world_event::WorldEventPacket;
    }

    pub use config::*;
    pub use login::*;
    pub use play::*;
    pub use status::*;
}

use std::fmt::Debug;

pub use client::*;
pub use server::*;

use crate::protocol::{decode::Decode, encode::Encode};

pub trait Packet
where
    Self: Debug + Clone,
{
}

/// Marks a [`Packet`] as one coming from the client.
pub trait ClientPacket
where
    Self: Packet + Decode,
{
}

/// Marks a [`Packet`] as one coming from the server.
pub trait ServerPacket
where
    Self: Packet + Encode,
{
}

#[derive(Debug, Clone)]
pub struct RawPacket {
    id: i32,
    data: Vec<u8>,
}

impl RawPacket {
    pub fn new(id: i32, data: Vec<u8>) -> Self {
        Self { id, data }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
