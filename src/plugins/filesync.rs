//! FileSync plugin
//!
//! Synchronizes files between devices with bidirectional sync support.
//! Tracks file changes, detects conflicts, and manages sync folders.

use crate::protocol::Packet;
use crate::error::Result;
use serde_json::json;

/// FileSync notification packet type
pub const PACKET_TYPE_FILESYNC: &str = "cconnect.filesync";
/// FileSync request packet type
pub const PACKET_TYPE_FILESYNC_REQUEST: &str = "cconnect.filesync.request";
/// FileSync conflict packet type
pub const PACKET_TYPE_FILESYNC_CONFLICT: &str = "cconnect.filesync.conflict";

/// Create a file sync notification packet
///
/// # Arguments
///
/// * `action` - Action type: "file_changed", "file_deleted", "file_added", "sync_complete", "sync_started"
/// * `path` - Relative file path within sync folder
/// * `checksum` - Optional file checksum (SHA-256)
/// * `size` - Optional file size in bytes
/// * `timestamp` - Optional last modified timestamp (epoch millis)
/// * `sync_folder_id` - Identifier for the sync folder pair
pub fn create_filesync_notification(
    action: &str,
    path: &str,
    checksum: Option<String>,
    size: Option<i64>,
    timestamp: Option<i64>,
    sync_folder_id: &str,
) -> Result<Packet> {
    let mut body = json!({
        "action": action,
        "path": path,
        "syncFolderId": sync_folder_id,
    });

    if let Some(cs) = checksum {
        body["checksum"] = json!(cs);
    }
    if let Some(s) = size {
        body["size"] = json!(s);
    }
    if let Some(ts) = timestamp {
        body["timestamp"] = json!(ts);
    }

    Ok(Packet::new(PACKET_TYPE_FILESYNC, body))
}

/// Create a request to sync a specific folder
pub fn create_filesync_request_sync(sync_folder_id: &str) -> Result<Packet> {
    Ok(Packet::new(
        PACKET_TYPE_FILESYNC_REQUEST,
        json!({
            "requestSync": true,
            "syncFolderId": sync_folder_id,
        }),
    ))
}

/// Create a request to add a new sync folder
pub fn create_filesync_add_folder(path: &str) -> Result<Packet> {
    Ok(Packet::new(
        PACKET_TYPE_FILESYNC_REQUEST,
        json!({
            "addSyncFolder": path,
        }),
    ))
}

/// Create a request to remove a sync folder
pub fn create_filesync_remove_folder(sync_folder_id: &str) -> Result<Packet> {
    Ok(Packet::new(
        PACKET_TYPE_FILESYNC_REQUEST,
        json!({
            "removeSyncFolder": sync_folder_id,
        }),
    ))
}

/// Create a request to list all sync folders
pub fn create_filesync_list_folders() -> Result<Packet> {
    Ok(Packet::new(
        PACKET_TYPE_FILESYNC_REQUEST,
        json!({
            "listSyncFolders": true,
        }),
    ))
}

/// Create a conflict notification packet
///
/// # Arguments
///
/// * `path` - Conflicting file path
/// * `local_checksum` - Local file checksum
/// * `remote_checksum` - Remote file checksum
/// * `local_timestamp` - Local modification time (epoch millis)
/// * `remote_timestamp` - Remote modification time (epoch millis)
/// * `sync_folder_id` - Which sync folder
pub fn create_filesync_conflict(
    path: &str,
    local_checksum: &str,
    remote_checksum: &str,
    local_timestamp: i64,
    remote_timestamp: i64,
    sync_folder_id: &str,
) -> Result<Packet> {
    Ok(Packet::new(
        PACKET_TYPE_FILESYNC_CONFLICT,
        json!({
            "path": path,
            "localChecksum": local_checksum,
            "remoteChecksum": remote_checksum,
            "localTimestamp": local_timestamp,
            "remoteTimestamp": remote_timestamp,
            "syncFolderId": sync_folder_id,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_filesync_notification_minimal() {
        let packet = create_filesync_notification(
            "file_added",
            "documents/test.txt",
            None,
            None,
            None,
            "folder-123",
        )
        .unwrap();
        assert_eq!(packet.packet_type, "cconnect.filesync");
        assert_eq!(packet.body["action"], "file_added");
        assert_eq!(packet.body["path"], "documents/test.txt");
        assert_eq!(packet.body["syncFolderId"], "folder-123");
        assert!(packet.body.get("checksum").is_none());
    }

    #[test]
    fn test_create_filesync_notification_full() {
        let packet = create_filesync_notification(
            "file_changed",
            "photos/vacation.jpg",
            Some("abc123def456".to_string()),
            Some(2048576),
            Some(1675000000000),
            "folder-456",
        )
        .unwrap();
        assert_eq!(packet.body["checksum"], "abc123def456");
        assert_eq!(packet.body["size"], 2048576);
        assert_eq!(packet.body["timestamp"], 1675000000000_i64);
    }

    #[test]
    fn test_create_filesync_request_sync() {
        let packet = create_filesync_request_sync("folder-789").unwrap();
        assert_eq!(packet.packet_type, "cconnect.filesync.request");
        assert_eq!(packet.body["requestSync"], true);
        assert_eq!(packet.body["syncFolderId"], "folder-789");
    }

    #[test]
    fn test_create_filesync_add_folder() {
        let packet = create_filesync_add_folder("/home/user/Documents").unwrap();
        assert_eq!(packet.packet_type, "cconnect.filesync.request");
        assert_eq!(packet.body["addSyncFolder"], "/home/user/Documents");
    }

    #[test]
    fn test_create_filesync_remove_folder() {
        let packet = create_filesync_remove_folder("folder-abc").unwrap();
        assert_eq!(packet.packet_type, "cconnect.filesync.request");
        assert_eq!(packet.body["removeSyncFolder"], "folder-abc");
    }

    #[test]
    fn test_create_filesync_list_folders() {
        let packet = create_filesync_list_folders().unwrap();
        assert_eq!(packet.packet_type, "cconnect.filesync.request");
        assert_eq!(packet.body["listSyncFolders"], true);
    }

    #[test]
    fn test_create_filesync_conflict() {
        let packet = create_filesync_conflict(
            "docs/report.docx",
            "local-hash-123",
            "remote-hash-456",
            1675000000000,
            1675001000000,
            "folder-conflict",
        )
        .unwrap();
        assert_eq!(packet.packet_type, "cconnect.filesync.conflict");
        assert_eq!(packet.body["path"], "docs/report.docx");
        assert_eq!(packet.body["localChecksum"], "local-hash-123");
        assert_eq!(packet.body["remoteChecksum"], "remote-hash-456");
        assert_eq!(packet.body["localTimestamp"], 1675000000000_i64);
        assert_eq!(packet.body["remoteTimestamp"], 1675001000000_i64);
        assert_eq!(packet.body["syncFolderId"], "folder-conflict");
    }
}
