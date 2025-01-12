# Async_nursery Wasm Example

Just showing that async_nursery works in WASM.

## Dependencies

```shell
rustup target add wasm32-unknown-unknown

# See: https://rustwasm.github.io/wasm-pack
#
cargo install wasm-pack
```

## Usage

Unfortunately these days you have to open a webserver, (doesn't have to be the python one) as 
browsers consider it too dangerous to let you load a .js file from a file:// URI.

```shell
git clone  https://github.com/najamelan/async_nursery
cd examples/wasm
wasm-pack build --target web
python -m http.server 8000
```

If all goes well you should see the last line of the output as:
```
| :-) Your wasm pkg is ready to publish at ./pkg.
```

Now open the index.html file in your browser and check the console. It should read:

nursery created
spawned slow: 1
spawned slow: 2
spawned slow: 3
spawned slow: 4
spawned slow: 5
end of resource_outlive.
ended slow: 1
ended slow: 3
ended slow: 2
ended slow: 4
ended slow: 5
