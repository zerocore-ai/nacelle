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

**`nacelle-wasi`** provides a minimal [WebAssembly System Interface (WASI)][wasi] implementation for use in the [`zerocore`][zerocore] project.
It replaces the traditional filesystem implementation and makes some adjustments to the capability system.

##

> [!WARNING]
> This project is in early development and is not yet ready for production use.

##

## Outline

- [Acknowledgments](#acknowledgments)
- [License](#license)

## Acknowledgments

This project makes extensive use of code from the [wasmtime](https://github.com/bytecodealliance/wasmtime) project, specifically the `wasmtime-wasi` sub-crate. The `wasmtime` project is an impactful open-source initiative that enables us to achieve our goals more efficiently by leveraging their robust WebAssembly runtime implementation.

## License

This project is licensed under the [Apache License 2.0](./LICENSE), or
[http://www.apache.org/licenses/LICENSE-2.0][apache].

The `wasmtime-wasi` code utilized in this project is available under the [Apache License 2.0 with LLVM Exception][wasmtime-apache], the same license as the `wasmtime` project itself. We are deeply grateful to the `wasmtime` contributors for their work and encourage users to support the original project.


[apache]: https://www.apache.org/licenses/LICENSE-2.0
[wasmtime-apache]: (https://github.com/bytecodealliance/wasmtime/blob/main/LICENSE)
[buildx]: https://github.com/docker/buildx
[cargo-expand]: https://github.com/dtolnay/cargo-expand
[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-watch]: https://github.com/watchexec/cargo-watch
[commit-spec]: https://www.conventionalcommits.org/en/v1.0.0/#specification
[commit-spec-site]: https://www.conventionalcommits.org/
[docker-engine]: https://docs.docker.com/engine/
[irust]: https://github.com/sigmaSd/IRust
[pre-commit]: https://pre-commit.com/
[zerocore]: https://github.com/zerocore-ai/zerocore
[wasi]: https://wasi.dev/
