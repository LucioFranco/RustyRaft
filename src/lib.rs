#![feature(conservative_impl_trait)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;
extern crate byteorder;

extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;

mod server;
mod client;
mod messages;
mod connection;

pub use server::Server;
