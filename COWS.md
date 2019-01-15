# COWS: Capability-Oriented WebAssembly Syscalls

Capability-Oriented WebAssembly Syscalls, or COWS, is a new POSIX-style API
being built for non-Web WebAssembly.

## Capability-Oriented

The design follows [CloudABI]'s (and in turn [Capsicum])'s concept of
[capability-based security], which fits well into WebAssembly's sandbox
model. Files, directories, network sockets, and other resources are
identified by UNIX-like file descriptors, which are indices into external
tables whose elements represent *capabilities*. Similar to how core WebAssembly
provides no ability to access the outside world without calling imported
functions, COWS APIs provide no ability to access the outside world without
an associated capability.

For example, instead of a typical `open` system call, COWS provides an
`openat`-like system call, requiring the calling process to have a file
descriptor for a directory that contains the file, representing the
*capability* to open files within that directory. (These ideas are common
in capability-based systems, and not new in COWS or CloudABI.)

However, the COWS libc implementation still does provide an implemention of
`open`, by taking the approach of [libpreopen]. Programs may be granted
capabilities for directories on launch, and the library maintains a mapping
from their filesystem path to the file descriptor indices representing the
associated capabilities. When a program calls `open`, they look up the file
name in the map, and automatically supply the appropriate directory capability.
This eases porting of existing applications without compromising the
underlying capability model.

This also allows C programs to have a normal `main` function, rather than
requiring them to use a CloudABI-like `program_main`. `main` can accept
conventional `argv` strings, which can be passed to `open` to open files,
and they're automatically mapped to appropriate capabilities.

COWS also automatically provides file descriptors for standard input and
output, and COWS libc provides a normal `printf`. In general, COWS is aiming
to support a fairly full-featured libc implementation, with the current
implementation work being based on musl.

## WebAssembly Syscalls

COWS is being designed from the ground up for WebAssembly, with sandboxing,
portability, and API tidiness in mind, making natural use of WebAssembly
features such as `i64`, import functions with descriptive names and
typed arguments, and aiming to avoid being tied to a particular
implementation.

COWS inherits much of its design from [CloudABI], which is a very
well designed capability-based POSIX-like API. COWS departs from CloudABI in
order to better suit the needs of WebAssembly, such as in excluding functions
such as `fork` and `exec` which aren't implementable in some of the places
people want to run WebAssembly, and such as in having an expanded scope to
address the many things that people want to do with WebAssembly, besides
running in the cloud.

And, as WebAssembly grows support for host bindings and related features,
capabilities can evolve to being represented as [reference types], and as
WebAssembly's tables become [more powerful][host bindings], the file
descriptor concept can evolve from being an index into an external table to
being an index into a WebAssembly table holding such references.

## Can COWS run on the Web?

With the help of polyfills, it will be able to. Many COWS APIs do not
directly map onto today's Web APIs, and assume the existence of a
filesystem and Berkeley sockets. However, several projects are working
on providing polyfills for this kinds of APIs. At the WebAssembly level,
COWS is just a set of exports that can be imported, and these can be
implemented in a variety of ways, including within browsers.

And in the future, it's possible that [builtin modules] could take these
ideas even further.

## Future Evolution

While CloudABI is attractive as a simple-to-get-started-with yet
complete-enough-to-be-interesting starting point, in the future this API
could evolve into something like [Fuchsia]'s low-level APIs, which are
more complex and abstract, though also more capable.

## Work in Progress

COWS is currently experimental. Feedback is welcome!

[CloudABI]: https://cloudabi.org/
[capability-based security]: https://en.wikipedia.org/wiki/Capability-based_security
[libpreopen]: https://github.com/musec/libpreopen
[Fuchsia]: https://en.wikipedia.org/wiki/Google_Fuchsia
[reference types]: https://github.com/WebAssembly/reference-types
[host bindings]: https://github.com/WebAssembly/host-bindings
[builtin modules]: https://github.com/tc39/ecma262/issues/395
