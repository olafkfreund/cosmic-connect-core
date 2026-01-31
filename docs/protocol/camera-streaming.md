# Camera Streaming Protocol Specification

> Version: 1.0.0
> Status: Draft
> Last Updated: 2026-01-31

## 1. Introduction

### 1.1 Purpose

This document specifies the camera streaming protocol for COSMIC Connect, enabling Android device cameras to be used as virtual webcams on COSMIC Desktop through V4L2 loopback.

### 1.2 Scope

- Camera capability advertisement
- Stream negotiation and control
- H.264 video frame transport
- Error handling and recovery

### 1.3 Terminology

| Term | Definition |
|------|------------|
| **Desktop** | COSMIC Desktop running cosmic-connect-daemon |
| **Phone** | Android device running cosmic-connect-android |
| **V4L2** | Video4Linux2 - Linux video device API |
| **NAL Unit** | Network Abstraction Layer unit (H.264) |
| **SPS/PPS** | Sequence/Picture Parameter Sets (decoder config) |
| **I-Frame** | Intra-frame (keyframe, independently decodable) |
| **P-Frame** | Predicted frame (delta, depends on previous) |

## 2. Protocol Overview

### 2.1 Packet Types

| Packet Type | Direction | Description |
|-------------|-----------|-------------|
| `cconnect.camera.capability` | Phone → Desktop | Camera capabilities |
| `cconnect.camera.start` | Desktop → Phone | Start streaming |
| `cconnect.camera.stop` | Desktop → Phone | Stop streaming |
| `cconnect.camera.settings` | Desktop → Phone | Change settings |
| `cconnect.camera.frame` | Phone → Desktop | Video frame data |
| `cconnect.camera.status` | Phone → Desktop | Status updates |

### 2.2 Connection Flow

```
Desktop                                         Phone
   │                                              │
   │◄──────── cconnect.camera.capability ─────────│
   │          (cameras, codecs, resolutions)      │
   │                                              │
   │                                              │
   │──────── cconnect.camera.start ──────────────►│
   │          (camera_id, resolution, fps)        │
   │                                              │
   │◄──────── cconnect.camera.status ─────────────│
   │          (status: starting)                  │
   │                                              │
   │◄──────── cconnect.camera.frame ──────────────│
   │          (SPS/PPS decoder config)            │
   │                                              │
   │◄──────── cconnect.camera.status ─────────────│
   │          (status: streaming)                 │
   │                                              │
   │◄──────── cconnect.camera.frame ──────────────│
   │          (I-Frame keyframe)                  │
   │                                              │
   │◄──────── cconnect.camera.frame ──────────────│
   │◄──────── cconnect.camera.frame ──────────────│
   │          (P-Frames)                          │
   │              ...                             │
   │                                              │
   │──────── cconnect.camera.stop ───────────────►│
   │                                              │
   │◄──────── cconnect.camera.status ─────────────│
   │          (status: stopped)                   │
   │                                              │
```

## 3. Packet Definitions

### 3.1 Camera Capability

Advertises available cameras and streaming capabilities.

**Packet Type:** `cconnect.camera.capability`
**Direction:** Phone → Desktop
**Timing:** Sent after device connection/pairing

```json
{
  "id": 1706745600000,
  "type": "cconnect.camera.capability",
  "body": {
    "cameras": [
      {
        "id": 0,
        "name": "Back Camera",
        "facing": "back",
        "maxResolution": { "width": 1920, "height": 1080 },
        "resolutions": [
          { "width": 1920, "height": 1080 },
          { "width": 1280, "height": 720 },
          { "width": 854, "height": 480 }
        ]
      },
      {
        "id": 1,
        "name": "Front Camera",
        "facing": "front",
        "maxResolution": { "width": 1280, "height": 720 },
        "resolutions": [
          { "width": 1280, "height": 720 },
          { "width": 854, "height": 480 }
        ]
      }
    ],
    "supportedCodecs": ["h264"],
    "audioSupported": false,
    "maxResolution": { "width": 1920, "height": 1080 },
    "maxBitrate": 8000,
    "maxFps": 60
  }
}
```

**Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cameras` | array | Yes | List of available cameras |
| `cameras[].id` | u32 | Yes | Camera identifier |
| `cameras[].name` | string | Yes | Human-readable name |
| `cameras[].facing` | string | Yes | "front", "back", or "external" |
| `cameras[].maxResolution` | Resolution | Yes | Maximum resolution |
| `cameras[].resolutions` | array | Yes | Supported resolutions |
| `supportedCodecs` | array | Yes | Supported codecs (currently only "h264") |
| `audioSupported` | bool | Yes | Whether audio streaming is supported |
| `maxResolution` | Resolution | Yes | Maximum total resolution |
| `maxBitrate` | u32 | Yes | Maximum bitrate in kbps |
| `maxFps` | u32 | Yes | Maximum frame rate |

### 3.2 Camera Start

Request to start camera streaming.

**Packet Type:** `cconnect.camera.start`
**Direction:** Desktop → Phone

```json
{
  "id": 1706745600001,
  "type": "cconnect.camera.start",
  "body": {
    "cameraId": 0,
    "resolution": { "width": 1280, "height": 720 },
    "fps": 30,
    "bitrate": 2000,
    "codec": "h264"
  }
}
```

**Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cameraId` | u32 | Yes | Camera to use |
| `resolution` | Resolution | Yes | Requested resolution |
| `fps` | u32 | Yes | Target frame rate |
| `bitrate` | u32 | Yes | Target bitrate in kbps |
| `codec` | string | Yes | Video codec ("h264") |

