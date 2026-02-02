//! Network module
//!
//! Network communication layer for KDE Connect protocol.
//!
//! ## Implemented Modules
//!
//! - [`discovery`] - UDP device discovery on port 1716
//! - [`transport`] - Transport abstraction (TCP, Bluetooth)
//!
//! ## Planned Modules
//!
//! The following modules require extraction from the desktop applet:
//!
//! ### tcp
//! - **Status**: Planned extraction
//! - **Description**: TCP connection management for device communication
//! - **Location**: Currently in `cosmic-connect-daemon/src/device.rs`
//! - **Requirements**: Extract and adapt for cross-platform FFI use
//! - **Features**: Connection pooling, automatic reconnection, port fallback (1716-1764)
//!
//! ### tls
//! - **Status**: Completed (Issue #47)
//! - **Description**: Secure TLS connections using rustls
//! - **Location**: Integrated into transport layer
//! - **Notes**: Uses self-signed certificates with SHA-256 fingerprints

// Module exports
pub mod discovery;  // ✅ Extracted (Issue #46)
pub mod transport;  // ✅ Transport abstraction layer

// Re-exports for convenience
pub use discovery::{
    DeviceInfo, DeviceType, Discovery, DiscoveryConfig, DiscoveryEvent, DiscoveryService,
    BROADCAST_ADDR, DEFAULT_BROADCAST_INTERVAL, DEFAULT_DEVICE_TIMEOUT, DISCOVERY_PORT,
    PORT_RANGE_END, PORT_RANGE_START,
};

pub use transport::{
    LatencyCategory, Transport, TransportAddress, TransportCapabilities, TransportFactory,
    TransportPreference, TransportType, KDECONNECT_SERVICE_UUID, MAX_BT_PACKET_SIZE,
    MAX_TCP_PACKET_SIZE, RFCOMM_READ_CHAR_UUID, RFCOMM_WRITE_CHAR_UUID,
};

// pub use tcp::TcpTransport;
// pub use tls::TlsTransport;
