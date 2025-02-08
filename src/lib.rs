mod constants;
mod fedilinker;
// pub use constants::{FEDILINK_BASE_URL, FEDILINK_SHORT_CODE_LENGTH};
//
pub use fedilinker::*;
// //
// // // worker added crates
// // use tower_service::Service;
// // use worker::*;
// //
// // // user added crates
// // use axum::*;
// // use axum::{
// //     extract::{Path, Extension, State},
// //     routing::{get, post},
// //     Router,
// //     Json,
// //     http::StatusCode,
// //     response::{IntoResponse, Json as J},
// // };
// // use serde::{Deserialize, Serialize};
// //
#[macro_use]
extern crate simple_error;
// // the workers crate was overwriting error and result so we're doing this I guess
use std::error::Error as E;
use std::result::Result as R;
// // use std::sync::{Arc, Mutex};
// // use axum::response::Redirect;
// // use worker::kv::KvStore;
// //
// used for error handling with simple_error
type BoxResult<T> = R<T, Box<dyn E>>;
// //
// // #[derive(Deserialize, Serialize)]
// // struct IncomingURL {
// //     url: String, // the url they want to link with fedilink
// //     platform: String, // lemmy, pixelfed, mastodon. Defined in constants.rs
// // }
// //
// // #[derive(Deserialize, Serialize)]
// // struct OutgoingFedilink {
// //     original: String,
// //     fedilink: String,
// // }
// //
// // struct AppState {
// //     env: Env,
// // }
// //
// // fn router(_env: Env) -> Router {
// //
// //     let shared_state = Arc::new(AppState { env: _env });
// //
// //     // Specify the state type explicitly when creating the router
// //     let app = Router::new()
// //         .route("/make-fedilink", post(handle_incoming_url))
// //         .with_state(shared_state);
// //     app
// //
// //     // let kv = KvStore::create("LINKS");
// //     // Router::new()
// //     //     .route("/", get(root))
// //     //     .route("/not-found", get(not_found_handler))
// //     //     .route("/make-fedilink", post(handle_incoming_url))
// //     //     .layer(Extension(Arc::new(Fedilinker::new())))
// //         // .route("/{short_code}", get(redirect_handler))
// //         // .route("/{platform}/{short_code}", get(redirect_handler))
// // }
// // async fn handle_incoming_url(
// //     Json(payload): Json<IncomingURL>,  // Change J to Json
// //     State(state): State<Arc<AppState>>, // Change axum::extract::State to State
// // ) -> impl IntoResponse {
// //     let fedilinker = Fedilinker::new();
// //     state.env.kv("LINKSTORAGE")
// //         .unwrap()
// //         .put("platform", "lemmy")
// //         .unwrap()
// //         .execute()
// //         .await;
// //
// //     Json(serde_json::json!({ "short_url": "asdf" }))  // Change J to Json
// // }
// //
// // #[event(fetch)]
// // async fn fetch(
// //     req: HttpRequest,
// //     _env: Env,
// //     _ctx: Context,
// // ) -> Result<axum::http::Response<axum::body::Body>> {
// //     console_error_panic_hook::set_once();
// //     Ok(router(_env).call(req).await?)
// // }
// //
// // pub async fn root() -> &'static str {
// //     "Hello Axum!"
// // }
// //
// // pub async fn not_found_handler() -> &'static str {
// //     "That was not found :("
// // }
// //
// // // testing
// // pub async fn get_id(Path(id): Path<i32>) -> String {
// //     let msg = format!("Hello {}!", id);
// //     msg
// // }
// //
// // // async fn redirect_handler(
// // //     Path((platform, short_code)): Path<(String, String)>,
// // //     Extension(state): Extension<Arc<Fedilinker>>,
// // // ) -> impl IntoResponse {
// // //
// // //     console_log!("platform: {}, short_url: {}",platform , short_code);
// // //     if let Some(original_url) = state.retri_url_from_fedilink(&format!("{}/{}", platform, short_code)) {
// // //         Redirect::temporary(original_url)
// // //     } else {
// // //         Redirect::temporary("/not-found")
// // //     }
// // // }
// // //
// // // /*
// // //    Get an incoming URL that needs to be converted to a fedilink and respond
// // //    with the newly minted fedilink
// // // */
// // // async fn handle_incoming_url(payload: J<IncomingURL>,
// // //                              Extension(fedilinker): Extension<Arc<Fedilinker>>,
// // //                              Extension(kvstore): Extension<KvStore>,
// // // ) -> impl IntoResponse {
// // //     println!("Recv'd url: {}", payload.url);
// // //     let result = fedilinker.create_fedilink(&payload.url, &payload.platform);
// // //
// // //     // Success response is the OutgoingFedilink struct filled out and a 200 OK
// // //     // Failure is the OutgoingFedilink with the original_url, an error message in the place of
// // //     // the fedilink, and a 400 BAD_REQUEST response
// // //     let (code, response) = match result {
// // //         Ok(fedilink) => {
// // //             let response = OutgoingFedilink {
// // //                 original: payload.url.to_string(),
// // //                 fedilink: fedilink.to_string(),
// // //             };
// // //             (StatusCode::OK, response)
// // //         }
// // //         Err(error_msg) => {
// // //             let response = OutgoingFedilink {
// // //                 original: payload.url.to_string(),
// // //                 fedilink: error_msg.to_string(),
// // //             };
// // //             (StatusCode::BAD_REQUEST, response)
// // //         }
// // //     };
// // //
// // //     (code, Json(response)).into_response()
// // // }
// //
// // async fn beans(payload: J<IncomingURL>,
// //                              Extension(fedilinker): Extension<Arc<Fedilinker>>,
// // ) -> impl IntoResponse {
// //
// //
// //         match fedilinker.validate_platform("lemmy") {
// //             Ok(()) => {
// //                 println!("Successfully validated the fedilinker!");
// //             }
// //             Err(_) => {
// //                 println!("Failed to validate the fedilinker!");
// //                 // bail!("invalid platform")
// //             }
// //         }
// //
// //
// //        // fedilinker.validate_platform("beans");
// //     let asdf = IncomingURL {
// //         url: "asdf".to_string(),
// //         platform: "asdf".to_string(),
// //     };
// //     J(asdf)
// //
// // }
// //
// // #[cfg(test)]
// // mod tests {
// //     use super::{Fedilinker, FEDILINK_BASE_URL, FEDILINK_SHORT_CODE_LENGTH};
// //
// //     /*
// //        Test that the fedilinker is properly mapping URLs back to their original
// //     */
// //     #[test]
// //     fn test_retri_url_from_fedilink_returns_some() {
// //         let mut fedilinker = Fedilinker::new();
// //         let original_url = "http://example.com/beans";
// //         let platform = "lemmy";
// //         let fedilink_url = fedilinker
// //             .create_fedilink_url(original_url, platform)
// //             .unwrap()
// //             .to_string();
// //
// //         // assert that we got an url out of the tretri
// //         assert!(fedilinker.retri_url_from_fedilink(&fedilink_url).is_some());
// //     }
// //     #[test]
// //     fn test_retri_url_from_fedilink_returns_original() {
// //         let mut fedilinker = Fedilinker::new();
// //         let original_url = "http://example.com/beans";
// //         let platform = "lemmy";
// //         let fedilink_url = fedilinker
// //             .create_fedilink_url(original_url, platform)
// //             .unwrap()
// //             .to_string();
// //
// //         // assert that the original_url and what we got out of retri match
// //         assert_eq!(
// //             fedilinker.retri_url_from_fedilink(&fedilink_url).unwrap(),
// //             original_url
// //         );
// //     }
// //
// //     #[test]
// //     fn test_fedilinker_returns_unique_fedilinks() {
// //         let mut fedilinker = Fedilinker::new();
// //         let original_url = "http://example.com/beans";
// //         let platform = "lemmy";
// //         let fedilink_url_one = fedilinker
// //             .create_fedilink_url(original_url, platform)
// //             .unwrap()
// //             .to_string();
// //         let fedilinker_url_two = fedilinker
// //             .create_fedilink_url(original_url, platform)
// //             .unwrap()
// //             .to_string();
// //
// //         // fedilinks should always be unique, regardless of the source url
// //         assert_ne!(fedilink_url_one, fedilinker_url_two);
// //     }
// //     #[test]
// //     fn test_fedilinker_expected_length() {
// //         let mut fedilinker = Fedilinker::new();
// //         let original_url = "http://example.com/beans";
// //         let platform = "lemmy";
// //         let fedilink_url_one = fedilinker
// //             .create_fedilink_url(original_url, platform)
// //             .unwrap()
// //             .to_string();
// //
// //         // fedilinks should have a defined and constant size, as defined in constants.rs
// //         // fedilinks are the baseurl/ redir_url/ short_code, but the redir_url doesn't contain the slash that's added by the
// //         // url crate, so count the length here.
// //         let expected_len =
// //             FEDILINK_BASE_URL.len() + platform.len() + "/".len() + FEDILINK_SHORT_CODE_LENGTH;
// //         assert_eq!(fedilink_url_one.len(), expected_len);
// //     }
// //
// //     /*
// //        Test that:
// //        - we are getting unique fedilinks
// //        - fedilinks are the appropriate length
// //        - fedilinks are a fixed length
// //     */
// //     #[test]
// //     fn test_fedilinker_two_unique_urls_are_same_length() {
// //         let mut fedilinker = Fedilinker::new();
// //         let original_url = "http://example.com/beans";
// //         let platform = "lemmy";
// //         let fedilink_url_one = fedilinker
// //             .create_fedilink_url(original_url, platform)
// //             .unwrap()
// //             .to_string();
// //         let fedilinker_url_two = fedilinker
// //             .create_fedilink_url(original_url, platform)
// //             .unwrap()
// //             .to_string();
// //
// //         assert_eq!(fedilink_url_one.len(), fedilinker_url_two.len());
// //     }
// //
// //     #[test]
// //     fn test_validate_platform_valid_platform() {
// //         let fedilinker = Fedilinker::new();
// //         let platform = "lemmy";
// //         let fedilink_url = fedilinker.validate_platform(platform);
// //
// //         assert!(fedilink_url.is_ok());
// //     }
// //     #[test]
// //     fn test_validate_platform_invalid_platform_returns_error() {
// //         let fedilinker = Fedilinker::new();
// //         let platform = "aninvalidaplatformchoice";
// //         let fedilink_url = fedilinker.validate_platform(platform);
// //
// //         assert!(fedilink_url.is_err());
// //     }
// //
// //     #[test]
// //     fn test_create_fedilink_with_invalid_platform_returns_error() {
// //         let mut fedilinker = Fedilinker::new();
// //         let original_url = "http://example.com/beans";
// //         let platform = "aninvalidplatformchoice";
// //         let fedilink_url_one =
// //             fedilinker.create_fedilink(&original_url.to_string(), &platform.to_string());
// //
// //         //TODO: check the error message is what we expect. Store in constants.
// //         assert!(fedilink_url_one.is_err());
// //     }
// //
// //     #[test]
// //     fn test_create_fedilink_with_valid_platform_returns_ok() {
// //         let mut fedilinker = Fedilinker::new();
// //         let original_url = "http://example.com/beans";
// //         let platform = "lemmy";
// //         let fedilink_url_one =
// //             fedilinker.create_fedilink(&original_url.to_string(), &platform.to_string());
// //
// //         assert!(fedilink_url_one.is_ok());
// //     }
// //     fn type_of<T>(_: &T) -> &'static str {
// //         use std::any::type_name;
// //         type_name::<T>()
// //     }
// //     #[test]
// //     fn test_create_fedilink_with_valid_platform_returns_string() {
// //         let mut fedilinker = Fedilinker::new();
// //         let original_url = "http://example.com/beans";
// //         let platform = "lemmy";
// //         let fedilink = fedilinker.create_fedilink(&original_url.to_string(), &platform.to_string());
// //
// //         assert_eq!(type_of(&fedilink.unwrap()), "alloc::string::String");
// //     }
// // }
//

