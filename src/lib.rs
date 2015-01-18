// Don't warn about unstable for now
#![allow(unstable)]
// We need regex plugin
#![feature(plugin)]

extern crate hyper;
extern crate url;
extern crate regex;
#[plugin] #[no_link] extern crate regex_macros;
extern crate time;

pub use client::Influx;
pub use transport::Scheme;

pub mod client;
mod transport;

#[test]
fn it_works() {
}
