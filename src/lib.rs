mod constants;
mod fedilinker;
pub use constants::{FEDILINK_BASE_URL, FEDILINK_SHORT_CODE_LENGTH};

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
    http::StatusCode,
    response::{IntoResponse, Json as J},
};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate simple_error;
// the workers crate was overwriting error and result so we're doing this I guess
use std::error::Error as E;
use std::result::Result as R;

// used for error handling with simple_error
type BoxResult<T> = R<T, Box<dyn E>>;

#[derive(Deserialize, Serialize)]
struct IncomingURL {
    url: String, // the url they want to link with fedilink
    platform: String, // lemmy, pixelfed, mastodon. Defined in constants.rs
}

#[derive(Deserialize, Serialize)]
struct OutgoingFedilink {
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
async fn handle_incoming_url(payload: J<IncomingURL>) -> impl IntoResponse {
    println!("Recv'd url: {}", payload.url);
    let mut fedilinker = Fedilinker::new();
    let result = fedilinker.create_fedilink(&payload.url, &payload.platform);

    // Success response is the OutgoingFedilink struct filled out and a 200 OK
    // Failure is the OutgoingFedilink with the original_url, an error message in the place of
    // the fedilink, and a 400 BAD_REQUEST response
    let (code, response) = match result {
        Ok(fedilink) => {
            let response = OutgoingFedilink {
                original: payload.url.to_string(),
                fedilink: fedilink.to_string(),
            };
            (StatusCode::OK, response)
        }
        Err(error_msg) => {
            let response = OutgoingFedilink {
                original: payload.url.to_string(),
                fedilink: error_msg.to_string(),
            };
            (StatusCode::BAD_REQUEST, response)
        }
    };

    (code, Json(response)).into_response()
}

#[cfg(test)]
mod tests {
    use super::{Fedilinker, FEDILINK_BASE_URL, FEDILINK_SHORT_CODE_LENGTH};

    /*
       Test that the fedilinker is properly mapping URLs back to their original
    */
    #[test]
    fn test_retri_url_from_fedilink_returns_some() {
        let mut fedilinker = Fedilinker::new();
        let original_url = "http://example.com/beans";
        let platform = "lemmy";
        let fedilink_url = fedilinker
            .create_fedilink_url(original_url, platform)
            .unwrap()
            .to_string();

        // assert that we got an url out of the tretri
        assert!(fedilinker.retri_url_from_fedilink(&fedilink_url).is_some());
    }
    #[test]
    fn test_retri_url_from_fedilink_returns_original() {
        let mut fedilinker = Fedilinker::new();
        let original_url = "http://example.com/beans";
        let platform = "lemmy";
        let fedilink_url = fedilinker
            .create_fedilink_url(original_url, platform)
            .unwrap()
            .to_string();

        // assert that the original_url and what we got out of retri match
        assert_eq!(
            fedilinker.retri_url_from_fedilink(&fedilink_url).unwrap(),
            original_url
        );
    }

    #[test]
    fn test_fedilinker_returns_unique_fedilinks() {
        let mut fedilinker = Fedilinker::new();
        let original_url = "http://example.com/beans";
        let platform = "lemmy";
        let fedilink_url_one = fedilinker
            .create_fedilink_url(original_url, platform)
            .unwrap()
            .to_string();
        let fedilinker_url_two = fedilinker
            .create_fedilink_url(original_url, platform)
            .unwrap()
            .to_string();

        // fedilinks should always be unique, regardless of the source url
        assert_ne!(fedilink_url_one, fedilinker_url_two);
    }
    #[test]
    fn test_fedilinker_expected_length() {
        let mut fedilinker = Fedilinker::new();
        let original_url = "http://example.com/beans";
        let platform = "lemmy";
        let fedilink_url_one = fedilinker
            .create_fedilink_url(original_url, platform)
            .unwrap()
            .to_string();

        // fedilinks should have a defined and constant size, as defined in constants.rs
        // fedilinks are the baseurl/ redir_url/ short_code, but the redir_url doesn't contain the slash that's added by the
        // url crate, so count the length here.
        let expected_len =
            FEDILINK_BASE_URL.len() + platform.len() + "/".len() + FEDILINK_SHORT_CODE_LENGTH;
        assert_eq!(fedilink_url_one.len(), expected_len);
    }

    /*
       Test that:
       - we are getting unique fedilinks
       - fedilinks are the appropriate length
       - fedilinks are a fixed length
    */
    #[test]
    fn test_fedilinker_two_unique_urls_are_same_length() {
        let mut fedilinker = Fedilinker::new();
        let original_url = "http://example.com/beans";
        let platform = "lemmy";
        let fedilink_url_one = fedilinker
            .create_fedilink_url(original_url, platform)
            .unwrap()
            .to_string();
        let fedilinker_url_two = fedilinker
            .create_fedilink_url(original_url, platform)
            .unwrap()
            .to_string();

        assert_eq!(fedilink_url_one.len(), fedilinker_url_two.len());
    }

    #[test]
    fn test_validate_platform_valid_platform() {
        let fedilinker = Fedilinker::new();
        let platform = "lemmy";
        let fedilink_url = fedilinker.validate_platform(platform);

        assert!(fedilink_url.is_ok());
    }
    #[test]
    fn test_validate_platform_invalid_platform_returns_error() {
        let fedilinker = Fedilinker::new();
        let platform = "aninvalidaplatformchoice";
        let fedilink_url = fedilinker.validate_platform(platform);

        assert!(fedilink_url.is_err());
    }

    #[test]
    fn test_create_fedilink_with_invalid_platform_returns_error() {
        let mut fedilinker = Fedilinker::new();
        let original_url = "http://example.com/beans";
        let platform = "aninvalidplatformchoice";
        let fedilink_url_one =
            fedilinker.create_fedilink(&original_url.to_string(), &platform.to_string());

        //TODO: check the error message is what we expect. Store in constants.
        assert!(fedilink_url_one.is_err());
    }

    #[test]
    fn test_create_fedilink_with_valid_platform_returns_ok() {
        let mut fedilinker = Fedilinker::new();
        let original_url = "http://example.com/beans";
        let platform = "lemmy";
        let fedilink_url_one =
            fedilinker.create_fedilink(&original_url.to_string(), &platform.to_string());

        assert!(fedilink_url_one.is_ok());
    }
    fn type_of<T>(_: &T) -> &'static str {
        use std::any::type_name;
        type_name::<T>()
    }
    #[test]
    fn test_create_fedilink_with_valid_platform_returns_string() {
        let mut fedilinker = Fedilinker::new();
        let original_url = "http://example.com/beans";
        let platform = "lemmy";
        let fedilink = fedilinker.create_fedilink(&original_url.to_string(), &platform.to_string());

        assert_eq!(type_of(&fedilink.unwrap()), "alloc::string::String");
    }
}
