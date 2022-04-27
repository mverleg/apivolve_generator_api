
# Examples of apivolve generators

There are a few examples of how to create an apivolve generator:

* An example of a generator in Rust with this library ([`debug_dump.rs`](rust_example/src/debug_dump.rs)).
* An example of a generator in Python _without_ this library ([`debug_dump.py`(./apivolve-gen1-python-example)]).

These are simple examples. They focus on the communication betwene Apivolve and the generator, now on generating fully usable code.

Important for the file your generator lives in:

* It must be executable (on linux/macos: `chmod +x filename`).
* The name must start with `apivolve-gen1-` to be found by Apivolve.
* It must be in a directory that is included in `$PATH`.

