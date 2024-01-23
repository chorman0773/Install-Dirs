# Rust Install Dirs

A library for handling install directories, similar to gnu autoconf and cmake. 

## serde

Currently, the `install-dirs` crate requires a serde version which is at most 1.0.171. This is for security as versions of the derive macro starting with 1.0.172 currently ship a precompiled binary that has not been reproduced. Additionally, there are compatibility concerns when porting to non-rustc compilers in the future. If you have a dependency issue, you are recomended to downgrade serde and serde_derive to 1.0.171 if possible.