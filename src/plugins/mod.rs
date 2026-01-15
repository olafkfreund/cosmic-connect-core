//! Plugin system module
//!
//! KDE Connect plugin trait and implementations.
//!
//! This module contains:
//! - `Plugin`: Unified plugin trait
//! - `PluginManager`: Plugin lifecycle management
//! - Plugin implementations (ping, battery, share, etc.)

// Module exports
// pub mod plugin_trait;  // TODO: Create unified Plugin trait (Issue #49)
// pub mod manager;       // TODO: Create PluginManager
// pub mod ping;          // TODO: Extract from applet
// pub mod battery;       // TODO: Extract from applet
// pub mod share;         // TODO: Extract from applet
// pub mod clipboard;     // TODO: Extract from applet
// pub mod mpris;         // TODO: Extract from applet
// pub mod notifications; // TODO: Extract from applet
// pub mod findmyphone;   // TODO: Extract from applet
// pub mod runcommand;    // TODO: Extract from applet

// Re-exports for convenience
// pub use plugin_trait::{Plugin, PluginCapabilities};
// pub use manager::PluginManager;

// Placeholder for now
#[allow(dead_code)]
const MAX_PLUGINS: usize = 32;
