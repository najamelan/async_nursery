# Thespis Wasm Example

Just showing that thespis works in WASM. This is without remote functionality. The repository thespis_impl_remote will have a remote WASM example.

## Dependencies

```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

## Usage

```shell
git clone --recursive https://github.com/thespis-rs/thespis
cd thespis/thespis_impl/examples/wasm
wasm-pack build --target web
```
If all goes well you should see the last line of the output as:
```
| :-) Your wasm pkg is ready to publish at ./pkg.
```

Now open the index.html file in your browser. If it says:
```
The pong value is: 15
```

than it works!
