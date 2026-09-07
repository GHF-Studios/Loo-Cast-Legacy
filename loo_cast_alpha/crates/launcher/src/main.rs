//! This crate owns the launcher UI/UX and meta-management workflow around launching
//! the game with selected mod/modpack configurations.
//! The launcher can create, modify, and (re-)configure modpacks, just not raw mods.
//! So: The launcher is not responsible for mod authoring or SDK workflows.
//!
//! # Audience scope:
//!
//! - Casual players: launch the game.
//! - Involved players: download/select modpacks, then launch.
//! - Technical users: compose local playable setups from downloaded compatible mods
//!   and modpacks.
//!
//! # Module layout
//!
//! - [`app`]: startup/lifecycle composition.
//! - [`ui`]: egui rendering surfaces and interaction handling.
//! - [`state`]: route and launcher state ownership.
//! - [`domain`]: shared launcher data types.
//! - [`services`]: install/update/launch contracts.
//! - [`storage`]: local persistence boundaries.
//! - [`platform`]: OS-level process/path adapters.
//! - [`tasks`]: async/background operation coordination.
//! - [`telemetry`]: diagnostics and activity/event/log feeds.
//!
//! # Alpha behavior
//!
//! In alpha bootstrap, this crate is intentionally layout-first: module boundaries and
//! UX flow are defined before full runtime/modding integration is wired.
//!
//! # Flow model:
//! `ui` emits intent -> `state` validates/selects action -> `services` and `tasks`
//! execute -> `state` updates -> `ui` re-renders.

pub mod app;
pub mod domain;
pub mod platform;
pub mod services;
pub mod state;
pub mod storage;
pub mod tasks;
pub mod telemetry;
pub mod ui;

fn main() -> eframe::Result {
    app::run()
}
