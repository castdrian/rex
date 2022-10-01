#![warn(clippy::all, rust_2018_idioms)]

mod backend;
mod constants;
mod fetch;
mod gui;
mod images;
mod response;
pub use gui::MyApp;
