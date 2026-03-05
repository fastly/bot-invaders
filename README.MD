# Bot Invaders

This is an implementation of Space Invaders, running as a WASM app in browser. We have themed it to Fastly's Bot Product and used it at conferences. It is paired with the [Leaderboard](https://github.com/fastly/leaderboard) to provide a fun way for attendees to compete for prizes.

# Usage

### Conferences

Bot Invaders is often used at conferences as an attraction that can help draw prospective customers and be a fun competitive game loosely themed around our bot protection product. Most organizers also use it to give out prizes, so it has an integrated leaderboard service that will keep high scores only for your conference and allow you to clear it at the start of the conference and day, if you should choose.

### For conference use, follow these steps

1. Pick a one word name that you can associate with your conference. Something like "rsa" or "cyber_eu".
2. Go to the [leaderboard admin](https://leaderboard.edgecompute.app/?secret=GGluuCFm7aVJ9OMhywPvNIQbmkp5iMha) and create a new leaderboard with your chosen name using the textbox under the "Create New Leaderboard" heading.
3. Confirm that your leaderboard name shows up in the list under "Active Leaderboards".
4. Notice the "Clear" button next to the leaderboard name. You can come back here to clear your leaderboard whenever you wish. Most conference organizers will clear it at the start of the day in order to be able to give out prizes each day.
5. Now, if you use the url https://bot-invaders.edgecompute.app?leaderboard=<YOUR_NAME_HERE>, you should be able to see the name of your leaderboard above the leaderboard. If you do not, it should indicate that the name is not valid; check your spelling or contact jliew@fastly.com

**NOTE** - we recommend that if a player beats the high score that you physically note down their name/initials they used on the leaderboard/contact info. The leaderboard only takes 3 letter initials which is typically insufficient to identify anyone, and we also would hate for a bug to cause their scores to be lost from the leaderboard.

# Building

The core game is written in Rust and compiles to a WASM binary that is loaded by the front end JS. To build the core game, navigate to the `game` directory and run `cargo build --release --target=wasm32-unknown-unknown`

You can test by running `python3 -m http.server`

To deploy to Compute, run `fastly compute publish` from the `compute` directory. The service is owned by jliew@fastly.com, so you'll need a key that can access that service to deploy. 