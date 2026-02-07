//! ScreenShare plugin
//!
//! Allows screen sharing between devices with configurable resolution, codec, and direction.
//! Reports sharing status and accepts control requests.

use crate::protocol::Packet;
use crate::error::Result;
use serde_json::{json, Value};

/// ScreenShare status packet type
pub const PACKET_TYPE_SCREENSHARE: &str = "cconnect.screenshare";
/// ScreenShare request packet type
pub const PACKET_TYPE_SCREENSHARE_REQUEST: &str = "cconnect.screenshare.request";

/// Create a screen share status packet
///
/// # Arguments
/// * `is_sharing` - Whether screen sharing is currently active
/// * `width` - Optional frame width in pixels
/// * `height` - Optional frame height in pixels
/// * `codec` - Optional video codec (e.g., "h264", "vp8")
/// * `fps` - Optional frames per second
/// * `direction` - Direction of sharing ("phone_to_desktop" or "desktop_to_phone")
pub fn create_screenshare_status(
    is_sharing: bool,
    width: Option<i32>,
    height: Option<i32>,
    codec: Option<String>,
    fps: Option<i32>,
    direction: &str,
) -> Result<Packet> {
    let mut body = json!({
        "isSharing": is_sharing,
        "direction": direction,
    });

    if let Some(w) = width {
        body["width"] = Value::from(w);
    }
    if let Some(h) = height {
        body["height"] = Value::from(h);
    }
    if let Some(c) = codec {
        body["codec"] = Value::from(c);
    }
    if let Some(f) = fps {
        body["fps"] = Value::from(f);
    }

    Ok(Packet::new(PACKET_TYPE_SCREENSHARE, body))
}

/// Create a screen share start request packet
///
/// # Arguments
/// * `width` - Requested frame width in pixels
/// * `height` - Requested frame height in pixels
/// * `codec` - Requested video codec
/// * `fps` - Requested frames per second
/// * `direction` - Requested direction of sharing
/// * `enable_input` - Whether to enable remote input forwarding
pub fn create_screenshare_start_request(
    width: i32,
    height: i32,
    codec: &str,
    fps: i32,
    direction: &str,
    enable_input: bool,
) -> Result<Packet> {
    Ok(Packet::new(
        PACKET_TYPE_SCREENSHARE_REQUEST,
        json!({
            "startSharing": true,
            "width": width,
            "height": height,
            "codec": codec,
            "fps": fps,
            "direction": direction,
            "enableInput": enable_input,
        }),
    ))
}

/// Create a screen share stop request packet
pub fn create_screenshare_stop_request() -> Result<Packet> {
    Ok(Packet::new(
        PACKET_TYPE_SCREENSHARE_REQUEST,
        json!({"stopSharing": true}),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_screenshare_status_sharing() {
        let packet = create_screenshare_status(
            true,
            Some(1920),
            Some(1080),
            Some("h264".to_string()),
            Some(30),
            "phone_to_desktop",
        )
        .unwrap();
        assert_eq!(packet.packet_type, "cconnect.screenshare");
        assert_eq!(packet.body["isSharing"], true);
        assert_eq!(packet.body["width"], 1920);
        assert_eq!(packet.body["height"], 1080);
        assert_eq!(packet.body["codec"], "h264");
        assert_eq!(packet.body["fps"], 30);
        assert_eq!(packet.body["direction"], "phone_to_desktop");
    }

    #[test]
    fn test_create_screenshare_status_not_sharing() {
        let packet = create_screenshare_status(false, None, None, None, None, "desktop_to_phone")
            .unwrap();
        assert_eq!(packet.body["isSharing"], false);
        assert_eq!(packet.body["direction"], "desktop_to_phone");
        assert!(packet.body.get("width").is_none());
        assert!(packet.body.get("height").is_none());
        assert!(packet.body.get("codec").is_none());
        assert!(packet.body.get("fps").is_none());
    }

    #[test]
    fn test_create_screenshare_status_partial_metadata() {
        let packet = create_screenshare_status(
            true,
            Some(1280),
            Some(720),
            None,
            Some(24),
            "phone_to_desktop",
        )
        .unwrap();
        assert_eq!(packet.body["width"], 1280);
        assert_eq!(packet.body["height"], 720);
        assert_eq!(packet.body["fps"], 24);
        assert!(packet.body.get("codec").is_none());
    }

    #[test]
    fn test_create_screenshare_start_request() {
        let packet = create_screenshare_start_request(
            1920,
            1080,
            "vp8",
            60,
            "desktop_to_phone",
            true,
        )
        .unwrap();
        assert_eq!(packet.packet_type, "cconnect.screenshare.request");
        assert_eq!(packet.body["startSharing"], true);
        assert_eq!(packet.body["width"], 1920);
        assert_eq!(packet.body["height"], 1080);
        assert_eq!(packet.body["codec"], "vp8");
        assert_eq!(packet.body["fps"], 60);
        assert_eq!(packet.body["direction"], "desktop_to_phone");
        assert_eq!(packet.body["enableInput"], true);
    }

    #[test]
    fn test_create_screenshare_start_request_no_input() {
        let packet =
            create_screenshare_start_request(800, 600, "h264", 30, "phone_to_desktop", false)
                .unwrap();
        assert_eq!(packet.body["enableInput"], false);
    }

    #[test]
    fn test_create_screenshare_stop_request() {
        let packet = create_screenshare_stop_request().unwrap();
        assert_eq!(packet.packet_type, "cconnect.screenshare.request");
        assert_eq!(packet.body["stopSharing"], true);
    }

    #[test]
    fn test_screenshare_direction_values() {
        let packet1 = create_screenshare_status(true, None, None, None, None, "phone_to_desktop")
            .unwrap();
        assert_eq!(packet1.body["direction"], "phone_to_desktop");

        let packet2 = create_screenshare_status(true, None, None, None, None, "desktop_to_phone")
            .unwrap();
        assert_eq!(packet2.body["direction"], "desktop_to_phone");
    }
}
