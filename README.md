[![TravisCI Build Status](https://travis-ci.org/rust-locale/locale_config.svg?branch=master)](https://travis-ci.org/rust-locale/locale_config)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/13100wtqs80tyink/branch/master?svg=true)](https://ci.appveyor.com/project/jan-hudec/locale-config/branch/master)

# `locale_config`

Remembers locale configuration per-thread and per-process and initializes the
values by inspecting the system for user preferences.

## Installation

You can depend on this library by adding `locale_config` to your Cargo dependencies:

```toml
[dependencies]
locale_config = "*"
```

Usually it is not recommended to depend on `*`, but in this case it is
important that incompatible version requirements don't cause multiple
versions to be pulled in the final binary, so I do recommend it here and
promise I will maintain good compatibility. Just please don't add traits to
the types defined here to avoid conflicts with potential future methods.

## Using

Usually you want to use this indirectly via a localization crate like
`locale`. However if you need to work with the identifier itself, or you need
to override it, use

```rust
Locale::current()
```

to find what you should be using at any given point in the application,

```rust
Locale::set_current()
```

to override it for current thread and

```rust
Locale::set_global_default()
```

to override it for new threads.

In case you need to access the initial value, you'll find it under

```rust
Locale::user_default()
```

See [full documentation](http://rust-locale.github.io/locale_config).
