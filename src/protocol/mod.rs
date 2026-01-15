//! Protocol module
//!
//! Core KDE Connect protocol types and implementations.
//!
//! This module contains:
//! - `Packet`: NetworkPacket serialization/deserialization
//! - `Device`: Device information and identity
//! - `Identity`: Device identity packets
//! - `Payload`: Payload transfer handling

// Module exports
// pub mod packet;    // TODO: Extract from applet (Issue #45)
// pub mod device;    // TODO: Extract from applet
// pub mod identity;  // TODO: Create
// pub mod payload;   // TODO: Extract from applet

// Re-exports for convenience
// pub use packet::Packet;
// pub use device::{Device, DeviceInfo, DeviceType};
// pub use identity::Identity;

// Placeholder for now
#[allow(dead_code)]
const PROTOCOL_VERSION: i32 = 7;
