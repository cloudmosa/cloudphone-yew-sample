# [Cloud Phone](https://www.cloudfone.com/) - Daily word game in Yew.rs

:beginner: A daily word guessing game in [Rust](https://rust-lang.org/) using [Yew.rs](https://yew.rs/) compiled to WebAssembly (WASM).

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

## :notebook: Prerequisites

* [`rustc` v1.84.0](https://www.rust-lang.org/tools/install)
* [`trunk`](https://trunkrs.dev/)
* [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
* A [GitHub](https://github.com/signup) account :octocat:

## :rocket: Deploy to GitHub Pages

> [!WARNING]  
> COMING SOON!

## :sunglasses: Developer Program

Register for the [Cloud Phone Developer Program](https://www.cloudfone.com/developer-program) to upload test widgets and use the Cloud Phone Simulator.

## :free: License

Licensed under the [Apache 2.0](./LICENSE) license
