#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate mustache;

mod util;
mod server;
pub use server::start;
