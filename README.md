# Wasmtime: a WebAssembly Runtime.

Wasmtime is a standalone wasm-only runtime for [WebAssembly], using the [Cranelift] JIT.

It runs WebAssembly code [outside of the Web], and can be used both as a command-line
utility or as a library embedded in a larger application.

[WebAssembly]: https://webassembly.org/
[Cranelift]: https://github.com/CraneStation/cranelift
[outside of the Web]: https://webassembly.org/docs/non-web/

[![Travis Status](https://travis-ci.org/CraneStation/wasmtime.svg?branch=master)](https://travis-ci.org/CraneStation/wasmtime)
[![Appveyor Status](https://ci.appveyor.com/api/projects/status/vxvpt2plriy5s0mc?svg=true)](https://ci.appveyor.com/project/CraneStation/cranelift)
[![Gitter chat](https://badges.gitter.im/CraneStation/CraneStation.svg)](https://gitter.im/CraneStation/Lobby)
![Minimum rustc 1.30](https://img.shields.io/badge/rustc-1.30+-green.svg)

Wasmtime is complete enough to pass the WebAssembly spec testsuite, and supports
a new experimental system API called [WebAssembly System Interface], or WASI.

WASI is derived from and very similar to [CloudABI], and benefits greatly from
CloudABI's clean and thoughtful capability-based design. WASI differs from
CloudABI by aiming to usable in more than just cloud environments, and in making
adaptations to better fit WebAssembly's unique needs.

WASI is a natural complement for WebAssembly, since WebAssembly provides
sandboxing for code but doesn't have any builtin I/O, and WASI provides
sandboxed I/O. See [here][WebAssembly System Interface] to learn more.

Wasmtime does not yet implement Spectre mitiations, such as those being
pioneered [by](https://www.wasmjit.org/blog/spectre-mitigations-part-1.html)
[wasmjit](https://www.wasmjit.org/blog/spectre-mitigations-part-2.html),
however it's a goal to add this.

We're currently working to create Rust C, C++, and other toolchains configured to
use this new API.

[CloudABI]: https://cloudabi.org/
[WebAssembly System Interface]: docs/WASI-overview.md

Additional goals for Wasmtime include:
 - Support a variety of host APIs (not just WASI), with fast calling sequences,
   and develop proposals for system calls in the WebAssembly
   [Reference Sysroot](https://github.com/WebAssembly/reference-sysroot).
 - Implement the [proposed WebAssembly C API].
 - Facilitate testing, experimentation, and development around the [Cranelift] and
   [Lightbeam] JITs.
 - Develop a the native ABI used for compiling WebAssembly suitable for use in both
   JIT and AOT to native object files.

[proposed WebAssembly C API]: https://github.com/rossberg/wasm-c-api
[Cranelift]: https://github.com/CraneStation/cranelift
[Lightbeam]: https://github.com/CraneStation/lightbeam

It's Wasmtime.