**Recommended Settings:**

| Quality | Resolution | FPS | Bitrate |
|---------|------------|-----|---------|
| Low | 854x480 | 30 | 1000 kbps |
| Medium | 1280x720 | 30 | 2000 kbps |
| High | 1920x1080 | 30 | 4000 kbps |

### 3.3 Camera Stop

Request to stop camera streaming.

**Packet Type:** `cconnect.camera.stop`
**Direction:** Desktop → Phone

```json
{
  "id": 1706745600002,
  "type": "cconnect.camera.stop",
  "body": {}
}
```

### 3.4 Camera Settings

Change streaming settings while active.

**Packet Type:** `cconnect.camera.settings`
**Direction:** Desktop → Phone

```json
{
  "id": 1706745600003,
  "type": "cconnect.camera.settings",
  "body": {
    "cameraId": 1,
    "resolution": { "width": 1280, "height": 720 },
    "flash": true
  }
}
```

**Fields (all optional):**

| Field | Type | Description |
|-------|------|-------------|
| `cameraId` | u32 | Switch to different camera |
| `resolution` | Resolution | Change resolution |
| `fps` | u32 | Change frame rate |
| `bitrate` | u32 | Change bitrate |
| `flash` | bool | Enable/disable flash |
| `autofocus` | bool | Enable/disable autofocus |

### 3.5 Camera Frame

Video frame data.

**Packet Type:** `cconnect.camera.frame`
**Direction:** Phone → Desktop

```json
{
  "id": 1706745600004,
  "type": "cconnect.camera.frame",
  "body": {
    "frameType": "iframe",
    "timestampUs": 1234567890,
    "sequenceNumber": 42,
    "size": 65536
  },
  "payloadSize": 65536
}
```

**Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `frameType` | string | Frame type: "sps_pps", "iframe", "pframe" |
| `timestampUs` | u64 | Presentation timestamp in microseconds |
| `sequenceNumber` | u64 | Frame sequence number |
| `size` | u64 | Frame data size in bytes |

**Frame Types:**

| Type | Value | Description |
|------|-------|-------------|
| SPS/PPS | `sps_pps` | Decoder configuration (must be first) |
| I-Frame | `iframe` | Keyframe, independently decodable |
| P-Frame | `pframe` | Delta frame, depends on previous |

**Payload Format:**
- Raw H.264 NAL units
- Annex B format with start codes (0x00 0x00 0x00 0x01)
- Sent via payload transfer mechanism

### 3.6 Camera Status

Streaming status updates.

**Packet Type:** `cconnect.camera.status`
**Direction:** Phone → Desktop

```json
{
  "id": 1706745600005,
  "type": "cconnect.camera.status",
  "body": {
    "status": "streaming",
    "cameraId": 0,
    "resolution": { "width": 1280, "height": 720 },
    "fps": 30,
    "bitrate": 2000
  }
}
```

**Status Values:**

| Status | Description |
|--------|-------------|
| `starting` | Camera is initializing |
| `streaming` | Actively streaming frames |
| `stopping` | Stopping stream |
| `stopped` | Stream stopped |
| `error` | Error occurred (see `error` field) |

**Error Status:**
```json
{
  "status": "error",
  "cameraId": 0,
  "resolution": { "width": 0, "height": 0 },
  "fps": 0,
  "bitrate": 0,
  "error": "Camera access denied"
}
```

## 4. Frame Transport

### 4.1 Payload Transfer

Frame data is transferred using the standard COSMIC Connect payload mechanism:

1. **Header Packet:** `cconnect.camera.frame` with `payloadSize` set
2. **Payload Data:** Raw frame bytes via payload channel

### 4.2 H.264 Stream Format

The video stream uses H.264/AVC format:

1. **Profile:** Baseline or Main Profile
2. **Level:** 3.1 or 4.0 (depending on resolution)
3. **NAL Unit Format:** Annex B with start codes
4. **Keyframe Interval:** Every 30 frames (1 second at 30fps)

### 4.3 Initial Frame Sequence

When streaming starts, frames MUST be sent in this order:

1. **SPS/PPS** - Decoder configuration (frame_type: "sps_pps")
2. **I-Frame** - First keyframe (frame_type: "iframe")
3. **P-Frames** - Delta frames until next keyframe

### 4.4 Keyframe Requirements