use serde::{Deserialize, Serialize};
use worker::*;
use std::sync::Arc;

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

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("in fetch");
    let router = Router::new();
    // We need to wrap this in arc so we can use it in the thread of router. This is used
    // to call into the Fedilinker impl and build our links.
    let fedilinker = Arc::new(Fedilinker::new());

    // They make a post request to /make-fedilink with some JSON data (IncomingUrl struct ref)
    router.post_async("/make-fedilink", {
        // we have to clone this within the context of the router, but before we perform
        // the move or rust fliiiiiiips out
        let fedilinker = Arc::clone(&fedilinker);
        move |mut req, ctx| {
            // and now that we have moved, we have to clone from the moved clone. This is gross.
            let fedilinker = Arc::clone(&fedilinker);
            async move {
                // parse the incoming request's json to get the url they want to make a fedilink for
                let incoming = match req.json::<IncomingURL>().await {
                    Ok(data) => data,
                    Err(_) => return Response::error("Failed to parse JSON", 400),
                };

                // actually create the fedilink. This however does not store it in the storage layer,
                // that is on us to orchestrate, because I'm not going to try and pass more thread
                // stuff around
                let result = fedilinker.create_fedilink(&incoming.url, &incoming.platform);

                console_log!("hi there");
                // check the result, if it's good then we store in workers kv. Otherwise we throw an error back
                match result {
                    Ok(fedilink) => {
                        console_log!("fedilink created, trying to put in kv");
                        //TODO: pull the kv name from env or constants or something smarter
                        // let storage = env.kv("cities")?;
                        // store url: fedilink into the kv store
                        match ctx.kv("cities")?.put( &fedilink.to_string(), &incoming.url)?.execute().await {
                            Ok(_) => console_log!("Saved it! {} == {}", &fedilink.to_string(), &incoming.url),
                            Err(_) => console_log!("Failed to save fedilink"),
                        }

                        let original_url = ctx.kv("cities")?.get(&fedilink.to_string()).text().await?;
                        console_log!("we have original?: {}", &original_url.unwrap());
                        // respond with the link
                        let resp = OutgoingFedilink {
                            original: incoming.url,
                            fedilink: fedilink.to_string(),
                        };
                        Ok(Response::from_json(&resp)?.with_status(200))
                    }
                    Err(error_msg) => {
                        console_log!("failed to create fedilink, ruhroh");
                        console_log!("failed storage_ret match: {}", error_msg);
                        let resp = OutgoingFedilink {
                            original: incoming.url,
                            fedilink: error_msg.to_string(),
                        };
                        Ok(Response::from_json(&resp)?.with_status(400))
                    }
                }
            }
        }
    })
    .get_async("/:platform/:short_code", |req, ctx| async move {
        let platform = ctx.param("platform");
        let short_code = ctx.param("short_code");
        console_log!("platform: [{}], short_code: [{}]", platform.unwrap(), short_code.unwrap());
        let original_url = ctx.kv("cities")?.get(&format!("http://fedilinks.net/{}/{}", platform.unwrap(), short_code.unwrap())).text().await?;
        match original_url {
            Some(url) => {
                console_log!("orig was got as: {}", url);
                Response::redirect(url.parse().unwrap())
            },
             None => {
                Response::error("errrrrr", 404)
            }
        }
    })
    .run(req, env)
    .await
}

