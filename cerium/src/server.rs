use parking_lot::Mutex;
use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};
use tokio::net::ToSocketAddrs;

use crate::{auth::KeyStore, entity::Player, event::Events, registry::Registries};

thread_local! {
    static CURRENT: RefCell<Option<Weak<Runtime>>> = RefCell::new(None);
}

pub struct EnterGuard(Option<Weak<Runtime>>);

impl Drop for EnterGuard {
    fn drop(&mut self) {
        CURRENT.with(|c| *c.borrow_mut() = self.0.take());
    }
}

#[derive(Debug)]
pub struct NoServerError;

impl std::fmt::Display for NoServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "no Server set for current thread — call Server::run() first"
        )
    }
}

impl std::error::Error for NoServerError {}

#[derive(Clone)]
pub struct Server {
    inner: Arc<Runtime>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Runtime::new()),
        }
    }

    /// Get current server (panics if not set)
    pub fn current() -> Self {
        Self::try_current().expect("no Server set for current thread")
    }

    /// Try to get current server
    pub fn try_current() -> Result<Self, NoServerError> {
        CURRENT.with(|c| {
            c.borrow()
                .as_ref()
                .and_then(|w| w.upgrade())
                .map(|inner| Self { inner })
                .ok_or(NoServerError)
        })
    }

    /// Enter this server context
    pub fn enter(&self) -> EnterGuard {
        CURRENT.with(|c| {
            let old = c.borrow_mut().replace(Arc::downgrade(&self.inner));
            EnterGuard(old)
        })
    }

    /// Run code inside this server context
    pub fn run<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Server) -> R,
    {
        let _guard = self.enter();
        f(self)
    }

    /// Bind and run server
    pub fn bind<A: ToSocketAddrs>(&self, addr: A) -> Result<(), ServerError> {
        let inner = self.inner.clone();
        inner.clone().runtime.block_on(inner.bind(addr))
    }

    pub fn registries(&self) -> &Registries {
        &self.inner.registries
    }

    pub fn players(&self) -> &Arc<Mutex<Vec<Player>>> {
        &self.inner.players
    }

    pub fn key_store(&self) -> &Arc<KeyStore> {
        &self.inner.key_store
    }

    pub fn events(&self) -> &Events {
        &self.inner.events
    }

    /// Shutdown server
    pub fn shutdown(&self) {
        self.inner
            .closed
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

mod __private {
    use parking_lot::Mutex;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    use tokio::net::{TcpListener, ToSocketAddrs};

    use crate::{
        auth::KeyStore,
        entity::Player,
        event::Events,
        network::client::Connection,
        registry::Registries,
        server::{Server, ServerError},
        tickable::Ticker,
    };

    pub struct Runtime {
        pub(super) runtime: tokio::runtime::Runtime,
        pub(super) closed: AtomicBool,

        pub(super) registries: Registries,
        pub(super) key_store: Arc<KeyStore>,
        pub(super) events: Events,
        pub(super) players: Arc<Mutex<Vec<Player>>>,
    }

    impl Runtime {
        pub(super) fn new() -> Self {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("failed to build Tokio runtime");

            Self {
                runtime,
                closed: AtomicBool::new(false),

                registries: Registries::new(),
                key_store: Arc::new(KeyStore::new()),
                events: Events::new(),
                players: Arc::new(Mutex::new(Vec::new())),
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
                let server = Server {
                    inner: this.clone(),
                };
                async move {
                    let mut ticker = Ticker::new(server.clone());
                    while !this.closed() {
                        ticker.tick().await;
                    }
                }
            });

            // Accept loop
            let base = Server {
                inner: self.clone(),
            };

            while !self.closed() {
                let (stream, addr) = listener.accept().await?;
                let server = base.clone();

                rt_handle.spawn(async move {
                    let _g = server.enter();
                    Connection::accept(addr, stream).await;
                });
            }

            Ok(())
        }

        pub fn closed(&self) -> bool {
            self.closed.load(Ordering::Relaxed)
        }
    }
}

use __private::Runtime;