- Send keyframe at least every 2 seconds
- Send keyframe when settings change
- Send keyframe on stream resume after pause

## 5. Error Handling

### 5.1 Connection Loss

If the connection is lost during streaming:

**Phone Behavior:**
- Stop encoding immediately
- Release camera resources
- Clean up encoder

**Desktop Behavior:**
- Stop V4L2 injection
- Hold last frame or show black
- Wait for reconnection

### 5.2 Decoder Errors

If the desktop encounters decoder errors:

1. **Request Keyframe:** Send `cconnect.camera.settings` with no changes
2. **Phone Response:** Send new SPS/PPS + I-Frame
3. **Continue:** Resume normal streaming

### 5.3 Camera Errors

Common camera errors and handling:

| Error | Cause | Recovery |
|-------|-------|----------|
| Camera in use | Another app using camera | Retry after app releases |
| Permission denied | Camera permission missing | Request permission |
| Hardware error | Camera malfunction | Show error, allow retry |

## 6. V4L2 Loopback

### 6.1 Module Requirements

The v4l2loopback kernel module must be loaded:

```bash
# Load module with browser compatibility
sudo modprobe v4l2loopback exclusive_caps=1 card_label="COSMIC Connect"

# Verify device
ls -la /dev/video*
```

**Required Options:**
- `exclusive_caps=1` - Required for WebRTC browser compatibility
- `card_label` - Human-readable device name

### 6.2 Format Negotiation

The desktop should configure the V4L2 device with:

| Property | Value |
|----------|-------|
| Pixel Format | YUYV (V4L2_PIX_FMT_YUYV) |
| Width | Matches stream resolution |
| Height | Matches stream resolution |
| Field | Progressive (V4L2_FIELD_NONE) |

### 6.3 Frame Injection

Frames are injected via:

1. Decode H.264 to YUV (ffmpeg-next)
2. Convert YUV420 to YUYV if needed
3. Write to V4L2 device using VIDIOC_QBUF

## 7. Performance Requirements

### 7.1 Latency Targets

| Component | Target | Maximum |
|-----------|--------|---------|
| Encoding (Phone) | 10ms | 33ms |
| Network Transfer | 20ms | 50ms |
| Decoding (Desktop) | 10ms | 33ms |
| **Total End-to-End** | **50ms** | **150ms** |

### 7.2 Quality Metrics

| Metric | Target |
|--------|--------|
| Frame Rate | 30 fps stable |
| Frame Drops | < 1% |
| Keyframe Interval | 30 frames |
| Recovery Time | < 1 second |

### 7.3 Battery Considerations

To minimize battery drain on Android:

- Use hardware MediaCodec encoder
- Target 2-4 Mbps bitrate
- Reduce resolution when battery low
- Pause when screen off (optional)

## 8. Security Considerations

### 8.1 Transport Security

All packets are sent over the existing COSMIC Connect TLS 1.3 connection:

- Certificate pinning validates device identity
- No additional encryption needed for video data
- Camera permission verified by Android OS

### 8.2 Privacy

- Camera access requires explicit user action
- Streaming indicator visible on phone
- User can stop streaming at any time

## 9. Implementation Notes

### 9.1 Android Implementation

**Required Permissions:**
- `android.permission.CAMERA`
- `android.Manifest.permission.FOREGROUND_SERVICE_CAMERA` (Android 14+)

**MediaCodec Configuration:**
```kotlin
val format = MediaFormat.createVideoFormat(
    MediaFormat.MIMETYPE_VIDEO_AVC,
    width,
    height
)
format.setInteger(MediaFormat.KEY_BIT_RATE, bitrate * 1000)
format.setInteger(MediaFormat.KEY_FRAME_RATE, fps)
format.setInteger(MediaFormat.KEY_I_FRAME_INTERVAL, 1)
format.setInteger(MediaFormat.KEY_COLOR_FORMAT,
    MediaCodecInfo.CodecCapabilities.COLOR_FormatSurface)
```

### 9.2 Desktop Implementation

**Rust Dependencies:**
```toml
[dependencies]
v4l = "0.14"
ffmpeg-next = "6"
tokio = { version = "1", features = ["full"] }
```

**V4L2 Setup:**
```rust
let device = Device::new("/dev/video10")?;
let format = Format {
    width: 1280,
    height: 720,
    fourcc: FourCC::new(b"YUYV"),
    ..Default::default()
};
device.set_format(format)?;
```

## 10. References

- [V4L2 API Documentation](https://www.kernel.org/doc/html/latest/userspace-api/media/v4l/v4l2.html)
- [v4l2loopback](https://github.com/umlaeute/v4l2loopback)
- [Android Camera2 API](https://developer.android.com/reference/android/hardware/camera2/package-summary)
- [Android MediaCodec](https://developer.android.com/reference/android/media/MediaCodec)
- [H.264 Specification](https://www.itu.int/rec/T-REC-H.264)
- [COSMIC Connect Protocol](https://valent.andyholmes.ca/documentation/protocol.html)
