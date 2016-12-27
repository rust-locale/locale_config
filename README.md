[![TravisCI](https://travis-ci.org/rust-locale/locale_config.svg?branch=master)](https://travis-ci.org/rust-locale/locale_config)

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
