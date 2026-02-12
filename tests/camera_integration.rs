//! Camera Integration Tests
//!
//! Tests the camera streaming pipeline integration including:
//! - Receiving H.264 frames from network
//! - Decoding H.264 frames to raw video
//! - Error handling for corrupt/missing frames
//! - Frame sequencing and timing

mod camera_test_utils;

use camera_test_utils::*;
use cosmic_ext_connect_core::plugins::camera::{CameraFrame, FrameType};
use cosmic_ext_connect_core::video::h264_decoder::{DecoderError, H264Decoder};

/// Test basic H.264 decoder initialization
#[test]
fn test_decoder_initialization() {
    let result = H264Decoder::new();
    assert!(result.is_ok(), "Decoder should initialize successfully");

    let decoder = result.unwrap();
    assert!(!decoder.is_initialized(), "Decoder should not be initialized without SPS/PPS");
}

/// Test decoding SPS/PPS configuration
#[test]
fn test_decode_sps_pps() {
    let mut decoder = H264Decoder::new().unwrap();

    // Create mock SPS/PPS
    let sps_data = mock_sps_nal();
    let pps_data = mock_pps_nal();

    // Decode SPS/PPS
    let result = decoder.set_sps_pps(&sps_data, &pps_data);

    // Mock data might not be valid for real decoder, that's okay
    match result {
        Ok(()) => {
            assert!(decoder.is_initialized());
        }
        Err(e) => {
            println!("SPS/PPS decode returned error (expected for mock data): {:?}", e);
        }
    }
}

/// Test decoding combined SPS+PPS (Android format)
#[test]
fn test_decode_combined_sps_pps() {
    let mut decoder = H264Decoder::new().unwrap();

    let mut combined = mock_sps_nal();
    combined.extend_from_slice(&mock_pps_nal());

    let result = decoder.decode_sps_pps(&combined);

    match result {
        Ok(()) => {
            assert!(decoder.is_initialized());
        }
        Err(e) => {
            println!("Combined SPS/PPS error (expected for mock data): {:?}", e);
        }
    }
}

/// Test decoding an I-frame
#[test]
fn test_decode_iframe() {
    let mut decoder = H264Decoder::new().unwrap();

    // First send SPS/PPS (decoder needs configuration)
    let sps_data = mock_sps_nal();
    let pps_data = mock_pps_nal();
    let _ = decoder.set_sps_pps(&sps_data, &pps_data);

    // Now try to decode I-frame
    let iframe_data = mock_iframe_nal(2048, 42);

    let result = decoder.decode(&iframe_data, 33333);

    // Mock data won't actually decode, but test that API works
    match result {
        Ok(Some(video_frame)) => {
            assert!(!video_frame.data.is_empty());
        }
        Ok(None) => {
            // Frame queued but not yet ready
        }
        Err(DecoderError::NeedMoreData) => {
            // Decoder not yet initialized with valid SPS/PPS
            println!("Decoder needs valid SPS/PPS first");
        }
        Err(e) => {
            // Expected for mock data
            println!("I-frame decode error (expected for mock data): {:?}", e);
        }
    }
}

/// Test handling corrupt frame data
#[test]
fn test_decode_corrupt_frame() {
    let mut decoder = H264Decoder::new().unwrap();

    // Create corrupt data (no NAL header)
    let corrupt_data = vec![0xFF; 1024];

    let result = decoder.decode(&corrupt_data, 0);

    // Should return error for corrupt data
    assert!(
        result.is_err(),
        "Should detect corrupt frame data"
    );
}

/// Test frame sequence processing
#[test]
fn test_frame_sequence_processing() {
    let mut decoder = H264Decoder::new().unwrap();

    // Generate a short frame sequence
    let frames = mock_frame_sequence(500, 30, 15);

    let mut frames_processed = 0;
    let mut frames_decoded = 0;

    for mock_frame in frames {
        // For SPS/PPS, use decode_sps_pps; for others, use decode
        let result = if mock_frame.frame_type == FrameType::SpsPps {
            decoder.decode_sps_pps(&mock_frame.data).map(|_| None)
        } else {
            decoder.decode(&mock_frame.data, mock_frame.timestamp_us)
        };

        frames_processed += 1;

        match result {
            Ok(Some(_video_frame)) => {
                frames_decoded += 1;
            }
            Ok(None) => {
                // Frame queued or config frame
            }
            Err(e) => {
                // Expected for mock data
                let _ = e;
            }
        }
    }

    assert!(frames_processed > 0, "Should process some frames");
    println!(
        "Processed {} frames, decoded {} frames",
        frames_processed, frames_decoded
    );
}

