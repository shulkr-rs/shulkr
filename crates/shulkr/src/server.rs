use parking_lot::Mutex;
use std::sync::{Arc, OnceLock, atomic::Ordering};
use tokio::net::ToSocketAddrs;

use crate::{
    auth::{AuthMode, KeyStore},
    command::CommandDispatcher,
    entity::Player,
    event::Events,
    registry::Registries,
};

static CURRENT: OnceLock<Arc<imp::Server>> = OnceLock::new();

#[derive(Clone)]
pub struct Server(Arc<imp::Server>);

impl Server {
    pub fn new(auth_mode: AuthMode) -> Self {
        Self::with_registries(auth_mode, Registries::new())
    }

    pub fn with_registries(auth_mode: AuthMode, registries: Registries) -> Self {
        let imp = Arc::new(imp::Server::new(auth_mode, registries));

        CURRENT
            .set(imp.clone())
            .map_err(|_| ())
            .expect("Server already initialized");

        Self(imp)
    }

    /// Get the current server (panics if not set)
    pub fn current() -> Self {
        Self::try_current().expect("no server found. did you start the server?")
    }

    /// Try to get the current server
    pub fn try_current() -> Option<Self> {
        CURRENT.get().map(|r| Self(r.clone()))
    }

    #[cfg(test)]
    pub(crate) fn test_server() -> Self {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        Self::try_current().unwrap_or_else(|| Self::new(crate::auth::AuthMode::Offline))
    }

    /// Bind and run server
    pub fn bind<A: ToSocketAddrs>(&self, addr: A) -> Result<(), ServerError> {
        let imp = self.0.clone();
        imp.clone().runtime.block_on(imp.bind(addr))
    }

    pub fn auth_mode(&self) -> &AuthMode {
        &self.auth_mode
    }

    pub fn registries(&self) -> &Registries {
        &self.registries
    }

    pub fn players(&self) -> &Arc<Mutex<Vec<Player>>> {
        &self.players
    }

    pub fn command_dispatcher(&self) -> &Arc<CommandDispatcher<Player>> {
        &self.command_dispatcher
    }

    pub fn key_store(&self) -> &Arc<KeyStore> {
        &self.key_store
    }

    pub fn events(&self) -> &Events {
        &self.events
    }

    pub fn set_brand(&self, brand: impl Into<String>) {
        *self.brand.lock() = brand.into();
    }

    pub fn brand(&self) -> String {
        self.brand.lock().clone()
    }

    /// Shutdown server
    pub fn shutdown(&self) {
        self.closed.store(true, Ordering::Relaxed);
    }
}

impl std::ops::Deref for Server {
    type Target = imp::Server;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

mod imp {
    use parking_lot::Mutex;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    use tokio::net::{TcpListener, ToSocketAddrs};

    use crate::{
        auth::{AuthMode, KeyStore},
        command::CommandDispatcher,
        entity::Player,
        event::Events,
        network::client::Connection,
        registry::Registries,
        server::ServerError,
        tickable::Ticker,
    };

    pub struct Server {
        pub(super) runtime: tokio::runtime::Runtime,
        pub(super) closed: AtomicBool,

        pub(super) auth_mode: AuthMode,
        pub(super) registries: Registries,
        pub(super) key_store: Arc<KeyStore>,
        pub(super) events: Events,
        pub(super) players: Arc<Mutex<Vec<Player>>>,
        pub(super) command_dispatcher: Arc<CommandDispatcher<Player>>,
        pub(super) brand: Mutex<String>,
    }

    impl Server {
        pub(super) fn new(auth_mode: AuthMode, registries: Registries) -> Self {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("failed to build Tokio runtime");

            Self {
                runtime,
                closed: AtomicBool::new(false),

                auth_mode,
                registries,
                key_store: Arc::new(KeyStore::new()),
                events: Events::new(),
                players: Arc::new(Mutex::new(Vec::new())),
                command_dispatcher: Arc::new(CommandDispatcher::new()),
                brand: Mutex::new("null".to_string()),
            }
        }

        pub(super) async fn bind(
            self: Arc<Self>,
            addr: impl ToSocketAddrs,
        ) -> Result<(), ServerError> {
            #[cfg(debug_assertions)]
            let _ = env_logger::try_init();

            let listener = TcpListener::bind(addr).await?;
            log::debug!("Listening on {}", listener.local_addr()?);

            let rt_handle = self.runtime.handle().clone();
            let this = self.clone();

            // Tick loop
            rt_handle.spawn({
                async move {
                    let mut ticker = Ticker::new();
                    while !this.closed() {
                        ticker.tick().await;
                    }
                }
            });

            // Accept loop

            while !self.closed() {
                let (stream, addr) = listener.accept().await?;

                rt_handle.spawn(async move {
                    Connection::accept(addr, stream).await;
                });
            }

            Ok(())
        }

        pub(super) fn closed(&self) -> bool {
            self.closed.load(Ordering::Relaxed)
        }
    }
}
