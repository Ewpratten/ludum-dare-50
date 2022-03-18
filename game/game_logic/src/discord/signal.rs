//! This file contains a system for signaling Discord RPC context changes between threads.
//!
//! ## Description
//!
//! The idea is that the thread containing the Discord RPC client can hold a `StatefulDiscordRpcSignalHandler` as a stateful context.
//! The game thread can then send `DiscordRpcSignal` values through an `mpsc` sender, which will be received by the Discord RPC client thread.

use chrono::Utc;
use discord_sdk::activity::{ActivityBuilder, Assets, IntoTimestamp};

/// Definitions of signals that can be sent to the Discord RPC thread to control how discord displays game status.
pub enum DiscordRpcSignal {
    /// Signal to begin a game timer (Discord will display `XX:XX elapsed`)
    BeginGameTimer,

    /// Signal to end a game timer
    EndGameTimer,

    /// Signal to begin a countdown timer (Discord will display `XX:XX left`)
    SetGameTimeRemainingTimestamp(chrono::DateTime<Utc>),

    /// Signal to clear the game remaining timer
    ClearGameTimeRemaining,

    /// Signal to set the details in the info card
    ChangeDetails {
        /// What the player is doing, eg. “Exploring the Wilds of Outland”.
        ///
        /// Limited to 128 bytes.
        details: String,

        /// The user’s currenty party status, eg. “Playing Solo”.
        ///
        /// Limited to 128 bytes.
        party_status: Option<String>,
    },

    /// Signal to change the graphical assets in the info card
    ChangeAssets(Assets),
}

/// A struct that can keep track of incoming signals and their effect on Discord
#[derive(Default, Debug, Clone)]
pub struct StatefulDiscordRpcSignalHandler {
    game_start_timer: Option<chrono::DateTime<Utc>>,
    game_end_timer: Option<chrono::DateTime<Utc>>,
    game_details: Option<String>,
    game_party_status: Option<String>,
    game_assets: Option<Assets>,
}

impl StatefulDiscordRpcSignalHandler {

    /// Apply a signal to generate a new activity
    pub fn apply(&mut self, signal: DiscordRpcSignal) -> ActivityBuilder {
        // Fill in the data based on the contents of the signal
        match signal {
            DiscordRpcSignal::BeginGameTimer => self.game_start_timer = Some(chrono::Utc::now()),
            DiscordRpcSignal::EndGameTimer => self.game_start_timer = None,
            DiscordRpcSignal::SetGameTimeRemainingTimestamp(timestamp) => {
                self.game_end_timer = Some(timestamp)
            }
            DiscordRpcSignal::ClearGameTimeRemaining => self.game_end_timer = None,
            DiscordRpcSignal::ChangeDetails {
                details,
                party_status,
            } => {
                self.game_details = Some(details);
                self.game_party_status = party_status;
            }
            DiscordRpcSignal::ChangeAssets(assets) => self.game_assets = Some(assets),
        }

        // Decide how to build the Discord RPC activity
        let mut builder = ActivityBuilder::default();
        if let Some(start_time) = &self.game_start_timer {
            builder = builder.start_timestamp(start_time.timestamp());
        }
        if let Some(end_time) = &self.game_end_timer {
            builder = builder.end_timestamp(end_time.timestamp());
        }
        if let Some(details) = &self.game_details {
            builder = builder.details(details);
        }
        if let Some(party_status) = &self.game_party_status {
            builder = builder.state(party_status);
        }
        if let Some(assets) = &self.game_assets {
            builder = builder.assets(assets.clone());
        }

        return builder;
    }
}
