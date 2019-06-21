#![feature(proc_macro_hygiene, decl_macro)]
#![feature(type_ascription)]

#[macro_use] extern crate rocket;
extern crate mustache;
extern crate chrono;
extern crate postgres;

mod render_page;
mod cookies;
mod server;
mod data;
pub mod database;
mod forms;
mod expandable_form;
pub use server::start;
