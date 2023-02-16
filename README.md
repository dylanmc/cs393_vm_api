This is a first whack at defining a medium-level abstraction of an
address space in Rust.

The idea is that other kernel operations use this API to create and
initialize an address space, that user-space operations can end up
calling (via a syscall bridge) operations like `add_map` and `flush`
to do things to their address space and data, and that the unified VM
/ File Buffer Cache will use the DataSource interface & (some still unnamed)
*cache coordinator* to cache data from DataSources.

First we need to make sure this API is complete. For one thing, it's
missing Flags in the `add_map` call -- *"what are your intentions?"*
with this mapping, `READ`, `WRITE`, `EXECUTE`, `COPY_ON_WRITE`,
`PRIVATE`, (vs.) `SHARED`, etc.

Next, we need some tests that make sure the `add_map` function
actually adds a mapping, that the mappings don't ever overlap, that
mappings can be removed, and virtual space reclaimed, etc.

At some point we'll want to migrate this into `reedos`, so we'll want
to switch to `#no_std`, use a kernel memory allocator, etc.
