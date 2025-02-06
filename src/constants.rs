pub const FEDILINK_SHORT_CODE_LENGTH: usize = 8;
// used in fedilink_short_code generation
pub const ALPHANUMERIC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
pub const FEDILINK_BASE_URL: &str = "http://fedilinks.net/";
// This is what goes after the FEDILINK_BASE_URL and before the short_code, so we get urls like:
// http://fedilinks.net/re/asdfasdf
pub const FEDILINK_REDIR_URL: &str = "re";
