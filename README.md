<div align="center">
  <!-- <a href="https://github.com/zerocore-ai/nacelle-wasi" target="_blank">
    <img src="https://raw.githubusercontent.com/zerocore-ai/nacelle-wasi/main/assets/a_logo.png" alt="nacelle-wasi Logo" width="100"></img>
  </a> -->

  <h1 align="center">nacelle-wasi</h1>

  <!-- <p>
    <a href="https://crates.io/crates/nacelle-wasi">
      <img src="https://img.shields.io/crates/v/nacelle-wasi?label=crates" alt="Crate">
    </a>
    <a href="https://codecov.io/gh/zerocore-ai/nacelle-wasi">
      <img src="https://codecov.io/gh/zerocore-ai/nacelle-wasi/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>
    </a>
    <a href="https://github.com/zerocore-ai/nacelle-wasi/actions?query=">
      <img src="https://github.com/zerocore-ai/nacelle-wasi/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/zerocore-ai/nacelle-wasi/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
    </a>
    <a href="https://docs.rs/nacelle-wasi">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Docs">
    </a>
  </p> -->
</div>

**nacelle** is a runtime and a collection of libraries for running WASI and zero-enabled applications. It introduces a new capability system that diverges a bit from the traditional WASI capabilities.

### Crate Dependency Graph

```mermaid
flowchart TD
    nacelle --> nacelle-wasi
    nacelle --> nacelle-zero

    nacelle-wasi --> cli
    nacelle-wasi --> clocks
    nacelle-wasi --> random
    nacelle-wasi --> sockets
    nacelle-wasi --> filesystem
    nacelle-wasi --> http

    nacelle-zero --> db
    nacelle-zero --> ml
    nacelle-zero --> ...
```

</br>

> [!WARNING]
> This project is in early development and is not yet ready for production use.

##

## Outline

- [Acknowledgments](#acknowledgments)
- [License](#license)

## Acknowledgments

This project makes use of code from the [wasmtime][wasmtime] project. The `wasmtime` project is an impactful open-source initiative that enables us to achieve our goals more efficiently by leveraging their robust WebAssembly runtime implementation.

## License

This project is licensed under the [Apache License 2.0](./LICENSE).

[apache]: https://www.apache.org/licenses/LICENSE-2.0
[zerocore]: https://github.com/zerocore-ai/zerocore
[wasi]: https://wasi.dev/
[wasmtime]: https://github.com/bytecodealliance/wasmtime
[cap_std]: https://github.com/bytecodealliance/cap-std
