//! `embedded-canvas` is a convenient crate for [`embedded-graphics`]
//! and provides a [`Canvas`](#canvas) and [`CanvasAt`](#canvasat) on which you
//! can draw anything with ease before drawing the pixels on the embedded display.
//!
//! > canvas - a piece of cloth backed or framed as a surface for a painting
//!
//! Based on [`embedded-graphics-core`] and [`embedded-graphics`]
//! (see `transform` feature in [Crate features](#crate-features)).
//!
//! This crate is `no_std` but requires `alloc` for the time being.
//!
//! The main advantages of the canvases in this crate are:
//!
//! 1. **Transparency** - pixels that haven't been drawn, won't override pixels on the display.
//! 2. **Cropping** - The ability to crop leaves only the part of the canvas you want to
//! draw on the display. This is especially useful when you want to
//! partially show text, figures and images.
//!
//! [`embedded-graphics`]: https://crates.io/crates/embedded-graphics
//! [`embedded-graphics-core`]: https://crates.io/crates/embedded-graphics-core
//!
//! # How to work with canvases
//!
//! There are **two** main canvases you can work with:
//!
//! ## `Canvas`
//!
//! A [`Canvas`] which you can draw on with origin [`Point::zero()`](embedded_graphics_core::geometry::Point::zero).
//! The canvas location is not set for the provided display.
//!
//! After drawing decide where to place it on the display using the methods:
//! - [`Canvas::place_at(top_left: Point) -> CanvasAt`](Canvas::place_at)
//! - [`Canvas::place_center(center: Point) -> CanvasAt`](Canvas::place_center)
//!
//! ## `CanvasAt`
//!
//! [`CanvasAt`] is a type of canvas ready to be drawn on the display
//! at specified location (hence the name [`CanvasAt`]).
//!
//! There are two ways of using [`CanvasAt`]:
//!
//! 1. Directly placing the [`CanvasAt`] on specified location on the display and drawing inside.
//! 2. Create a [`Canvas`] and when ready to draw it on the display place the
//!  [`Canvas`] at specified location using the methods:
//!   - [`Canvas::place_at(top_left: Point) -> CanvasAt`](Canvas::place_at)
//!   - [`Canvas::place_center(center: Point) -> CanvasAt`](Canvas::place_center)
//!
//! # Crate features
//! - `default` features - `transform`
//! - `transform` - enables the trait implementation of [`embedded_graphics::transform::Transform`] for [`CanvasAt`].
#![no_std]

#[doc(inline)]
pub use canvas::{Canvas, CanvasAt};

mod canvas;
