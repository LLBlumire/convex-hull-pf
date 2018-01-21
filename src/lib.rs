//! Provides tools for calculating a path around a set of polygons through a set of points using
//! convex hulls.

#![warn(missing_docs)]

#[macro_use]
extern crate serde_derive;

extern crate image;
extern crate serde;

pub mod io;
pub mod process;
pub mod shape;
