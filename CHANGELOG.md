# async_nursery - CHANGELOG

## [Unreleased]

  [Unreleased]: https://github.com/najamelan/async_nursery/compare/0.6.0...dev

## [0.6.0] - 2025-01-12

  [0.6.0]: https://github.com/najamelan/async_nursery/compare/0.5.0...0.6.0
  
## Updated
  - **BREAKING**: Updated _async_executors_ to 0.7. Note that TokioCt now throws
    an error that is not `Send`.

## [0.5.0] - 2022-05-11

  [0.5.0]: https://github.com/najamelan/async_nursery/compare/0.4.1...0.5.0
  
## Updated
  - **BREAKING**: Updated _async_executors_ to 0.6.

## [0.4.1] - 2022-05-11

  [0.4.1]: https://github.com/najamelan/async_nursery/compare/0.4.0...0.4.1
  
## Fixed
  - Properly require features on our dependencies when we dont have all features enabled.

## [0.4.0] - 2021-01-06

  [0.4.0]: https://github.com/najamelan/async_nursery/compare/0.3.1...0.4.0
  
## Added
  - a default feature "implementation" that allows to get only the traits when
    depending with `default-features: false`. This will shed some dependencies.
  - Forward all new traits from async_executors on the `Nursery`, so now 
    `Timer`, `TokioIo`, `SpawnBlocking` and `YieldNow` are now available on
    the `Nursery` and the tracing wrappers when the wrapped executor provides those.
    Note that blocking tasks spawned with `SpawnBlocking` are not managed by
    the nursery. In any case they can't be interrupted once they start running.

## Updated
  - **BREAKING**: Updated _async_executors_ to 0.5.

## [0.3.1] - 2021-06-11

  [0.3.1]: https://github.com/najamelan/async_nursery/compare/0.3.0...0.3.1

## Fixed
  - Remove external_doc for rustdoc 1.54
  - Move CI to github


## [0.3.0] - 2021-01-01

[0.3.0]: https://github.com/najamelan/async_nursery/compare/0.3.0-beta.1...0.3.0

### Updated
  - **BREAKING CHANGE**: Update _async_nursery_ to 0.4.0 (supports tokio 1.0).

## [0.3.0-beta.1] - 2020-11-01

[0.3.0-beta.1]: https://github.com/najamelan/async_nursery/compare/0.2.0...0.3.0-beta.1

### Updated
  - **BREAKING CHANGE**: Update _async_nursery_ to 0.4.0-beta.1. Will drop beta when tokio releases 1.0

### Added
  - add proper support for tracing `Instrument` and `WithDispatch`.

### Fixed
  - remove thiserror dependency.

## [0.2.0] - 2020-06-11

[0.2.0]: https://github.com/najamelan/async_nursery/compare/0.1.0...0.2.0

### Updated
  - Update _async_nursery_ to 0.3.

## 0.1.0 - 2020-04-30

  - Initial release.




