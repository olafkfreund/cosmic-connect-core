//! Network module
//!
//! Network communication layer for KDE Connect protocol.
//!
//! This module contains:
//! - `Discovery`: UDP device discovery on port 1716
//! - `TcpTransport`: TCP connection management
//! - `TlsTransport`: Secure TLS connections

// Module exports
pub mod discovery; // ✅ Extracted (Issue #46)
// pub mod tcp;     // TODO: Extract from applet
// pub mod tls;     // TODO: Rewrite with rustls (Issue #47)

// Re-exports for convenience
pub use discovery::{
    DeviceInfo, DeviceType, Discovery, DiscoveryConfig, DiscoveryEvent, DiscoveryService,
    BROADCAST_ADDR, DEFAULT_BROADCAST_INTERVAL, DEFAULT_DEVICE_TIMEOUT, DISCOVERY_PORT,
    PORT_RANGE_END, PORT_RANGE_START,
};
// pub use tcp::TcpTransport;
// pub use tls::TlsTransport;
