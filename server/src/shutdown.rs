use std::{
    future::Future,
    sync::atomic::{AtomicBool, Ordering},
};

use tokio::{signal, sync::watch};

/// If the shutdown handler is registered this variable is set to true if a program shutdown is desired.
static SHUTDOWN_REQUEST: AtomicBool = AtomicBool::new(false);

static HANDLER_REGISTERED: AtomicBool = AtomicBool::new(false);

#[derive(Clone)]
pub struct ShutdownListener {
    receiver: watch::Receiver<()>,
}

/// function which returns the global state of SHUTDOWN_REQUEST
pub fn termination_requested() -> bool {
    SHUTDOWN_REQUEST.load(Ordering::Relaxed)
}

impl ShutdownListener {
    /// register shutdown listeners
    ///
    /// When called multiple times only the first one is registered.
    pub(super) fn new() -> Result<Self, ()> {
        if HANDLER_REGISTERED.swap(true, Ordering::SeqCst) {
            return Err(());
        }

        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("could not set up CTRL-C listener")
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("could not set up TERM listener")
                .recv()
                .await
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        let (tx, rx) = watch::channel(());

        tokio::spawn(async move {
            tokio::select! {
                _ = ctrl_c => {},
                _ = terminate => {},
            }
            // terminate or ctrl_c received setting static variable
            SHUTDOWN_REQUEST.store(true, Ordering::SeqCst);
            tx.send(()).ok();
        });

        Ok(Self { receiver: rx })
    }

    pub(crate) fn handle(&self) -> impl Future<Output = ()> {
        let mut rx = self.receiver.clone();

        async move {
            // if expression is important because termination could be requested while creating the handle
            if !termination_requested() {
                rx.changed().await.ok();
            }
        }
    }
}
