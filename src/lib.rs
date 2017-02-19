#![feature(conservative_impl_trait, plugin)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;
extern crate tokio_core;
extern crate futures;

use futures::Future;



use std::collections::HashMap;
use std::sync::Arc;


mod server;
mod client;

pub use server::Server;
pub use server::FutureServiceExt as ServiceExt;
pub use client::Client;

