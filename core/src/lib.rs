mod catcher;
pub mod depot;
mod error;
pub mod fs;
mod handler;
pub mod http;
pub mod routing;
pub mod server;
mod service;
#[cfg(feature = "tls")]
mod tls;
mod transport;
mod writer;

#[macro_use]
extern crate pin_utils;
#[macro_use]
extern crate futures_util;

#[cfg(feature = "anyhow")]
pub use anyhow;
pub use hyper;

pub use self::catcher::{Catcher, CatcherImpl};
pub use self::depot::Depot;
pub use self::error::Error;
pub use self::handler::Handler;
pub use self::http::{Request, Response};
pub use self::routing::Router;
pub use self::server::Server;
#[cfg(feature = "tls")]
pub use self::server::TlsServer;
pub use self::service::Service;
pub use self::writer::Writer;
pub use async_trait::async_trait;
pub use salvo_macros::fn_handler;
pub type Result<T> = std::result::Result<T, Error>;

pub mod prelude {
    pub use crate::depot::Depot;
    pub use crate::http::errors::*;
    pub use crate::http::{Request, Response, StatusCode};
    pub use crate::routing::filter;
    pub use crate::routing::Router;
    pub use crate::server::Server;
    #[cfg(feature = "tls")]
    pub use crate::server::TlsServer;
    pub use crate::service::Service;
    pub use crate::writer::*;
    pub use crate::Handler;
    pub use async_trait::async_trait;
    pub use salvo_macros::fn_handler;
}

use std::future::Future;
use tokio::runtime::{self, Runtime};

fn new_runtime(threads: usize) -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(threads)
        .thread_name("salvo-worker")
        .enable_all()
        .build()
        .unwrap()
}

pub fn start<F: Future>(future: F) {
    start_with_threads(future, num_cpus::get())
}

pub fn start_with_threads<F: Future>(future: F, threads: usize) {
    let runtime = crate::new_runtime(threads);
    let _ = runtime.block_on(async { future.await });
}
