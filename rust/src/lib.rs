#![feature(conservative_impl_trait)]
#![feature(custom_derive)]
#![feature(type_ascription)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate mustache;
extern crate chrono;
extern crate postgres;

mod util;
mod server;
mod data;
pub mod database;
pub use server::start;
