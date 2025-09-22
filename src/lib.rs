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
//! - Modular architecture for easy maintenance
//!
//! ## Architecture
//!
//! The crate is organized into several modules:
//! - `simulation`: Core simulation logic and cell management
//! - `rendering`: Visual rendering of cells and grid
//! - `ui`: User interface and interaction handling
//! - `config`: Configuration parameters and constants
//! - `utils`: Utility functions and diagnostic tools

pub mod simulation;
pub mod rendering;
pub mod ui;
pub mod config;
pub mod utils;