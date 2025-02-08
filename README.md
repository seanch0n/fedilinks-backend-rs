# fedilinks-backend-rs

The rust implementation for the fedilinks backend.

Fedilinks is a service to help facilitate easy sharing of URLs within the fediverse. No longer will you need to share URLs with no idea where they're going.

Runs on top of cloudflare workers

## Run/test/deploy

```aiignore
# run:
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' npx wrangler dev

```


## Features / todo

### Done
- [x] generate short codes for urls and respond
- [x] hook up cloudflare workers kv stuff
- [x] redirect when visited
- [x] link expiration

### todo
- [ ] optionally attribute urls to users so they can see their history/share their profile of links
- [ ] static site for home page
- [ ] better error handling
