// Don't warn about unstable for now
#![allow(unstable)]
// We need regex plugin
#![feature(plugin)]

extern crate hyper;
extern crate url;
extern crate regex;
#[plugin] #[no_link] extern crate regex_macros;
extern crate time;

pub use client::Cluster;
pub use client::Scheme;

mod client;

#[test]
fn it_works() {
}
