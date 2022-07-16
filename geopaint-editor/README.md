# Geopaint Editor

A web application to paint geometric figures.

## Getting started

### Pre-Requirements

#### Rust & Wasm Tooling

1. install rust: <https://www.rust-lang.org/tools/install>
2. install WebAssembly target
   ```
   rustup target add wasm32-unknown-unknown
   ```
3. Install Trunk
   ```
   cargo install trunk
   ```

#### Serve Application

As a developer you can serve the project locally using:

```
trunk serve
```

Should you wish to serve your code in release mode you can do so by applying the `--release` option:

```
trunk serve --release
```

### Release Application

First step is to build the application using the following command.

```
trunk build --release
```

The resulting content in [./dist](./dist) can be copied to your server (or Docker image)
and served using your favorite static web server.
