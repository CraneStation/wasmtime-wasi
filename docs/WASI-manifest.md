# WASI manifest format

The following is a very rough draft.

## Background: Module names

First, WASI uses the following import module names:

Name                     | Description
------------------------ | -----------
`wasi:indexed`           | functions that take "resource indices" (integer file descriptors)
`wasi`                   | functions that take "resource references"
`wasi:resources:indexed` | i32 immutable global variable imports with names as described below
`wasi:resources`         | ref immutable global variable imports with names as described below

The `:indexed` forms use i32 indices instead of reference types. These
are sometimes called "file descriptors" due to our POSIX heritage.
And currently all the documentation and implementation here are using
the `:indexed` forms, but we're starting to anticipate reference-typed
things here too.

## Resource import names

Now, within the `wasi:resources:indexed` and `wasi:resources` modules, wasm
modules can declare requests for access to various capabilities. The details of the
requests are encoded in the import names.

Note that these aren't a limit on what modules may access, as capabilities may
be passed between instances at runtime.

Resources import names are encoded as '|'-separated substring sequences.

The first substring identifies the type:

Name                     | Description
------------------------ | -----------
`file`                   | a single byte-oriented file, preopened
`directory`              | a directory, which may contain files, directories, or other things, preopened
`socket`                 | an openable socket (network or otherwise, stream or otherwise), not pre-opened

### `file` imports

For `file` imports, the next substring is the name, which is a UTF-8 string.

Backslash is interpreted as an escape character as follows:

Name                       | Description
-------------------------- | -----------
"\\\\"                     | a single backslash
"\\\|"                     | a literal "\|"

Note that the name here isn't a path; it's just a key that the implementation
can map to a path, possibly with input from the user. There are no significant
path separators such as '/'.

The rest of the string is "\|"-separated attributes in any order, which may be:

Name       | Description
---------- | -----------
'read'     | supports `fd_read`
'write'    | supports `fd_write`
'tell'     | supports reading the current position in the file
'seek'     | supports seeking as with lseek
'append'   | if file already exists, it is opened for appending
'new'      | if file already exists, `path_open` returns `__WASI_EEXIST`
'truncate' | if file already exists, its size and position are set to 0

`append`, `new`, and `truncate` depend on `write`.

Name     | Description
-------- | -----------

Examples:

Import Name                       | Meaning
--------------------------------- | -----------
`file\|errors.log\|write\|append` | file named "errors.log" opened for appending to
`file\|.gitconfig\|read`          | file named ".gitconfig" opened for reading

### `directory` imports

For `directory` imports, the next substring is the name, which
is a UTF-8 string, interpreted in the same way as `file` import names.

The name may any application-determined name, though the following names are
recognized and implementations may choose to streamline the user experience
for these paths:

Name        |
----------- |
`Desktop`   |
`Documents` |
`Downloads` |
`Music`     |
`Pictures`  |
`Videos`    |

The rest of the string is '|'-separated attributes in any order, which may be:

Name    | Description
------- | -----------
`list`  | can get a listing of this directory's contents (aka readdir)
`write` | can create or rename files

`write` depends on `list`.

Examples:

Import Name                 | Meaning
--------------------------- | -----------
`directory\|Pictures\|list` | can list and read the contents of the user's Pictures folder
`directory\|logs\|write`    | can write to log files (but not see the names of existing log files)

### `socket` imports

For `socket` imports, the next substring is the name, which
is a UTF-8 string, interpreted in the same way as `file` import names.

The next field specifies the domain, which is one of the following:

Name            | Description
--------------- | -----------
`ip`            | IPv4 or IPv6

The next field specifies the type, which is one of the following:

Name            | Description
--------------- | -----------
`stream`        | connection-based socket
`datagram`      | connectionless socket

This is followed by a mode, which is one of the following:

Name                      | Description
------------------------- | -----------
`listen`                  | listen for incoming connections or packets
`connect`                 | initiate a connection (for `stream`) or set a default destination (for `datagram`); see below for details

#### `listen` sockets

For `listen` socket imports, the next substring is the arity, which
is one of the following:

Name            | Description
--------------- | -----------
`single`        | exactly one address will be provided by the implementation
`multiple`      | the implementation may provide multiple addresses

This is optionally followed by a port suggestion, which for `ip` sockets
is `:` followed by either a number, an [IANA port name], or "i want it all 🎶".

Examples:

Import Name                                 | Meaning
------------------------------------------- | -----------
`socket\|ip\|datagram\|listen\|:ntp`        | Request a socket address that maybe listened on for network datagrams, and suggest port 123
`socket\|ip\|stream\|listen\|:8080`         | Request a socket address that maybe listened on for network streams, and suggest port 8080
`socket\|ip\|stream\|listen\|:i want it all 🎶` | Request a socket address that maybe listened on for network streams, and suggest allowing any port

#### `connect` sockets

For `connect` socket imports, an optional destination suggestion may follow.

Destination description for `ip` sockets starts with an address set,
which is either [CIDR notation] or a [domain name] in which components may be
replaced by "\*" to indicate that any name at that level is to be permitted.
For IPV6 addresses, the CIDR notation is enclosed in brackets ("\[" and "\]").
It is followed by ":" and either a port number, an [IANA port name], or
"the whole marshmelon".

Examples:

Import Name                        | Meaning
---------------------------------- | -----------
`socket\|ip\|stream\|connect\|\*.example.com:20` | Suggest allowing connecting to ftp ports on hosts under example.com
`socket\|ip\|stream\|connect\|\[2001:4860:4860::8888/125\]:80` | IPv6 CIDR notation with brackets, single port
`socket\|ip\|datagram\|connect\|10.0.0.0/24:the whole marshmelon` | Suggest allowing sending datagrams to any port on addresses in 10.0.0.\*

Note that WASI does not currently support `sendto`, so datagram sockets can\'t
send packets to non-default addresses.

## To make all this work:

Add functions to create/bind/listen/connect sockets. getpeername? getsockname?

[IANA port name]: https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml
[CIDR notation]: https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing
[domain name]: https://en.wikipedia.org/wiki/Domain_name
