## Why not a more traditional set of POSIX-like syscalls?

In related work, the LLVM wasm backend started out trying to use ELF object
files for wasm, to be as conventional as possible. But wasm doesn't fit into
ELF in some very fundamental ways. Code isn't in the address space, callers
have to know their callee's exact signatures, imports and exports don't have
ELF semantics, function pointers require tables to be populated, index 0 is
valid in some contexts where it isn't in ELF, and so on. It ultimately got
to the point where the work we were considering doing to emulate ELF
interfaces to make existing tools happy looked like more than the work that
would be required to just build new tools.

The analogy isn't perfect, but there are some parallels to what we're now
figuring out about system calls. Many people, including myself, had initially
assumed that at least some parts of the wasm ecosystem would eventually
standardize on a basic map of Linux-like system calls into wasm imports.
However, this turns out to be more complex than it initially seems.

One of WebAssembly's unique attributes is the ability to run sandboxed
without relying on OS process boundaries. Requiring a 1-to-1 correspondence
between wasm instances and heavyweight OS processes would take away this key
advantage for many use cases. Fork/exec are the obvious example of an API
that's difficult to implement well if you don't have POSIX-style processes,
but a lot of other things in POSIX are tied to processes too.

We should note that Spectre concerns are relevant here, though for now we'll
just observe that actual security depends on the details of implementations
and use cases, and it's not necessarily a show-stopper.

Another area where WebAssembly differs from traditional POSIX-like platforms
is in its Capability-oriented approach to security. WebAssembly core has no
ability to address the outside world, except through interacting with
imports/exports. And when reference types are added, they'll be able to
represent very fine-grained and dynamic capabilities.

A capability-oriented system interface fits naturally into WebAssembly's
existing sandbox model, by extending the simple story that a wasm module
can't do anything until given capabilities. There are ways to sandbox
Traditional OS filesystem APIs too, but in a multiple-implementation
ecosystem where the methods for setting up path filtering will likely
differ between implementations, designing the platform around capabilities
will make it easier for people to consistently configure the capabilities
available to wasm modules.
