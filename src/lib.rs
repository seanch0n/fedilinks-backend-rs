mod constants;
mod fedilinker;
pub use constants::{FEDILINK_BASE_URL, FEDILINK_REDIR_URL, FEDILINK_SHORT_CODE_LENGTH};

pub use fedilinker::*;

// worker added crates
use tower_service::Service;
use worker::*;

// user added crates
use axum::*;
use axum::{
    extract,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct IncomingURL {
    url: String,
    platform: String, // lemmy, pixelfed, mastodon, etc etc.
}

#[derive(Deserialize, Serialize)]
struct OutgoingFedilnk {
    original: String,
    fedilink: String,
}

fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/{id}", get(get_id))
        .route("/make-fedilink", post(handle_incoming_url))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}

// testing
pub async fn get_id(extract::Path(id): extract::Path<i32>) -> String {
    let msg = format!("Hello {}!", id);
    msg
}

/*
   Get an incoming URL that needs to be converted to a fedilink and respond
   with the newly minted fedilink
*/
async fn handle_incoming_url(payload: Json<IncomingURL>) -> Json<OutgoingFedilnk> {
    println!("Recv'd url: {}", payload.url);
    // convert_call(payload.url)
    let fedilink = create_fedilink(&payload.url, &payload.platform);
    // respond with fedilink
    let response = OutgoingFedilnk {
        original: payload.url.to_string(),
        fedilink: fedilink.to_string(),
    };
    Json(response)
}

/*
   Create a fedilink for a given URL and store in cloudflare workers kv
*/
fn create_fedilink(original_url: &String, platform: &String) -> String {
    let mut fedilinker = Fedilinker::new();

    fedilinker
        .create_fedilink_url(original_url.as_str())
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{Fedilinker, FEDILINK_BASE_URL, FEDILINK_REDIR_URL, FEDILINK_SHORT_CODE_LENGTH};

    /*
       Test that the fedilinker is properly mapping URLs back to their original
    */
    #[test]
    fn test_fedilinker() {
        let mut fedilinker = Fedilinker::new();
        let original_url = "http://example.com/beans";
        let fedilink_url = fedilinker
            .create_fedilink_url(original_url)
            .unwrap()
            .to_string();

        // println!("orig: {}, fedi: {}", original_url, fedilink_url);
        // assert that we got a url out of the tretri
        assert!(fedilinker.retri_url_from_fedilink(&fedilink_url).is_some());
        // assert that the original_url and what we got out of retri match
        assert_eq!(
            fedilinker.retri_url_from_fedilink(&fedilink_url).unwrap(),
            original_url
        );
    }

    /*
       Test that:
       - we are getting unique fedilinks
       - fedilinks are the appropriate length
       - fedilinks are a fixed length
    */
    #[test]
    fn test_fedilinker_quality() {
        let mut fedilinker = Fedilinker::new();
        let original_url = "http://example.com/beans";
        let fedilink_url_one = fedilinker
            .create_fedilink_url(original_url)
            .unwrap()
            .to_string();
        let fedilinker_url_two = fedilinker
            .create_fedilink_url(original_url)
            .unwrap()
            .to_string();

        println!(
            "orig: {} one {} two {}",
            original_url, fedilink_url_one, fedilinker_url_two
        );
        // fedilinks should always be unique, regardless of the source url
        assert_ne!(fedilink_url_one, fedilinker_url_two);
        // fedilinks should have a defined and constant size, as defined in constants.rs
        // fedilinks are the baseurl/ redir_url/ short_code, but the redir_url doesn't contain the slash that's added by the
        // url crate, so count the length here.
        let expected_len = FEDILINK_BASE_URL.len()
            + FEDILINK_REDIR_URL.len()
            + "/".len()
            + FEDILINK_SHORT_CODE_LENGTH;
        assert_eq!(fedilink_url_one.len(), expected_len);
        // fedilinks should always be the same length
        assert_eq!(fedilink_url_one.len(), fedilinker_url_two.len());
    }
}
