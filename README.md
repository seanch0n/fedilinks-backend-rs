# fedilinks-backend-rs

The rust implementation for the fedilinks backend.

Fedilinks is a service to help facilitate easy sharing of URLs within the fediverse. No longer will you need to share URLs with no idea where they're going.

Runs on top of cloudflare workers


## Features / todo

### Done
- [x] generate short codes for urls and respond

### todo
- [ ] redirect when visited
- [ ] hook up cloudflare workers kv stuff
- [ ] optionally attribute urls to users so they can see their history/share their profile of links




# SCRATCH NOTES TODO

- [x] implement tests for everything
- [ ] look into pulling strings and stuff from env
- [ ] build out consts
- [ ] better errors?
- [ ] it would be really nice to have the psot_async in a handler function. it's gross.