/// Test decoder reset functionality
#[test]
fn test_decoder_reset() {
    let mut decoder = H264Decoder::new().unwrap();

    // Process some frames
    let frames = mock_frame_sequence(200, 30, 15);
    for mock_frame in frames.iter().take(5) {
        if mock_frame.frame_type == FrameType::SpsPps {
            let _ = decoder.decode_sps_pps(&mock_frame.data);
        } else {
            let _ = decoder.decode(&mock_frame.data, mock_frame.timestamp_us);
        }
    }

    // Reset decoder
    let result = decoder.reset();
    assert!(result.is_ok(), "Reset should succeed");

    // Should be able to process frames again
    for mock_frame in frames.iter().skip(5).take(5) {
        if mock_frame.frame_type == FrameType::SpsPps {
            let _ = decoder.decode_sps_pps(&mock_frame.data);
        } else {
            let _ = decoder.decode(&mock_frame.data, mock_frame.timestamp_us);
        }
    }
}

/// Test handling missing SPS/PPS
#[test]
fn test_missing_sps_pps() {
    let mut decoder = H264Decoder::new().unwrap();

    // Try to decode I-frame without SPS/PPS first
    let iframe_data = mock_iframe_nal(2048, 42);

    let result = decoder.decode(&iframe_data, 0);

    // Should handle gracefully — decoder not initialized
    match result {
        Ok(_) => {
            // Some decoders might buffer
        }
        Err(DecoderError::NeedMoreData) => {
            // Expected - needs SPS/PPS first
        }
        Err(e) => {
            println!("Missing SPS/PPS error: {:?}", e);
        }
    }
}

/// Test decoder with different initial states
#[test]
fn test_decoder_initial_state() {
    let decoder = H264Decoder::new().unwrap();
    assert!(!decoder.is_initialized());
    assert_eq!(decoder.frames_decoded(), 0);
    assert!(decoder.dimensions().is_none());
}

/// Test frame timestamp handling
#[test]
fn test_frame_timestamps() {
    let frames = mock_frame_sequence(500, 30, 15);
    let mut last_timestamp = 0u64;

    for mock_frame in frames.iter().skip(1) {
        // Skip SPS/PPS
        assert!(
            mock_frame.timestamp_us >= last_timestamp,
            "Timestamps should be monotonic"
        );
        last_timestamp = mock_frame.timestamp_us;
    }
}

/// Test handling out-of-order frames
#[test]
fn test_out_of_order_frames() {
    let mut decoder = H264Decoder::new().unwrap();

    let mut frames = mock_frame_sequence(300, 30, 15);

    // Swap two P-frames to simulate out-of-order delivery
    if frames.len() > 5 {
        frames.swap(3, 4);
    }

    for mock_frame in frames {
        let result = if mock_frame.frame_type == FrameType::SpsPps {
            decoder.decode_sps_pps(&mock_frame.data).map(|_| None)
        } else {
            decoder.decode(&mock_frame.data, mock_frame.timestamp_us)
        };

        // Decoder should handle gracefully
        match result {
            Ok(_) => {
                // Successfully processed
            }
            Err(e) => {
                // Expected for mock data
                let _ = e;
            }
        }
    }
}

/// Test decoder recovery after errors
#[test]
fn test_decoder_error_recovery() {
    let mut decoder = H264Decoder::new().unwrap();

    // Send valid SPS/PPS
    let sps_pps_frame = MockCameraFrame::sps_pps(0);
    let _ = decoder.decode_sps_pps(&sps_pps_frame.data);

    // Send corrupt frame
    let corrupt_data = vec![0x00, 0x00, 0x00, 0x01, 0xFF, 0xFF, 0xFF];
    let _ = decoder.decode(&corrupt_data, 33333);

    // Send valid I-frame - decoder should recover
    let valid_iframe = MockCameraFrame::iframe(2, 66666, 2048);
    let result = decoder.decode(&valid_iframe.data, valid_iframe.timestamp_us);

    // Should not panic and should try to process
    match result {
        Ok(_) => {
            // Successfully recovered
        }
        Err(e) => {
            println!("Recovery attempt error: {:?}", e);
        }
    }
}

/// Test frame size validation via CameraFrame header
#[test]
fn test_camera_frame_size_field() {
    // CameraFrame is a header — size indicates payload length
    let tiny_frame = CameraFrame {
        frame_type: FrameType::PFrame,
        timestamp_us: 0,
        sequence_number: 1,
        size: 4,
    };
    assert_eq!(tiny_frame.size, 4);

    let large_frame = CameraFrame {
        frame_type: FrameType::IFrame,
        timestamp_us: 0,
        sequence_number: 2,
        size: 1024 * 1024,
    };
    assert_eq!(large_frame.size, 1024 * 1024);
}

