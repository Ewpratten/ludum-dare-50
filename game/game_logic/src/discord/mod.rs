//! Interfacing with Discord
//! 
//! ## Overview
//! 
//! When the game is run at the same time as a Discord client on a computer, it will attach to the user's 
//! account and display [Rich Presence](https://discord.com/rich-presence) information.
//! 
//! This is handled through the [`discord-sdk`](https://github.com/EmbarkStudios/discord-sdk) crate, 
//! but still requires some additional code to get everything set up.
//! 
//! Our main focuses in this module are:
//! 
//! - Ensuring that the game does not crash when Discord is not running
//! - Ensuring that Discord can not pause the game by taking too long to respond to an update
//! 
//! To solve these, we run this task in its own thread, and talk to it through 
//! Tokio's [`mpsc`](https://docs.rs/tokio/latest/tokio/sync/mpsc/fn.channel.html) 
//! implementation (as we are already working in an async context).
//! 
//! ## Usage
//! 
//! ```rust,no_run
//! let app_id = 123456789;
//! 
//! // Connect to discord
//! let discord = DiscordRpcThreadHandle::new(app_id).await.unwrap();
//! let event_loop_discord_tx = discord.get_channel();
//! 
//! // When this variable is dropped, the connection is closed, so keep this around
//! let discord_task_handle = discord.begin_thread_non_blocking();
//! 
//! // We can then send signals any time we want
//! event_loop_discord_tx.send(DiscordRpcSignal::BeginGameTimer).await.unwrap();
//! ```

mod signal;
use std::time::Duration;

pub use signal::DiscordRpcSignal;
use tokio::{
    sync::{mpsc::Receiver, mpsc::Sender},
    task::JoinHandle,
};

use self::{ipc::DiscordRpcClient, signal::StatefulDiscordRpcSignalHandler};
mod ipc;
pub use ipc::DiscordError;

/// How long to wait before we give up on connecting to Discord.
const DISCORD_CONNECT_TIMEOUT_SECONDS: u64 = 5;

/// A cross-thread communication channel for sending Discord RPC events.
pub type DiscordChannel = Sender<DiscordRpcSignal>;

pub struct DiscordRpcThreadHandle {
    tx_chan: DiscordChannel,
    rx_chan: Receiver<DiscordRpcSignal>,
    internal_client: Option<DiscordRpcClient>,
    state: StatefulDiscordRpcSignalHandler,
}

impl DiscordRpcThreadHandle {
    /// Construct a new `DiscordRpcThreadHandle`
    pub async fn new(app_id: i64) -> Result<Self, DiscordError> {
        // Create the Discord client
        info!("Trying to locate and connect to a local Discord process for RPC. Will wait up to {} seconds before timing out", DISCORD_CONNECT_TIMEOUT_SECONDS);
        let rpc_client = match tokio::time::timeout(
            Duration::from_secs(DISCORD_CONNECT_TIMEOUT_SECONDS),
            DiscordRpcClient::new(app_id, discord_sdk::Subscriptions::ACTIVITY),
        )
        .await
        {
            Ok(client) => Some(client?),
            Err(t) => {
                error!(
                    "Timed out trying to connect to Discord RPC. Duration: {}",
                    t
                );
                None
            }
        };
        info!("Successfully connected to Discord");

        // Set up channels
        let (tx, rx) = tokio::sync::mpsc::channel(5);

        Ok(Self {
            tx_chan: tx,
            rx_chan: rx,
            internal_client: rpc_client,
            state: StatefulDiscordRpcSignalHandler::default(),
        })
    }

    /// Get access to the inter-thread channel for communicating to discord
    pub fn get_channel(&self) -> DiscordChannel {
        self.tx_chan.clone()
    }

    /// Run the inner communication task in an async context
    pub fn begin_thread_non_blocking(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                // Handle any possible incoming events
                match self.rx_chan.try_recv() {
                    Ok(signal) => match self.internal_client {
                        Some(ref client) => {
                            client
                                .set_rich_presence(self.state.apply(signal))
                                .await
                                .unwrap();
                        }
                        None => { /* The client could not connect */ }
                    },
                    Err(_) => { /* Do Nothing */ }
                }
            }
        })
    }
}
