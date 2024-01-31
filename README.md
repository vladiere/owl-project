# OWL 

OWL Monitoring

## Dependencies need to be installed
First install rustup
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
You will also need the wasm32-unknown-unknown target installed. This installs the necessary tools to compile your code to WebAssembly.
```
rustup target add wasm32-unknown-unknown
```
Install trunk in order to serve WebAssembly
```
cargo install --locked trunk
```
