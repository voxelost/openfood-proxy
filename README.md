## What is this
This is a simple wrapper around the [Open Food Facts](https://world.openfoodfacts.org) site's search engine. I wrote it as a simple Cloudflare Worker POC and because someone thought it would be a good idea to server-side render HTML and send valid JSON as an inline HTML JS script variable instead of simply sending the JSON straight away and letting the client render their stuff. It uses one of the uglier webscraping hacks I've written so far, so please don't think of it as stable or anyhow production ready.

## How do i deploy it
Tools you'l need:
- `rustup` with `wasm32-unknown-unknown` toolchain installed
- `node`, `npm` or `yarn` and (`wranger` CLI tool)[https://www.npmjs.com/package/wrangler] installed - you can read more about this (here)[https://developers.cloudflare.com/workers/wrangler/install-and-update/]


### Develop locally
```sh
wrangler dev
```

### Build locally
```sh
cargo build --lib --release --target wasm32-unknown-unknown
```

### Deploy to a connected Cloudlare account
```sh
wrangler deploy
```
