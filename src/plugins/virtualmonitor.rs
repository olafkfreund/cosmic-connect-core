//! Virtual Monitor plugin
//!
//! Allows using Android device as an additional display for the desktop.
//! Reports virtual monitor status and accepts enable/disable commands.

use crate::protocol::Packet;
use crate::error::Result;
use serde_json::json;

/// Virtual monitor status packet type
pub const PACKET_TYPE_VIRTUALMONITOR: &str = "cconnect.virtualmonitor";
/// Virtual monitor request packet type
pub const PACKET_TYPE_VIRTUALMONITOR_REQUEST: &str = "cconnect.virtualmonitor.request";

/// Create a virtual monitor status packet
pub fn create_virtualmonitor_status(
    is_active: bool,
    width: Option<i32>,
    height: Option<i32>,
    dpi: Option<i32>,
    position: Option<String>,
    refresh_rate: Option<i32>,
) -> Result<Packet> {
    let mut body = json!({"isActive": is_active});

    if let Some(w) = width {
        body["width"] = json!(w);
    }
    if let Some(h) = height {
        body["height"] = json!(h);
    }
    if let Some(d) = dpi {
        body["dpi"] = json!(d);
    }
    if let Some(p) = position {
        body["position"] = json!(p);
    }
    if let Some(r) = refresh_rate {
        body["refreshRate"] = json!(r);
    }

    Ok(Packet::new(PACKET_TYPE_VIRTUALMONITOR, body))
}

/// Create a virtual monitor enable request packet
pub fn create_virtualmonitor_enable_request(
    width: i32,
    height: i32,
    dpi: i32,
    position: &str,
    refresh_rate: i32,
) -> Result<Packet> {
    Ok(Packet::new(
        PACKET_TYPE_VIRTUALMONITOR_REQUEST,
        json!({
            "enableMonitor": true,
            "width": width,
            "height": height,
            "dpi": dpi,
            "position": position,
            "refreshRate": refresh_rate,
        }),
    ))
}

/// Create a virtual monitor disable request packet
pub fn create_virtualmonitor_disable_request() -> Result<Packet> {
    Ok(Packet::new(
        PACKET_TYPE_VIRTUALMONITOR_REQUEST,
        json!({"disableMonitor": true}),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_status_active_minimal() {
        let packet = create_virtualmonitor_status(true, None, None, None, None, None).unwrap();
        assert_eq!(packet.packet_type, "cconnect.virtualmonitor");
        assert_eq!(packet.body["isActive"], true);
        assert!(packet.body.get("width").is_none());
    }

    #[test]
    fn test_create_status_inactive() {
        let packet = create_virtualmonitor_status(false, None, None, None, None, None).unwrap();
        assert_eq!(packet.body["isActive"], false);
    }

    #[test]
    fn test_create_status_with_full_config() {
        let packet = create_virtualmonitor_status(
            true,
            Some(1920),
            Some(1080),
            Some(240),
            Some("right".to_string()),
            Some(60),
        )
        .unwrap();

        assert_eq!(packet.body["isActive"], true);
        assert_eq!(packet.body["width"], 1920);
        assert_eq!(packet.body["height"], 1080);
        assert_eq!(packet.body["dpi"], 240);
        assert_eq!(packet.body["position"], "right");
        assert_eq!(packet.body["refreshRate"], 60);
    }

    #[test]
    fn test_create_enable_request() {
        let packet = create_virtualmonitor_enable_request(1920, 1080, 240, "left", 60).unwrap();
        assert_eq!(packet.packet_type, "cconnect.virtualmonitor.request");
        assert_eq!(packet.body["enableMonitor"], true);
        assert_eq!(packet.body["width"], 1920);
        assert_eq!(packet.body["height"], 1080);
        assert_eq!(packet.body["dpi"], 240);
        assert_eq!(packet.body["position"], "left");
        assert_eq!(packet.body["refreshRate"], 60);
    }

    #[test]
    fn test_create_disable_request() {
        let packet = create_virtualmonitor_disable_request().unwrap();
        assert_eq!(packet.packet_type, "cconnect.virtualmonitor.request");
        assert_eq!(packet.body["disableMonitor"], true);
        assert!(packet.body.get("enableMonitor").is_none());
    }

    #[test]
    fn test_status_with_partial_config() {
        let packet = create_virtualmonitor_status(
            true,
            Some(1920),
            Some(1080),
            None,
            Some("above".to_string()),
            None,
        )
        .unwrap();

        assert_eq!(packet.body["width"], 1920);
        assert_eq!(packet.body["height"], 1080);
        assert_eq!(packet.body["position"], "above");
        assert!(packet.body.get("dpi").is_none());
        assert!(packet.body.get("refreshRate").is_none());
    }

    #[test]
    fn test_position_values() {
        for pos in &["left", "right", "above", "below"] {
            let packet = create_virtualmonitor_status(
                true,
                None,
                None,
                None,
                Some(pos.to_string()),
                None,
            )
            .unwrap();
            assert_eq!(packet.body["position"], *pos);
        }
    }
}
