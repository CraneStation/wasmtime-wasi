# WASI manifest format

The following is a very rough draft.

## Defined module names

First, WASI uses the following import module names:

Name                     | Description
------------------------ | -----------
"wasi:indexed"           | functions that take "resource indices" (integer file descriptors)
"wasi"                   | functions that take "resource references"
"wasi:resources:indexed" | i32 immutable global variable imports with names as described below
"wasi:resources"         | ref immutable global variable imports with names as described below

The "indexed" forms use i32 indices instead of reference types. These
are sometimes called "file descriptors" due to our POSIX heritage.
And currently all the documentation and implementation here are using
the "indexed" forms, but we're starting to anticipate reference-typed
things here too.

## Resource import names

Within the "wasi:resources:indexed" and "wasi:resources" modules, wasm modules
can declare requests for access to various capabilities. The details of the
requests are encoded in the import names.

Note that these aren't a limit on what modules may access, as capabilities may
be passed between instances at runtime.

Resources import names are encoded as '|'-separated substring sequences.

The first substring identifies the type:

Name                     | Description
------------------------ | -----------
"file"                   | a single byte-oriented file
"directory"              | a directory, which may contain files, directories, or other things
"socket"                 | a network socket

### "file" imports

For "file" imports, the next substring is the name, which is
a UTF-8 string.

Backslash is interpreted as an escape character as follows:

Name                     | Description
------------------------ | -----------
"\\"                     | a single backslash
"\|"                     | a literal "|"

Note that the name here isn't a path; it's just a key that the implementation
can map to a path, possibly with input from the user.

The rest of the string is '|'-separated attributes in any order, which may be:

Name     | Description
-------- | -----------
"read"   | supports `fd_read`
"write"  | supports `fd_write`
"tell"   | reading the current position in the file
"seek"   | seeking as with lseek
...      | ...

A "write" import must additionally have exactly one of the following attributes:

Name     | Description
-------- | -----------
"append" | file is opened with "O\_APPEND"
"new"    | file is opened with "O\_EXCL"

Examples:

Import Name                       | Meaning
--------------------------------- | -----------
"file\|errors.log\|write\|append" | file named "errors.log" opened for appending to
"file\|.gitconfig\|read"          | file named ".gitconfig" opened for reading

### "directory" imports

For "directory" imports, the next substring is the name, which
is a UTF-8 string, interpreted in the same as "file" import names.

The name may any application-determined name, though the following names are
recognized and implementations may choose to streamline the user experience
for these paths:

Name        |
----------- |
"Desktop"   | 
"Documents" |
"Downloads" |
"Music"     |
"Pictures"  |
"Videos"    |

The rest of the string is '|'-separated attributes in any order, which may be:

Name    | Description
------- | -----------
"write" | can create files
"list"  | can get a listing of this directory's contents (aka readdir)
...     | ...

Examples:

Import Name                 | Meaning
--------------------------- | -----------
"directory\|Pictures\|list" | can list and read the contents of the user's Pictures folder
"directory\|logs\|write"    | can write to log files (but not see the names of existing log files)

TODO: Does "write" without "list" make sense?

### "socket" imports

For "socket" imports, the next field specifies the type, which is one of the following:

Name            | Description
--------------- | -----------
"stream"        | connection-based socket
"datagram"      | connectionless socket

This is followed by a mode, which is one of the following:

Name                      | Description
------------------------- | -----------
"listen=\<LISTEN\_AT\>"   | listen for incoming connections or packets
"connect=\<CONNECT\_TO\>" | initiate a connection (for "stream") or set a default destination (for datagram); see below for details

The rest of the string is '|'-separated attributes in any order.

#### "listen" sockets

For "listen" sockets, the next field specifies the address, which is one of the following:

"\<LISTEN\_AT\>" starts with the address scope:

Name            | Description
--------------- | -----------
"local"         | allow binding to localhost only, preventing connections from other hosts
"remote"        | allow binding to remotely-accessible addresses

The scope may optionally be followed by a colon and a set of ports,
as a comma-separated list of ranges. Ranges are either single numbers, or
ranges using `[`/`(`/`]`/`)` inclusive/exclusive notation. If this is
omitted, any port is permitted.

TODO: How can applications determine which address(s) they should bind to
for "remote" connections.

The rest of the string is '|'-separated attributes in any order, which may be:

Name                | Description
------------------- | -----------
... | ...

TODO: for "local" sockets, how do we chose between ipv4 and ipv6?

Examples:

Import Name                                 | Meaning
------------------------------------------- | -----------
"socket\|datagram\|listen=remote:80"        | Allow listening for incoming datagrams on port 80
"socket\|stream\|listen=local:\[8080,8090)" | Allow listening on localhost for incoming connections on ports where 8080 <= port < 8090.

#### "connect" sockets

"\<CONNECT\_TO\>" is a comma-separated list of network destination descriptions.

Network destination description start with an address set, which is either
[CIDR notation](https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
or a fully-qualified domain name in which components may be replaced by
"\*" to indicate that any name at that level is to be permitted.

For IPV6 addresses, the CIDR notation is enclosed in brackets ("\[" and "\]").

The address set may optionally be followed by a colon and a set of ports,
as a comma-separated list of ranges. Ranges are either single numbers, or
ranges using `[`/`(`/`]`/`)` inclusive/exclusive notation. If this is
omitted, any port is permitted.

The rest of the string is '|'-separated attributes in any order, which may be:

Name                | Description
------------------- | -----------
... | ...

TODO: Is it worth trying harder to foil deceptive-looking network addresses?
For example, we may want to require special syntax for "access most any host on
the internet" rather than allowing this to be done through CIDR notation, so
that it stands out.

#### socket examples

Examples:

Import Name                        | Meaning
---------------------------------- | -----------
"socket\|stream\|connect=\*.example.com:\[20,22),\[989,991)" | Allow connecting to ftp and ftps ports on hosts under example.com
"socket\|datagram\|connect=10.0.0.0/24:\[0,1024)" | Default to sending datagrams to well known ports on addresses in 10.0.0.\*
"socket\|stream\|connect=\[2001:4860:4860::8888/125\]:80" | IPv6 CIDR notation with brackets, single port

Note that WASI currently does not support `sendto`, so datagram sockets can\'t
send packets to non-default addresses.
