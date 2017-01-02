[![TravisCI Build Status](https://travis-ci.org/rust-locale/locale_config.svg?branch=master)](https://travis-ci.org/rust-locale/locale_config)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/13100wtqs80tyink/branch/master?svg=true)](https://ci.appveyor.com/project/jan-hudec/locale-config/branch/master)
[![Crates.io Version](https://img.shields.io/crates/v/locale_config.svg)](https://crates.io/crates/locale_config)
[![Docs.rs](https://docs.rs/locale_config/badge.svg)](https://docs.rs/locale_config/)

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

See full documentation on [![Docs.rs](https://docs.rs/locale_config/badge.svg)](https://docs.rs/locale_config/) or [github](https://rust-locale.github.io/locale_config/locale_config/).

## Supported systems

* **Unix:** Using the POSIX standard environment variables `LANG`, `LC_*` and
  `LANGUAGES`. The variables are recognized on all systems and take
  precedence on most of them.

* **Windows:** Vista and newer

    - Uses API available from Vista and Server 2008 only.
    - The `GetUserPreferredUILanguages` is only available for desktop, but
      not store applications. Store applications should have equivalent
      functionality, but I didn't try accessing it from Rust yet.
    - Customization to individual locale elements done in “Regional and
      Language options” (digits, calendar, decimal and thousand separator
      etc.) are not detected (yet).
    - Not well tested.

* **CGI:** The `HTTP_ACCEPT_LANGUAGE` environment variable is used if
  detected. Hopefully it is specific enough to the CGI environment that it
  can be used whenever detected.

## Changelog

### 0.1.1

* Added basic Windows support.

### 0.1.0

* Initial version, with Unix and CGI support.
