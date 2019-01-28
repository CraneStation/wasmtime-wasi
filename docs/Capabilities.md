# Some misc thoughts about Capabilities.

## Unforgeable references.

One of the key words we want to emphasize is that capabilities are
*unforgeable*.

A pointer in C is forgeable, because untrusted code could cast an integer
to a pointer, thus *forging* access to whatever that pointer value points
to.

MVP WebAssembly doesn't have unforgeable types, but what we can do instead
is just use integer values which are indices into a table that's held outside
the reach of untrusted code. The indices themselves are forgeable, but
ultimately the table is the thing which holds the actual capabilities, and
its elements are unforgeable. There's no way to gain access to a new resource
by making up a new index.

When the reference-types proposal lands, references will be unforgeable, and
will likely subsume the current integer-based APIs, at the WASI API layer.

## Filesystem rules.

It happens that integer indices representing capabilities is same thing that
POSIX does, except that it calls these indices *file descriptors*.

One difference though is that POSIX normally allows processes to request
a file descriptor for any file in the entire filesystem hierarchy, which is
granted based on whatever security policies are in place. This doesn't
violate the capability model, but it doesn't take full advantage of it.

CloudABI, Fuchsia, and other capability-oriented systems prefer to take
advantage of the hierarchical nature of the filesystem and require untrusted
code to have a capability for a directory in order to access things inside
that directory.

So you can launch untrusted code, and at runtime give it access to specific
directories, without having to set permissions in the filesystem or in
per-application or per-user configuration settings.

## Berkely socket rules.

Sockets aren't naturally hierarchical though, so we'll need to decide what
capabilities look like.

The basic version from CloudABI is for users to launch programs with the
sockets they need already created. We'll probably start there, and that's
enough for simple cases.

I anticipate an eventual extension to that, where we create a capability
that represents a set of possible sockets that can be created. A set
might be described by ranges of permitted ports, ranges of permitted
addresses, or sets of permitted protocols. In this case the actual socket
wouldn't be created until the application actually requests it.

## Other info.

CloudABI's intro to capability-based OS security:

https://github.com/NuxiNL/cloudabi#capability-based-security


A Fuchsia blog post about capability-based OS security:

https://fuchsia.googlesource.com/docs/+/HEAD/the-book/dotdot.md
