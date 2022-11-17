# Rust-training-GRPC-protobuf
Trining GRPC protobuf communitacion between Rust apps

Testing Rust GRPC implementation mechanisms and async solutions, which can be used in embedded systems communication. To achive this goal Rust Tonic library has been used (https://github.com/hyperium/tonic). Proto files are automaticly compiled to the Rust source files. Look at "build.rs" file.

## Build & run
```sh
#build and run client binary
cargo run --bin client
#build and run server binary
cargo run --bin server
```
