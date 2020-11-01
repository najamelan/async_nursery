# async_nursery

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/async_nursery.svg?branch=master)](https://travis-ci.org/najamelan/async_nursery)
[![Docs](https://docs.rs/async_nursery/badge.svg)](https://docs.rs/async_nursery)
[![crates.io](https://img.shields.io/crates/v/async_nursery.svg)](https://crates.io/crates/async_nursery)


> Primitive for structured concurrency.

The nursery allows writing concurrent programs adhering to structured concurrency. If you are new to the concept, there are some excellent resources on the dedicated [structured concurrency forum](https://trio.discourse.group/t/structured-concurrency-resources/21). The name of the library is inspired by the excellent python [Trio library](https://github.com/python-trio/trio).


## Table of Contents

- [Description](#description)
- [Install](#install)
  - [Upgrade](#upgrade)
  - [Dependencies](#dependencies)
  - [Security](#security)
  - [Performance](#performance)
- [Usage](#usage)
  - [Basic Example](#basic-example)
  - [Returning errors](#returning-errors)
  - [Recover other return types](#recover-other-return-types)
  - [Panics](#panics)
  - [Differences with FuturesUnordered](#differences-with-FuturesUnordered)
  - [API](#api)
- [Contributing](#contributing)
  - [Code of Conduct](#code-of-conduct)
- [License](#license)


## Description

_async_nursery_ brings a structured concurrency primitive to Rust. There are three main goals in structured concurrency:

### 1. A sane control flow for concurrent programs.

[Notes on structured concurrency, or: Go statement considered harmful](https://vorpus.org/blog/notes-on-structured-concurrency-or-go-statement-considered-harmful/) by Nathaniel J. Smith explains this exquisitely. To summarize, if a function wants to split of and do some work concurrently, make sure that all it's child tasks are finished when the function returns. That way it functions as the black box we are used to from synchronous code. A function has inputs and a return value, and when it is done, no code it created is running anymore.

You could already do this by stuffing `JoinHandle`s from _async_executors_ in a `FuturesUnordered`, but as we will see below, _async_nursery_ is a bit more flexible and convenient. As opposed to the `JoinHandle`s from _tokio_ or _async-std_ directly, the ones from _async_executors_ do not detach on drop by default.

### 2. Prevent resource leakage.

Orphaned tasks, spawned without a JoinHandle can potentially stay alive forever, either running in loops, or deadlocking. Structured concurrency makes sure there are no leaks, putting all resources neatly in a call tree, very similar to a call stack. In a call tree, a stack frame can be several stack frames sitting side by side doing things concurrently, but when we return to the previous stack frame, all of them are done.

### 3. Propagate errors

In Rust it is common to propagate errors up the call stack. If you spawn a task and let it run off in the void, you need out of band error handling like channels. In structured concurrency, since all tasks get joined before their parent returns, you can return the errors just like in sync code. It is also possible to cancel all sibling tasks if one task runs into an error.

## Properties of _async_nursery_:

- `Nursery` acts as spawner.
- `NurseryStream` implements `Stream<Out>` of the results of all the futures it nurses.
- `NurseryStream` implements  `Future<Output=()>` if you just want to wait for everything to finish, but don't care for returned values.
- `NurseryStream` basically manages `JoinHandle`s for you.
- Can be backed by any executor that implements [`SpawnHandle`](https://docs.rs/async_executors/*/async_executors/trait.SpawnHandle.html) or [`LocalSpawnHandle`](https://docs.rs/async_executors/*/async_executors/trait.LocalSpawnHandle.html).
- Cancels all running futures on drop.
- `Nursery` implements Sink for [`FutureObj`](https://docs.rs/futures/*/futures/task/struct.FutureObj.html) and/or [`LocalFutureObj`](https://docs.rs/futures/*/futures/task/struct.LocalFutureObj.html) as well as `Nurse` and `NurseExt`.


## Missing features

- **timeouts**: timers are quite tightly coupled with executors so it seems and there is no integration for timers in _async_executors_ yet. Both _tokio_ and _async-std_ have a `timeout` method and _futures-timer_ can work for anything else but will create a global timer thread and could have some overhead compared to executor specific implementations. However that's not much good for agnostic libraries. I will look into that, but until then you will have to choose your timeout implementation manually.

- No API provided for **cooperative cancellation**. Since there is no support for that in `std::task::Context`, you must basically pass some cancellation token into a task __that needs to do cleanup and doesn't support being dropped at all await points__. Since it requires specific support of the spawned task, I leave this to the user. An example using an `AtomicBool` is included in the [examples directory](https://github.com/najamelan/async_nursery/blob/master/examples). The advantage is flexibility. You could cancel just certain tasks in the nursery and leave others running, or let the others be canceled by drop if they support it, etc. [Async drop](https://internals.rust-lang.org/t/asynchronous-destructors/11127) will most likely alleviate this pain one day, but it's not there yet.

- No API is provided for running non-`'static` futures. This is not possible in safe rust because `std::mem::forget` could be used to leak the nursery and trick it to outlive it's parent stack frame, at which point it would hold an invalid reference. If you really want to go there, I suggest you look at the [_async-scoped_](https://docs.rs/async-scoped) crate which allows it by requiring you to use unsafe.


## Install
With [cargo add](https://github.com/killercup/cargo-edit):
`cargo add async_nursery`

With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
```yaml
dependencies:

   async_nursery: ^0.3.0-beta
```

With Cargo.toml
```toml
[dependencies]

   async_nursery = "0.3.0-beta"
```

### Upgrade

Please check out the [changelog](https://github.com/najamelan/async_nursery/blob/master/CHANGELOG.md) when upgrading.


### Dependencies

This crate has few dependencies (_futures_ and _async_executors_). Cargo will automatically handle it's dependencies for you. You will have to choose executors from the _async_executors_ crate and set the correct feature on that crate to enable it.

There are no optional features.


### Security

The crate uses `forbid(unsafe)`, but depends on `futures` which has quite some unsafe. There are no security issues I'm aware of specific to using this crate.


### Performance

Currently the implementation is simple. `Nursery` just sends the `JoinHandle` to `NurseryStream` over an unbounded channel. This is convenient, because it means `NurseExt::nurse` doesn't have to be async, but it has some overhead compared to using the underlying executor directly. In the future I hope to optimize the implementation.

## Usage

**Warning**: If ever you wait on the stream to finish, remember it will only finish if there are no `Nursery`'s alive anymore. You must drop the Nursery before awaiting the `NurseryStream`. If your program deadlocks, this should be the first place to look.

All tasks spawned on a nursery must have the same `Future::Output` type.

### Basic example

There is an extensive list of examples for all kinds of patterns of using _async_nursery_ in the [examples directory](https://github.com/najamelan/async_nursery/blob/master/examples). Please have a look at them.

```rust
use
{
   async_nursery   :: { Nursery, NurseExt } ,
   async_executors :: { AsyncStd          } ,
};

pub type DynResult<T> = Result<T, Box< dyn std::error::Error + Send + Sync + 'static >>;

async fn self_contained() -> DynResult<()>
{
   let (nursery, output) = Nursery::new( AsyncStd );

   for _ in 0..5
   {
      nursery.nurse( async { /* do something useful */ } )?;
   }

   // This is necessary. Since we could keep spawning tasks even after starting to poll
   // the output, it can't know that we are done, unless we drop all senders or call
   // `close_nursery`. If we don't, the await below deadlocks.
   //
   drop(nursery);

   // Resolves when all spawned tasks are done.
   //
   output.await;

   Ok(())
}
```

### Returning errors

The functionality of `TryStreamExt::try_next` can be used to bail early if all concurrent tasks need to complete successfully. You can now drop the `NurseryStream` and cancel all running sibling tasks.

### Recover other return types

It's possible to return useful data sometimes from spawned tasks. You can effectively see them as function calls or closures that can run concurrently. The nursery let's you recover these as you go. It could be used to implement a progress bar for example.

Another possibility is using `collect` to gain a collection of all returned values when everything is done.

### Panics

Nursery has no special handling of panics. If your task panics, it depends on the executor what happens. Currently _tokio_ is different from other executors in that it will `catch_unwind` your spawned tasks. Other executors propagate the panic to the thread that awaits the `JoinHandle`s (eg. that awaits the `NurseryStream`). If you want a resilient application that works on all executors, use the `catch_unwind` combinator from the futures library. Again using `TryStreamExt::try_next` you can bail early if one task panics.

### Differences with FuturesUnordered

`Nursery` and `NurseryStream` wrap a FuturesUnordered internally. The main feature this gives us is that it allows to us to start polling the stream of outputs and still continue to spawn more subtasks. `FuturesUnordered` has a very strict two phase API. First spawn, then get output. This allows us to use `NuseryStream` as a long-lived container. Eg. if you are going to spawn network requests, you can continuously listen to `NurseryStream` for errors that happened during processing while continuing to spawn further requests. Then when the connection closes, we want to stop processing outstanding requests for this connection. By dropping the `NurseryStream`, we can do that.

Further a few conveniences are added:
  - `Nursery` does the spawning for you, never need to handle `JoinHandle`s manually.
  - `NurseryStream` not only implements `Stream`, but also `Future`, if you just want to wait for everything to finish and don't care for the Outputs.
  - `Nursery` can be cloned and send around, into function calls and spawned subtasks. You don't have to send back the `JoinHandle`s through a channel manually to push them into the `FuturesUnordered`.


## API

API documentation can be found on [docs.rs](https://docs.rs/async_nursery).


## Contributing

Please check out the [contribution guidelines](https://github.com/najamelan/async_nursery/blob/master/CONTRIBUTING.md).


### Testing

`cargo test` and `wasm-pack test --firefox --headless -- -Z features=itarget --no-default-features` although the latter requires nightly and doesn't work until https://github.com/rustwasm/wasm-pack/issues/698 is resolved or you patch wasm-pack. You could use `wasm-bindgen-cli`.


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](https://github.com/stumpsyn/policies/blob/master/citizen_code_of_conduct.md#4-unacceptable-behavior) are not welcome here and might get you banned. If anyone, including maintainers and moderators of the project, fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)

