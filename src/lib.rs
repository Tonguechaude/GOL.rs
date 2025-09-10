//! # Conway's Game of Life
//!
//! A Rust implementation of Conway's Game of Life using the Bevy game engine.
//! This crate provides a cellular automaton simulation that can run both as a
//! desktop application and in web browsers using WebAssembly.
//!
//! ## Features
//!
//! - Real-time Game of Life simulation
//! - Interactive GUI controls
//! - Configurable simulation speed
//! - Step-by-step mode for debugging
//!
//! ## Usage
//!
//! ```no_run
//! use jeu_de_la_vie::cellule::CellPosition;
//!
//! let cell = CellPosition { x: 0, y: 0 };
//! ```

pub mod cellule;
pub mod gui;
