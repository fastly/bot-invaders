# Bot Invaders

This is an implementation of Space Invaders, running as a WASM app in browser. We have themed it to Fastly's Bot Product and used it at conferences. It is paired with the [Leaderboard](https://github.com/fastly/leaderboard) to provide a fun way for attendees to compete for prizes.

# Building

The core game is written in Rust and compiles to a WASM binary that is loaded by the front end JS. To build the core game, navigate to the `game` directory and run `cargo build --release --target=wasm32-unknown-unknown`

You can test by running `python3 -m http.server`

To deploy to Compute, run `fastly compute publish` from the `compute` directory. The service is owned by jliew@fastly.com, so you'll need a key that can access that service to deploy. 