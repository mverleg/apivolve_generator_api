
# Apivolve generator API

This project is part of [Apivolve](https://github.com/mverleg/apivolve).

If you want to create your own code generator for Apivolve in Rust, you do not need to extend Apivolve, just this small library is enough.

Note that you do not need to use Rust, you can write a generator in any other language. In that case, you parse the json yourself (or with another library).

This repository contains:

* The Rust types that Apivolve uses to describe the API
* The boilerplate code for requesting and parsing this data in Rust.
* [Examples](./examples/) of simple generators in a few languages.

## Protocol v1

* Apivolve runs the generator executable (without arguments).
* The generator sends the configuration it expects to use ([`AcceptsConfig`](./src/gen1/connect/accepts.rs)).
* Apivolve sends the generator options, such as output directory.
* Apivolve sends the requested information per version, one version at a time, from new to old.
* For each of those, the generator generates the necessary code, and persists it.
* Apivolve indicates that all versions have been sent, and the generator exits.

