# async_nursery

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/async_nursery.svg?branch=master)](https://travis-ci.org/najamelan/async_nursery)
[![Docs](https://docs.rs/async_nursery/badge.svg)](https://docs.rs/async_nursery)
[![crates.io](https://img.shields.io/crates/v/async_nursery.svg)](https://crates.io/crates/async_nursery)


> [tagline]

The nursery allows writing concurrent programs adhering to structured concurrency. If you are new to the concept, here are some excellent ressources to look at:
- [Notes on structured concurrency, or: Go statement considered harmful](https://vorpus.org/blog/notes-on-structured-concurrency-or-go-statement-considered-harmful/) by Nathaniel J. Smith.
-

Properties:

- act as spawner
- implements `Stream<Out>` of the results of all the futures it nurses.
- basically manages joinhandles for you.
- be backed by any executor that implements SpawnHandle.
- cancels all running futures on drop
- co-operative canceling? observable? problem is requirement for clone and most error types do not implement clone.
- a function leaves no orphaned tasks. When it returns, all of it's concurrent tasks have ended, or it has added tasks
  to a nursery that was passed in.

## Table of Contents

- [Install](#install)
   - [Upgrade](#upgrade)
   - [Dependencies](#dependencies)
   - [Security](#security)
- [Usage](#usage)
   - [Basic Example](#basic-example)
   - [API](#api)
- [Contributing](#contributing)
   - [Code of Conduct](#code-of-conduct)
- [License](#license)


## Install
With [cargo add](https://github.com/killercup/cargo-edit):
`cargo add async_nursery`

With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
```yaml
dependencies:

   async_nursery: ^0.1
```

With Cargo.toml
```toml
[dependencies]

    async_nursery = "0.1"
```

### Upgrade

Please check out the [changelog](https://github.com/najamelan/async_nursery/blob/master/CHANGELOG.md) when upgrading.


### Dependencies

This crate has few dependencies. Cargo will automatically handle it's dependencies for you.

There are no optional features.


### Security




## Usage



### Basic example

```rust

```

## API

API documentation can be found on [docs.rs](https://docs.rs/async_nursery).


## Contributing

Please check out the [contribution guidelines](https://github.com/najamelan/async_nursery/blob/master/CONTRIBUTING.md).


### Testing


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](https://github.com/stumpsyn/policies/blob/master/citizen_code_of_conduct.md#4-unacceptable-behavior) are not welcome here and might get you banned. If anyone, including maintainers and moderators of the project, fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)

