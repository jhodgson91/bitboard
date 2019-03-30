// Looks like we've run into a compiler bug :(
// https://github.com/rust-lang/rust/issues/58987

// Possibly started when I changed statics.rs to be more const.
// Compiles on nightly, so I guess when 1.34 releases it should be fixed
extern crate criterion;
extern crate num;
extern crate typenum;

#[cfg(test)]
mod tests;

mod board;

pub use board::*;
