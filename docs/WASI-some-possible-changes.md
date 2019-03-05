# Possible changes

Currently WASI follows POSIX semantics, when applicable. However, some details
of POSIX aren't unambiguously ideal.

## Argv/argc, environment variables

These are planned but not implemented yet.

## Detecting EOF from read/recv explicitly.

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

## More detailed capability error reporting

Replace `__WASI_ENOTCAPABLE` with error codes that indicate *which* capabilities
were required but not present.

## Rename directory-oriented syscalls to `__wasi_dir_*`?

`__wasi_fd_readdir` isn't really about files, so maybe it could be
`__wasi_dir_read`, along with renaming `__wasi_file_create_directory`
to `__wasi_dir_create`, and `__wasi_file_unlink_directory` to
`__wasi_dir_unlink`.

We could also split `__WASI_RIGHT_FILE_OPEN` into file vs directory, and
then maybe also split a `__wasi_dir_open` out of `__wasi_file_open` too
(obviating `__WASI_O_DIRECTORY`).

## Fix the y2556 bug

In some places, timestamps are measured in nanoseconds since the UNIX epoch,
so our calculations indicate a 64-bit counter will overflow on
Sunday, July 21, 2554, at 11:34:33 pm UTC.

These timestamps aren't used in that many places, so it wouldn't cost that
much to widen these timestamps. We can either just extend the current type to
128 bits (two i64's in wasm) or move to a `timespec`-like `tv_sec`/`tv_nsec`
pair.

## Remove `fd_allocate`?

Darwin doesn't implement `fd_allocate`, despite it being a in POSIX
since 2001. So we don't currently know any way to implement `fd_allocate`
on Darwin that's safe from race conditions. Should we remove it from the API?

## Redesign `fstflags_t`

The relationship between `*_SET_*TIM` and `*_SET_*TIM_NOW` is non-obvious.
We should look at this again.

## readdir

Truncating entries that don't fit into a buffer may be error-prone. Should
we redesign how directory reading works?

# symlinks

Symlinks are fairly UNIX-specific. Should we remove `__wasi_file_symlink`
and `__wasi_file_readlink`?
