use std::sync::Arc;
use md5::{Digest, Md5};
use schema::unique_bytes::UniqueBytes;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Account {
    pub uuid: Uuid,
    pub username: Arc<str>,
    pub offline: bool,
    pub head: Option<UniqueBytes>,
}

/// Generates a deterministic Version 3 (MD5-based) UUID for an offline Minecraft player.
/// This matches standard Minecraft Java Edition behavior:
/// `UUID.nameUUIDFromBytes(("OfflinePlayer:" + username).getBytes(StandardCharsets.UTF_8))`
pub fn offline_player_uuid(username: &str) -> Uuid {
    let mut hasher = Md5::new();
    hasher.update(b"OfflinePlayer:");
    hasher.update(username.as_bytes());
    let mut bytes: [u8; 16] = hasher.finalize().into();

    // Set version to 3 (0011xxxx in byte 6)
    bytes[6] = (bytes[6] & 0x0f) | 0x30;
    // Set variant to RFC 4122 / IETF (10xxxxxx in byte 8)
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    Uuid::from_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offline_player_uuid() {
        // Minecraft offline UUID for "Steve" is 5627dd98-e6be-3c21-b8a8-e92344183641
        let uuid = offline_player_uuid("Steve");
        assert_eq!(uuid.to_string(), "5627dd98-e6be-3c21-b8a8-e92344183641");

        // Minecraft offline UUID for "Alex" is 36532b5e-c442-3dbb-a24c-c7e55d0f979a
        let uuid_alex = offline_player_uuid("Alex");
        assert_eq!(uuid_alex.to_string(), "36532b5e-c442-3dbb-a24c-c7e55d0f979a");
    }
}