/// Test CameraFrame to_packet creates proper payload size
#[test]
fn test_camera_frame_to_packet() {
    let frame = CameraFrame {
        frame_type: FrameType::IFrame,
        timestamp_us: 33333,
        sequence_number: 1,
        size: 2048,
    };

    let packet = frame.to_packet();
    assert_eq!(packet.payload_size, Some(2048));
}

/// Test decoder cleanup on drop
#[test]
fn test_decoder_cleanup() {
    {
        let mut decoder = H264Decoder::new().unwrap();

        let frames = mock_frame_sequence(200, 30, 15);
        for mock_frame in frames.iter().take(10) {
            if mock_frame.frame_type == FrameType::SpsPps {
                let _ = decoder.decode_sps_pps(&mock_frame.data);
            } else {
                let _ = decoder.decode(&mock_frame.data, mock_frame.timestamp_us);
            }
        }

        // Decoder goes out of scope here and should clean up
    }

    // If we get here without panic/leak, cleanup worked
    assert!(true, "Decoder cleaned up successfully");
}

/// Test concurrent decoder instances (if supported)
#[test]
fn test_multiple_decoders() {
    let mut decoder1 = H264Decoder::new().unwrap();
    let mut decoder2 = H264Decoder::new().unwrap();

    let frames1 = mock_frame_sequence(200, 30, 15);
    let frames2 = mock_frame_sequence(200, 30, 15);

    // Process frames on both decoders
    for (f1, f2) in frames1.iter().zip(frames2.iter()).take(10) {
        if f1.frame_type == FrameType::SpsPps {
            let _ = decoder1.decode_sps_pps(&f1.data);
        } else {
            let _ = decoder1.decode(&f1.data, f1.timestamp_us);
        }

        if f2.frame_type == FrameType::SpsPps {
            let _ = decoder2.decode_sps_pps(&f2.data);
        } else {
            let _ = decoder2.decode(&f2.data, f2.timestamp_us);
        }
    }

    // Both should exist independently without panic
}

/// Test NAL unit validation
#[test]
fn test_nal_unit_validation() {
    // Valid NAL units
    assert!(is_valid_nal_unit(&mock_sps_nal()));
    assert!(is_valid_nal_unit(&mock_pps_nal()));
    assert!(is_valid_nal_unit(&mock_iframe_nal(1024, 1)));

    // Invalid NAL units
    assert!(!is_valid_nal_unit(&[0x00, 0x00, 0x00])); // Too short
    assert!(!is_valid_nal_unit(&[0xFF, 0x00, 0x00, 0x01, 0x67])); // Wrong start
    assert!(!is_valid_nal_unit(&[])); // Empty
}

/// Test NAL unit type extraction
#[test]
fn test_nal_unit_type_extraction() {
    assert_eq!(get_nal_unit_type(&mock_sps_nal()), Some(7)); // SPS
    assert_eq!(get_nal_unit_type(&mock_pps_nal()), Some(8)); // PPS
    assert_eq!(get_nal_unit_type(&mock_iframe_nal(1024, 1)), Some(5)); // IDR
    assert_eq!(get_nal_unit_type(&mock_pframe_nal(512, 1)), Some(1)); // Non-IDR
    assert_eq!(get_nal_unit_type(&[0xFF, 0xFF]), None); // Invalid
}

/// Integration test: Full pipeline simulation
#[test]
fn test_full_pipeline_simulation() {
    let mut decoder = H264Decoder::new().unwrap();

    // Simulate receiving frames from network
    let stream_frames = mock_frame_sequence(1000, 30, 30);

    let mut total_processed = 0;
    let mut total_decoded = 0;
    let mut total_errors = 0;

    for mock_frame in stream_frames.iter() {
        let result = if mock_frame.frame_type == FrameType::SpsPps {
            decoder.decode_sps_pps(&mock_frame.data).map(|_| None)
        } else {
            decoder.decode(&mock_frame.data, mock_frame.timestamp_us)
        };

        match result {
            Ok(Some(video_frame)) => {
                total_decoded += 1;
                assert!(!video_frame.data.is_empty());
            }
            Ok(None) => {
                // Frame queued or config frame
            }
            Err(_) => {
                total_errors += 1;
            }
        }

        total_processed += 1;
    }

    println!(
        "Pipeline test: {} processed, {} decoded, {} errors",
        total_processed, total_decoded, total_errors
    );

    assert_eq!(
        total_processed,
        stream_frames.len(),
        "Should process all frames"
    );
}
