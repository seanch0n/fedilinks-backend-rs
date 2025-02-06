pub use crate::constants::{
    ALPHANUMERIC, FEDILINK_BASE_URL, FEDILINK_REDIR_URL, FEDILINK_SHORT_CODE_LENGTH,
    VALID_FEDILINK_SUBDOMAINS,
};

use getrandom;
use std::collections::{HashMap, HashSet};
use url::{ParseError, Url};
use worker::js_sys::Array;

pub struct Fedilinker {
    url_map: HashMap<String, String>,
    valid_subdomains: HashSet<String>,
}
impl Fedilinker {
    pub fn new() -> Self {
        let valid_subdomains_set = VALID_FEDILINK_SUBDOMAINS
            .iter()
            .cloned()
            .collect::<HashSet<_>>();

        Self {
            url_map: HashMap::new(),
            valid_subdomains: valid_subdomains_set,
        }
    }


    /*
       Generate a fedilink shortcode, which is an 8 character short code.
       We have to use the getrandom crate instead of rand because it supports
       wasm-js which we need for cloudflare workers
    */
    fn generate_fedilink_shortcode(&self) -> String {
        let mut buf = [0; FEDILINK_SHORT_CODE_LENGTH];
        getrandom::fill(&mut buf).expect("Failed to get random bytes");

        // Map random bytes to characters in the ALPHABET/upper/lower/numbers
        let short_code: String = buf
            .iter()
            .map(|&b| ALPHANUMERIC[(b as usize) % ALPHANUMERIC.len()] as char)
            .collect();

        println!("Shortcode: {}", short_code);
        format!("{}/{}", FEDILINK_REDIR_URL, short_code)
        // short_code
    }

    /*
       Combine the short_code with the fedilinks.net to create the url.
    */
    pub fn create_fedilink_url(&mut self, original_url: &str, platform: &str) -> Result<Url, ParseError> {
        let short_code = self.generate_fedilink_shortcode();
        self.url_map
            .insert(short_code.clone(), original_url.to_string());

        //TODO: we need to add subdomain/platform to the fedilink. We are parsing it from
        //TODO: the incoming request, but we need to add it to the fedilink, and then probably
        //TODO: change how we parse the fedilinks on retrival, and add test cases for it too.
        let mut url = Url::parse(FEDILINK_BASE_URL)?;
        url.set_path(&short_code);
        Ok(url)
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

    pub fn validate_subdomain(&self, subdomain: &str) -> Result<(), String> {
        if self.valid_subdomains.contains(subdomain) {
            Ok(())
        } else {
            Err(format!("Invalid subdomain: {}", subdomain))
        }
    }
}
