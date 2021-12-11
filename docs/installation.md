# Installation

There are two ways to install fortytwo-lang.

- [🆕 Compile and run via WebAssembly](#webassembly)
- [Use docker](#docker)
- [Compile binaries yourself](#compile-yourself)

## WebAssembly

For compiling fortytwo-lang yourself, you need to [install rust](https://www.rust-lang.org/tools/install).

```sh
# Clone repository
$ git clone https://github.com/linuskmr/fortytwo-lang
$ cd fortytwo-lang

# Add WebAssembly WASI target for rust
$ rustup target add wasm32-wasi

# Compile to WebAssembly WASI
$ cargo build --target wasm32-wasi --release

# Run generated WebAssembly binary with wasmer...
$ wasmer run --dir . target/wasm32-wasi/release/ftlcc.wasm examples/extern_write.ftl
# ... or with wasmtime
$ wasmtime run --dir . target/wasm32-wasi/release/ftlcc.wasm examples/extern_write.ftl
```

## Docker

For using the docker container, you need to [install docker](https://docs.docker.com/get-docker/) first.

Alternatively you can use the [online docker playground](https://labs.play-with-docker.com).

### Use existing image

Pull the existing image from [hub.docker.com](https://hub.docker.com):

```
docker pull linuskmr/fortytwo-lang
```

Run fortytwo-lang in the container. Replace `BINARY` with the [binary](/src/bin/) and arguments you want to run.

```
docker run -i linuskmr/fortytwo-lang BINARY
```

### Build docker image yourself

```
docker build -t fortytwo-lang .
```

Run fortytwo-lang in the container. Replace `BINARY` with the [binary](/src/bin/) and arguments you want to run.

```
docker run -i fortytwo-lang BINARY
```

## Compile yourself

For compiling fortytwo-lang yourself, you need to [install rust](https://www.rust-lang.org/tools/install).

### Global installation

```
cargo install --git https://github.com/linuskmr/fortytwo-lang
```

Now you can run it. Replace `BINARY` with the [binary](/src/bin/) and arguments you want to run.

```
BINARY
```

### Compile in local folder

Download the git repository and build:

```sh
git clone https://github.com/linuskmr/fortytwo-lang
cd fortytwo-lang
cargo build --release
```

Now you can run it. Replace `BINARY` with the [binary](/src/bin/) and arguments you want to run.

```
./BINARY
```
