pub use crate::constants::{
    ALPHANUMERIC, FEDILINK_BASE_URL, FEDILINK_SHORT_CODE_LENGTH,
    VALID_FEDILINK_PLATFORMS,
};

use getrandom;
use std::collections::{HashMap, HashSet};
use url::{ParseError, Url};
use worker::js_sys::{Array, Boolean};

pub struct Fedilinker {
    url_map: HashMap<String, String>,
    valid_platforms: HashSet<&'static str>,
}
impl Fedilinker {
    pub fn new() -> Self {
        let valid_platforms_set = VALID_FEDILINK_PLATFORMS
            .iter()
            .cloned()
            .collect::<HashSet<_>>();

        Self {
            url_map: HashMap::new(),
            valid_platforms: valid_platforms_set,
        }
    }


    /*
       Generate a fedilink shortcode, which is an 8 character short code.
       We have to use the getrandom crate instead of rand because it supports
       wasm-js which we need for cloudflare workers
    */
    fn generate_fedilink_shortcode(&self, platform: &str) -> String {
        let mut buf = [0; FEDILINK_SHORT_CODE_LENGTH];
        getrandom::fill(&mut buf).expect("Failed to get random bytes");

        // Map random bytes to characters in the ALPHABET/upper/lower/numbers
        let short_code: String = buf
            .iter()
            .map(|&b| ALPHANUMERIC[(b as usize) % ALPHANUMERIC.len()] as char)
            .collect();

        println!("Platform: [{}] Shortcode:[{}]", platform, short_code);
        format!("{}/{}", platform, short_code)
    }

    /*
       Combine the short_code with the fedilinks.net to create the url.
    */
    pub fn create_fedilink_url(&mut self, original_url: &str, platform: &str) -> Result<Url, ParseError> {
        let short_code = self.generate_fedilink_shortcode(platform);
        self.url_map
            .insert(short_code.clone(), original_url.to_string());

        let base_url = Url::parse(FEDILINK_BASE_URL)?;
        let full_url = base_url.join(&short_code)?;
        Ok(full_url)
    }

    /*
       Get the url that maps to the passed in fedilink
    */
    pub fn retri_url_from_fedilink(&mut self, fedilink_url: &str) -> Option<&String> {
        let short_code = Url::parse(fedilink_url).unwrap();
        println!("the map is: {:?}", self.url_map);
        println!(
            "looking for key: uh {} blop {}",
            short_code,
            short_code.path().strip_prefix('/').unwrap()
        );
        self.url_map
            .get(short_code.path().strip_prefix('/').unwrap())
    }

    pub fn validate_subdomain(&self, platform: &str) -> Result<(), bool> {
        if self.valid_platforms.contains(platform) {
            Ok(())
        } else {
            println!("The platform provided [{}] is invalid.", platform);
            Err(false)
        }
    }
}
