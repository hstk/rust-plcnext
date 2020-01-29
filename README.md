# rust-plcnext

[![crates.io](https://img.shields.io/crates/v/plcnext.svg)](https://crates.io/crates/plcnext)

PLCnext bindings for the Rust programming language.

This crate is currently in the early development stage and should not be used for production applications.

[Getting started](https://github.com/plcnext/rust-sample-runtime).

[Documentation](https://docs.rs/plcnext).

## Building

To build the project, you'll need to export the following three vars; for WSL1, append these to your `.bashrc`.

```
export PATH="$PATH:/opt/pxc/2020.0/sysroots/x86_64-pokysdk-linux/usr/bin:/opt/pxc/2020.0/sysroots/x86_64-pokysdk-linux/usr/sbin:/opt/pxc/2020.0/sysroots/x86_64-pokysdk-linux/bin:/opt/pxc/2020.0/sysroots/x86_64-pokysdk-linux/sbin:/opt/pxc/2020.0/sysroots/x86_64-pokysdk-linux/usr/bin/../x86_64-pokysdk-linux/bin:/opt/pxc/2020.0/sysroots/x86_64-pokysdk-linux/usr/bin/arm-pxc-linux-gnueabi:/opt/pxc/2020.0/sysroots/x86_64-pokysdk-linux/usr/bin/arm-pxc-linux-musl"

export BINDGEN_EXTRA_CLANG_ARGS="-target arm-pxc-linux-gnueabi --sysroot=/opt/pxc/2020.0 -I/opt/pxc/2020.0/sysroots/cortexa9t2hf-neon-pxc-linux-gnueabi/usr/include/plcnext"

export PLCNEXT_HEADERS="-I/opt/pxc/2020.0/sysroots/cortexa9t2hf-neon-pxc-linux-gnueabi/usr/include/plcnext"
```

After that, you should be able to build normally via `cargo build --target=armv7-unknown-linux-gnueabihf`.