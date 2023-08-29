# kireta

```console
$ wasm-pack build --no-pack --release --target web crates/wasm
...

$ rm -rf assets/pkg
$ mv crates/wasm/pkg assets/

$ cargo run
...

$ open 'http://localhost:3000/assets/index.html'
```
