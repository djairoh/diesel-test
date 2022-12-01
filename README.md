# diesel-test
This repo is mostly for testing integration between the Diesel orm and actix-web.
Intended to be used for the webEngineering project this year.

To run web server simply do:
```sh
cd ./backend
cargo run
```

This will start the web-server on `127.0.0.1:8000`.
To test the functionality of the api,
use a command like `curl` to query the various parts of the program.

### currently implemented
At the time of writing, the following api endpoints have been implemented:
 - /songs: fetches all songs in the database
 - /songs/{name}: fetches the first thing to match {name}
 - /artists/{name}: fetches all songs by {artist}