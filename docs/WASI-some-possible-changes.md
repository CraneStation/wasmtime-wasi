# Possible changes

Currently WASI follows POSIX semantics, when applicable. However, some details
of POSIX aren't unambiguously ideal.

## Detecting EOF from a non-empty read

POSIX's `read` returns 0 if and only if it reaches the end of a file or stream.

Say you have a read buffer of 1024 bytes, and are reading a file that happens
to be 7 bytes long. The first `read` call will return 7, but unless you happen
to know how big the file is supposed to be, you can't distinguish between
that being all there is, and `read` getting interrupted and returning less
data than you requested.

Many applications today do an extra `read` when they encounter the end of a
file, to ensure that they get a `read` that returns 0 bytes read, to confirm
that they've reached the end of the file. If `read` instead had a way to
indicate that it had reached the end, this extra call wouldn't be necessary.

And, `read` on a socket is almost equivalent to `recv` with no flags -- except for
one surprising special case: on a datagram socket, if there's a zero-length
datagram, `read` can't consume it, while `recv` can. This is because `read` can't
indicate that it successfully read 0 bytes, because it has overloaded the meaning
of 0 to indicate eof-of-file.

So, it would be tidier from multiple perspectives if `read` could indicate
that it had reached the end of a file or stream, independently of how many
bytes it has read.

Unfortunately, this should would require implementations using POSIX internally
to do an extra `read` implicitly, and sometimes in cases where it isn't
ultimately needed. And it would take away the ability to do a `read` on a socket
without consuming a zero-length datagram, though it seems unlikely that much
code would be knowingly depending on this.
