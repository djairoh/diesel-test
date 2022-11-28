# diesel-test
To test the functions in this package, do the following:
```shell
cd ./backend
cargo run --bin <bin_file>
```
where `<bin_file>` refers to one of the files in the bin directory.

For example, to fetch all songs in the database one would run these commands:
```shell
cd ./backend
cargo run --bin get_songs
```