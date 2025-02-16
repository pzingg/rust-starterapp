//! Jelly implements various helpers, traits, and types for actix-web that
//! enable a nicer building experience. It's not released as a framework,
//! as I don't think this works long-term - instead, clone and chisel away
//! to get what you need.

// We re-export/hoist a few things that are commonly imported.
// Less time screwing around with Cargo.toml for a framework-feel is
// ideal.
pub use actix_rt;
pub use actix_service;
pub use actix_session;
pub use actix_web;
pub use anyhow;
pub use async_trait;
pub use chrono;
pub use djangohashers;
pub use futures;
pub use serde;
pub use serde_json;
pub use sqlx;
pub use tera;

#[cfg(feature = "oauth")]
pub use oauth2;

#[macro_use]
pub extern crate log;

pub mod accounts;
pub mod email;
pub mod error;
pub mod forms;
pub mod guards;
pub mod jobs;
pub mod prelude;
pub mod request;
pub mod utils;

mod server;
mod templates;
pub use server::{Server, ServerConfig};

#[cfg(feature = "oauth")]
pub mod oauth;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub const NO_PASSWORD: Option<String> = None;
pub const SESSION_FLASH: &str = "flsh";
pub const SESSION_USER: &str = "sku";

#[cfg(feature = "oauth")]
pub const SESSION_OAUTH_FLOW: &str = "oflw";
pub const SESSION_OAUTH_TOKEN: &str = "rfsh";
