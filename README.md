# dpar

## Introduction

dpar is a neural network transition-based dependency parser. The
original Go version can be found in the `oldgo` branch.

## Dependencies

### Build-time

* A modern Rust [toolchain](https://rustup.rs).
* libhdf5
* Many Rust crates.

### Run-time

* Tensorflow
* libhdf5 (for training of new parser models)

## Building dpar

To compile and install dpar, run the following in the main project directory:

~~~
cargo install --path dpar-utils
~~~

To do a debug build and run unit tests, run `cargo build` in the main project
directory. To generate API documentation, run `cargo doc`.
