//! ganit-ai — AI integration: daimon client, hoosh inference.
//!
//! Provides the standard AGNOS daimon/hoosh client pattern for the ganit agent.

mod daimon;

pub use daimon::{DaimonClient, DaimonError};

/// Agent name used when registering with daimon.
pub const AGENT_NAME: &str = "ganit";

/// Default daimon API URL.
pub const DEFAULT_DAIMON_URL: &str = "http://localhost:8090";

/// Default hoosh (LLM gateway) API URL.
pub const DEFAULT_HOOSH_URL: &str = "http://localhost:8088";
