pub const FEDILINK_SHORT_CODE_LENGTH: usize = 8;
// used in fedilink_short_code generation
pub const ALPHANUMERIC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
pub const FEDILINK_BASE_URL: &str = "http://fedilinks.net/";
pub const VALID_FEDILINK_PLATFORMS: [&str; 3] = ["lemmy", "pixelfed", "mastodon"];
pub const KV_STORE_NAME: &str = "cities";
pub const KV_STORE_EXPIR_SECONDS: u64 = 864000;
