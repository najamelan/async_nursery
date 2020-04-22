# TODO

- don't spin in stream when not closed.
- thorough use case analysis and turn those into tests
- loom tests.

- what about restart? does FuturesUnordered support polling after it has ended?

- make Sink::poll_close wait for the in_flight to be 0?

- deadlock in mutex. we use block_on in size_hint.

- futures mutex doesn't poison. check unwind safety.

- what happens when people use try_collect. The TryCollect from futures stops polling as soon as an error happens, but the nursery shouldn't have any
  orphaned tasks that keep running after the stream ends...

- cooperative canceling

- verify drop behavior of futures unordered
