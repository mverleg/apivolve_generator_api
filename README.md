
# Apivolve generator API

This project is part of [Apivolve](https://github.com/mverleg/apivolve), a project to describe api changes in evolution files.

The api description is automatically converted to code for parsing, including full backwards compatibility. This is done by generators for each language, which can be written fully independently from the main Apivolve repo.

This project is for if you want to use Rust to create your own Apivolve generator. You can use any other language you want; in that case, you parse the json yourself and do not need this library ([example](./examples/apivolve-gen1-python-example)).

This repository contains:

* The Rust types that Apivolve uses to describe the API
* The boilerplate code for requesting and parsing this data in Rust.
* [Examples](./examples/) of simple generators in a few languages.

## Protocol v1

Version 1 is currently the latest/only version.

Communication happens over a localhost TCP connection.

EDIT: support embedded (Rust) only for now, and then bootstrap the protocol with Apivolve itself

* Apivolve runs the generator executable, with tcp address as argument.
* The generator sends the configuration it expects to use ([`AcceptedFormat`](./src/gen1/connect/acceptedformat.rs)).
* Apivolve sends the generator options, such as output directory and arguments ([`UserPreferences`](./src/gen1/connect/userpreferences.rs)).
* The generator sends the features and parties it wants, with ([`FunctionalityRequest`](./src/gen1/connect/functionalityrequest.rs)).
* Apivolve sends the requested information per version, one version at a time, from new to old.
* For each of those, the generator generates the necessary code, and persists it.
* Apivolve indicates that all versions have been sent, and the generator exits.

