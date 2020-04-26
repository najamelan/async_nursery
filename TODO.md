# TODO

- add timeout support.
- consider being channel agnostic?
- test what happens if spawner is not 'static
- thorough use case analysis and turn those into tests
- loom tests.

- what about restart? does FuturesUnordered support polling after it has ended?

- make Sink::poll_close wait for the in_flight to be 0?

- what happens when people use try_collect. The TryCollect from futures stops polling as soon as an error happens, but the nursery shouldn't have any
  orphaned tasks that keep running after the stream ends...

- cooperative canceling

- verify drop behavior of futures unordered
