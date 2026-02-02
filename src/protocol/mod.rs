//! Protocol module
//!
//! Core KDE Connect protocol types and implementations.
//!
//! ## Implemented Modules
//!
//! - [`packet`] - NetworkPacket serialization/deserialization (Issue #45)
//!
//! ## Planned Modules
//!
//! The following modules are planned for extraction from the desktop applet:
//!
//! ### device
//! - **Status**: Planned extraction (Issue #46)
//! - **Description**: Device information, state management, and pairing
//! - **Location**: Currently in `cosmic-connect-daemon/src/device.rs`
//! - **Requirements**: Refactor for FFI compatibility, separate platform-specific UI code
//! - **Components**: Device struct, pairing state machine, certificate management
//! - **Blockers**: Multiple plugins depend on this extraction (telephony, contacts, clipboard, etc.)
//!
//! ### identity
//! - **Status**: Planned creation
//! - **Description**: Device identity packet generation and validation
//! - **Requirements**: Extract from Device implementation
//! - **Components**: Identity packet builder, capability negotiation, device type enum
//! - **Dependencies**: Requires `device` module extraction first
//!
//! ### payload
//! - **Status**: Planned extraction
//! - **Description**: Large file/data payload transfer handling
//! - **Location**: Currently in `cosmic-connect-daemon/src/device.rs` and plugins
//! - **Components**: Payload metadata, streaming transfer, progress tracking
//! - **Use cases**: File sharing, camera streaming, clipboard large content

// Module exports
pub mod packet;       // ✅ Extracted from applet (Issue #45)

// Re-exports for convenience
pub use packet::Packet;
// pub use device::{Device, DeviceInfo, DeviceType};
// pub use identity::Identity;

/// KDE Connect protocol version implemented by this library
/// Updated to version 8 to match latest KDE Connect Android app
pub const PROTOCOL_VERSION: i32 = 8;
