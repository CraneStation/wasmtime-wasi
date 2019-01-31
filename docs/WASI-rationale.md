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

Unfortunately, POSIX doesn't provide us with a ready-made solution, as
it does in many other areas. POSIX's `select`, `poll` and `aio` interfaces
all have limitations, and modern operating systems have grown a wide
variety of interfaces to use in their place, with no apparent sign of any
of them converging toward POSIX-like ubiquity. There are libraries which
provide portable abstractions, such as libuv, and these may eventually
provide insights into a path forward, however we'd still need to figure
out a line between host interfaces and application libraries, as this is
uncharted territory.

The "SI" in "WASI" is the same "SI" as in "POSIX". We didn't add an "X"
at the end because that's for "UNIX", and WASI is sufficiently different
from UNIX that "WASIX" didn't feel accurate -- there's no `fork`, after
all. But it is a "System Interface" that has much in common with POSIX.

Leveraging the highly ubiquitous synchronous parts of POSIX has some
parallels to how the WebAssembly MVP leveraged the great degree of
commonality between general-purpose CPUs. All popular general-purpose
systems have 8-bit bytes, little-endian byte ordering, and two's
complement integers, and this commonality is a large part of why
WebAssembly works. In a somewhat similar way, most general-purpose
computing environments support some flavor of blocking I/O roughly
sufficient to implement WASI calls, particularly when it doesn't
require fork and friends.

We're planning to launch a WebAssembly CG subgroup to maintain WASI
going forward and produce official documents. Once that's in place, this
subgroup can also serve as the forum where a successor to WASI can be
proposed, evaluated, and standardized. Such a successor may wish to go
by a new name, as the "SI" connection to POSIX may well be weaker.

When support for reference-types becomes available, we'll likely want to
change many interfaces to take advantage of them. And UNIX-style
filesystem APIs may want to become a library on top of a more
general-purpose foundation. Maybe people want zero-copy I/O,
everything-is-a-file, everything-is-a-URL, or any number of other OS
abstractions. These abstractions tend to interact, creating an enormous
and complex design space, so a future API will need to be designed
carefully.

And, with WASI's API being defined in terms of wasm exports, if we come
up with something better in the future, it's likely that we'll be able to
reimplement WASI as a library on top of that new thing, and avoid baking
in WASI for the long term.

Synchronous open/close/read/write have been around for over 40 years.
Berkely sockets have been around for over 30 years. Even if we consider
these legacy, it's an impressive legacy. There's a core of functionality
that's so simple and useful, it survived
[wide-spread fragmentation](http://www.unix-diagram.org/) mostly intact.

If you want to run WASI programs in browsers, your options could be
roughly the same as what you have in Emscripten's POSIX-like APIs today:

 - Run the program on a worker, using the sync APIs only available in
   workers, including SharedArrayBuffer to handle the general case
 - Run the program on the main thread but limit all I/O to interacting with
   in-memory data structures, such as in-memory filesystems.
