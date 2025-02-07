mod constants;
mod fedilinker;
// pub use constants::{FEDILINK_BASE_URL, FEDILINK_SHORT_CODE_LENGTH};
pub use fedilinker::*;

#[macro_use]
extern crate simple_error;
// the workers crate was overwriting error and result so we're doing this I guess
use std::error::Error as E;
use std::result::Result as R;

// used for error handling with simple_error
type BoxResult<T> = R<T, Box<dyn E>>;
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
    let router = Router::new();
    // We need to wrap this in arc so we can use it in the thread of router. This is used
    // to call into the Fedilinker impl and build our links.
    let fedilinker = Arc::new(Fedilinker::new());

    // They make a post request to /make-fedilink with some JSON data (IncomingUrl struct ref)
    router.post_async("/make-fedilink", {
        // we have to clone this within the context of the router, but before we perform
        // the move or rust fliiiiiiips out
        let fedilinker = Arc::clone(&fedilinker);
        move |mut req, env| {
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

                // check the result, if it's good then we store in workers kv. Otherwise we throw an error back
                match result {
                    Ok(fedilink) => {
                        //TODO: pull the kv name from env or constants or something smarter
                        let storage = env.kv("cities")?;
                        // store url: fedilink into the kv store
                        match storage.put(&incoming.url, &fedilink.to_string())?.execute().await {
                            Ok(_) => console_log!("Saved it! {} == {}", &incoming.url, &fedilink.to_string()),
                            Err(_) => console_log!("Failed to save fedilink"),
                        }
                        // respond with the link
                        let resp = OutgoingFedilink {
                            original: incoming.url,
                            fedilink: fedilink.to_string(),
                        };
                        Ok(Response::from_json(&resp)?.with_status(200))
                    }
                    Err(error_msg) => {
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
        .run(req, env)
        .await
}

