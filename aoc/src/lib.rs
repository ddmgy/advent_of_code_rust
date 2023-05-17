pub use aoc_derive::*;

pub mod common;
pub mod error;
pub mod input;
mod register;
mod runners;
mod solutions;

pub use register::register_runners;
pub use runners::{get_runner, register_runner};
