//! AudioStream plugin
//!
//! Allows streaming audio between devices (phone ↔ desktop).
//! Supports codec negotiation and bidirectional audio streaming.

use crate::protocol::Packet;
use crate::error::Result;
use serde_json::json;

/// AudioStream status packet type
pub const PACKET_TYPE_AUDIOSTREAM: &str = "cconnect.audiostream";
/// AudioStream control request packet type
pub const PACKET_TYPE_AUDIOSTREAM_REQUEST: &str = "cconnect.audiostream.request";
/// AudioStream capability packet type
pub const PACKET_TYPE_AUDIOSTREAM_CAPABILITY: &str = "cconnect.audiostream.capability";

/// Create an audio stream status packet
///
/// # Arguments
///
/// * `is_streaming` - Whether audio is currently streaming
/// * `codec` - Active codec (e.g., "opus", "aac")
/// * `sample_rate` - Sample rate in Hz
/// * `channels` - Number of audio channels (1=mono, 2=stereo)
/// * `direction` - Stream direction ("phone_to_desktop" or "desktop_to_phone")
pub fn create_audiostream_status(
    is_streaming: bool,
    codec: &str,
    sample_rate: i32,
    channels: i32,
    direction: &str,
) -> Result<Packet> {
    Ok(Packet::new(PACKET_TYPE_AUDIOSTREAM, json!({
        "isStreaming": is_streaming,
        "codec": codec,
        "sampleRate": sample_rate,
        "channels": channels,
        "direction": direction,
    })))
}

/// Create an audio stream start request packet
///
/// # Arguments
///
/// * `codec` - Requested codec
/// * `sample_rate` - Requested sample rate in Hz
/// * `channels` - Requested number of channels
/// * `direction` - Requested stream direction
pub fn create_audiostream_start_request(
    codec: &str,
    sample_rate: i32,
    channels: i32,
    direction: &str,
) -> Result<Packet> {
    Ok(Packet::new(PACKET_TYPE_AUDIOSTREAM_REQUEST, json!({
        "startStreaming": true,
        "codec": codec,
        "sampleRate": sample_rate,
        "channels": channels,
        "direction": direction,
    })))
}

/// Create an audio stream stop request packet
pub fn create_audiostream_stop_request() -> Result<Packet> {
    Ok(Packet::new(PACKET_TYPE_AUDIOSTREAM_REQUEST, json!({
        "stopStreaming": true,
    })))
}

/// Create an audio stream capability query packet
pub fn create_audiostream_capability_query() -> Result<Packet> {
    Ok(Packet::new(PACKET_TYPE_AUDIOSTREAM_CAPABILITY, json!({})))
}

/// Create an audio stream capability response packet
///
/// # Arguments
///
/// * `codecs_json` - JSON array of supported codecs (e.g., "[\"opus\",\"aac\"]")
/// * `sample_rates_json` - JSON array of supported sample rates (e.g., "[44100,48000]")
/// * `max_channels` - Maximum number of channels supported
pub fn create_audiostream_capability_response(
    codecs_json: &str,
    sample_rates_json: &str,
    max_channels: i32,
) -> Result<Packet> {
    Ok(Packet::new(PACKET_TYPE_AUDIOSTREAM_CAPABILITY, json!({
        "supportedCodecs": codecs_json,
        "sampleRates": sample_rates_json,
        "maxChannels": max_channels,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_audiostream_status_streaming() {
        let packet = create_audiostream_status(
            true,
            "opus",
            48000,
            2,
            "phone_to_desktop"
        ).unwrap();
        assert_eq!(packet.packet_type, "cconnect.audiostream");
        assert_eq!(packet.body["isStreaming"], true);
        assert_eq!(packet.body["codec"], "opus");
        assert_eq!(packet.body["sampleRate"], 48000);
        assert_eq!(packet.body["channels"], 2);
        assert_eq!(packet.body["direction"], "phone_to_desktop");
    }

    #[test]
    fn test_create_audiostream_status_not_streaming() {
        let packet = create_audiostream_status(
            false,
            "aac",
            44100,
            1,
            "desktop_to_phone"
        ).unwrap();
        assert_eq!(packet.body["isStreaming"], false);
        assert_eq!(packet.body["codec"], "aac");
        assert_eq!(packet.body["sampleRate"], 44100);
        assert_eq!(packet.body["channels"], 1);
        assert_eq!(packet.body["direction"], "desktop_to_phone");
    }

    #[test]
    fn test_create_audiostream_start_request() {
        let packet = create_audiostream_start_request(
            "opus",
            48000,
            2,
            "phone_to_desktop"
        ).unwrap();
        assert_eq!(packet.packet_type, "cconnect.audiostream.request");
        assert_eq!(packet.body["startStreaming"], true);
        assert_eq!(packet.body["codec"], "opus");
        assert_eq!(packet.body["sampleRate"], 48000);
        assert_eq!(packet.body["channels"], 2);
        assert_eq!(packet.body["direction"], "phone_to_desktop");
    }

    #[test]
    fn test_create_audiostream_stop_request() {
        let packet = create_audiostream_stop_request().unwrap();
        assert_eq!(packet.packet_type, "cconnect.audiostream.request");
        assert_eq!(packet.body["stopStreaming"], true);
    }

    #[test]
    fn test_create_audiostream_capability_query() {
        let packet = create_audiostream_capability_query().unwrap();
        assert_eq!(packet.packet_type, "cconnect.audiostream.capability");
    }

    #[test]
    fn test_create_audiostream_capability_response() {
        let packet = create_audiostream_capability_response(
            "[\"opus\",\"aac\"]",
            "[44100,48000]",
            2
        ).unwrap();
        assert_eq!(packet.packet_type, "cconnect.audiostream.capability");
        assert_eq!(packet.body["supportedCodecs"], "[\"opus\",\"aac\"]");
        assert_eq!(packet.body["sampleRates"], "[44100,48000]");
        assert_eq!(packet.body["maxChannels"], 2);
    }
}
