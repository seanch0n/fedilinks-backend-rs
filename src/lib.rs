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
                let result = fedilinker.create_fedilink(&incoming.platform);

                // check the result, if it's good then we store in workers kv. Otherwise we throw an error back
                match result {
                    Ok(fedilink) => {
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
    .get_async("/:platform/:short_code", |_req, ctx| async move {
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

#[cfg(test)]
mod tests {
    use super::{Fedilinker, FEDILINK_BASE_URL, FEDILINK_SHORT_CODE_LENGTH};

    #[test]
    fn test_fedilinker_returns_unique_fedilinks() {
        let fedilinker = Fedilinker::new();
        let platform = "lemmy";
        let fedilink_url_one = fedilinker
            .create_fedilink_url(platform)
            .unwrap()
            .to_string();
        let fedilinker_url_two = fedilinker
            .create_fedilink_url(platform)
            .unwrap()
            .to_string();

        // fedilinks should always be unique, regardless of the source url
        assert_ne!(fedilink_url_one, fedilinker_url_two);
    }
    #[test]
    fn test_fedilinker_expected_length() {
        let fedilinker = Fedilinker::new();
        let platform = "lemmy";
        let fedilink_url_one = fedilinker
            .create_fedilink_url(platform)
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
        let fedilinker = Fedilinker::new();
        let platform = "lemmy";
        let fedilink_url_one = fedilinker
            .create_fedilink_url(platform)
            .unwrap()
            .to_string();
        let fedilinker_url_two = fedilinker
            .create_fedilink_url(platform)
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
        let fedilinker = Fedilinker::new();
        let platform = "aninvalidplatformchoice";
        let fedilink_url_one =
            fedilinker.create_fedilink(&platform.to_string());

        //TODO: check the error message is what we expect. Store in constants.
        assert!(fedilink_url_one.is_err());
    }

    #[test]
    fn test_create_fedilink_with_valid_platform_returns_ok() {
        let fedilinker = Fedilinker::new();
        let platform = "lemmy";
        let fedilink_url_one =
            fedilinker.create_fedilink( &platform.to_string());

        assert!(fedilink_url_one.is_ok());
    }
    fn type_of<T>(_: &T) -> &'static str {
        use std::any::type_name;
        type_name::<T>()
    }
    #[test]
    fn test_create_fedilink_with_valid_platform_returns_string() {
        let fedilinker = Fedilinker::new();
        let platform = "lemmy";
        let fedilink = fedilinker.create_fedilink(&platform.to_string());

        assert_eq!(type_of(&fedilink.unwrap()), "alloc::string::String");
    }
}
