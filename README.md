#

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```bash
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```bash
wasm-pack build
```

to generate LLVM

```bash
RUSTFLAGS="--emit=llvm-ir" wasm-pack build
```

the output LLVM will be in

```text
target/wasm32-unknown-unknown/release/deps/
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```bash
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```bash
wasm-pack publish
```

### From C++

CPP -> LLVM-IR

```bash
clang main.cpp -S -emit-llvm
```

LLVM-IR -> WASM

```bash
llc-10 -mtriple=wasm32-unknown-unknown -O3 -filetype=obj main.ll -o main.o
wasm-ld-10 main.o -o main.wasm --no-entry -allow-undefined --export-all
```

### Transcoding the LLVM-IR

```bash
xcode -compiler llvm main.ll  -function_indirection off -light_datatransform_level 100 -controlflowlevel 0 -auto_transforms off -auto_function_transforms off -string_transforms off -o main.cloak.ll
```

LLVM-IR -> WASM (of the cloaked LLVM)

```bash
llc-10 -mtriple=wasm32-unknown-unknown -O3 -filetype=obj main.cloak.ll -o main.cloak.o
wasm-ld-10 main.cloak.o -o main.cloak.wasm --no-entry -allow-undefined --export-all
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
