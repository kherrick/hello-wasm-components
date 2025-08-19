# [Hello Wasm Components](https://github.com/kherrick/hello-wasm-components) ([0.1.0](https://github.com/users/kherrick/packages/container/package/hello-wasm-components))

## dependencies

* [Rust](https://www.rust-lang.org/)
* [Docker](https://www.docker.com/)
* [Wasmtime](https://wasmtime.dev/)
* [regctl](https://github.com/regclient/regclient)
* [wkg (wasm-pkg-tools)](https://github.com/bytecodealliance/wasm-pkg-tools) / [wkg on crates.io](https://crates.io/crates/wkg)

## build

* `cargo build --target wasm32-wasip2 --release`

## serve build

* `wasmtime serve -S cli=y target/wasm32-wasip2/release/hello_wasm_components.wasm`

## authenticating to ghcr.io

* `echo $CR_PAT | docker login ghcr.io -u YOUR_GITHUB_USERNAME --password-stdin`

## pushing to ghcr.io

* `wkg oci push ghcr.io/kherrick/hello-wasm-components:0.1.0 target/wasm32-wasip2/release/hello_wasm_components.wasm`

## pulling from ghcr.io

* `wkg oci pull ghcr.io/kherrick/hello-wasm-components:0.1.0 -o hello_wasm_components.wasm`

## get manifest

* `regctl manifest get ghcr.io/kherrick/hello-wasm-components:0.1.0`

## misc

* `rustup target add wasm32-wasip2`

### troubleshooting on ubuntu

* `ln -s ~/snap/docker/3265/.docker ~/.docker`

### further reading

* [Distributing WebAssembly components using OCI registries ](https://opensource.microsoft.com/blog/2024/09/25/distributing-webassembly-components-using-oci-registries/)
* [The WebAssembly Component Model](https://unit.nginx.org/news/2024/wasm-component-model-part-1)
* [Wasm Component Model By Example](https://tty4.dev/development/wasm-component-model-example/)
