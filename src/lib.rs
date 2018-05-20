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
mod forms;
pub use server::start;
