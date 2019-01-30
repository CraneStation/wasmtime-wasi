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

## Why not async?

There's a huge range of use cases for async I/O, from applications
running in browsers that need to return to the event loop regularly
to let it service the UI, to servers with many thousands of clients.

Neither of the problems mentioned here has a ready solution in POSIX, or
CloudABI. POSIX's `select`, `poll` and `aio` interfaces all have
limitations, and modern operating systems have grown a wide variety of
interfaces to use in their place, with no apparent sign of any of them
converging toward POSIX-like ubiquity. There are libraries which provide
portable abstractions, such as libuv, and these may eventually provide
insights into a path forward, however we'd still need to figure out a
line between host interfaces and application libraries, as this is
uncharted territory.

The "SI" in "WASI" is the same "SI" as in "POSIX". We didn't add "X"
at the end because that's for "UNIX", and WASI is sufficiently different
from UNIX that "WASIX" didn't feel sufficiently accurate. What's a UNIX
without `fork`, for example? But it is a "System Interface" very much in
the spirit of POSIX.

And, we know that future WebAssembly API iterations may look radically
different anyway. Particularly, when support for reference-types becomes
available, we're going to want to massively redesign a lot of things
no matter what. And those UNIX-style filesystem APIs may want to become
a library on top of a more general-purpose foundation. And maybe people
want zero-copy I/O. Or everything-is-a-file. Or everything-is-a-URL.
Or any number of other OS abstractions. These abstractions all interact,
creating an enormous and complex design space.

Leveraging the highly ubiquitous synchronous parts of POSIX has strong
parallels to how the WebAssembly MVP leveraged the great degree of
commonaly commonality between general-purpose CPUs. Everyone has
8-bit bytes, little-endian, and two's complement integers, and this
commonality is a large part of why WebAssembly works. Similarly,
most general-purpose computing environments support some flavor of
blocking I/O roughly sufficient to implement WASI calls, particularly
when it doesn't require fork and friends. And like 128-bit SIMD, it's
not quite as universal as little-endian etc., but it's pretty good, and
much much simpler than more theoretically portable alternatives.

We're planning to launch a WebAssembly CG subgroup to maintain WASI
going forward and produce official documents. Once that's in place, this
subgroup can serve as the forum where WASI's successor can be proposed,
evaluated, and standardized. Quite possibly it will go by a new name,
as the "SI" relationship to POSIX may well be weaker.

Synchronous open/close/read/write have been around for over 40 years.
Even if we consider them legacy, it's an impressive legacy. There's a core
of functionality that's so simple and useful, it survived
[intense large-scale fragmentation](https://en.wikipedia.org/wiki/History_of_Unix#/media/File:Unix_history-simple.svg)
and the [UNIX wars](https://en.wikipedia.org/wiki/Unix_wars) mostly
intact.

And, with this API being defined in terms of wasm exports, if we invent
something better in the future, we can reimplement WASI in terms of that
new thing, and avoid baking in WASI for the long term.

So if you want to run WASI programs in browsers, your options could be
roughly the same as what you have in Emscripten's POSIX-like APIs today:

 - Run the program on a worker, optionally with proxying to the main thread.
 - Run the program on the main thread but limit all I/O to interacting with
   in-memory data structures.
