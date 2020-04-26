# TODO

## Tests

- check code coverage and add tests.
- loom tests.
- test what happens if spawner is not 'static
- verify drop behavior of futures unordered

## Features

- add timeout support.
- consider being channel agnostic?
- cooperative canceling


- what about restart? does FuturesUnordered support polling after it has ended?

- make Sink::poll_close wait for the in_flight to be 0?

- what happens when people use try_collect. The TryCollect from futures stops polling as soon as an error happens, but the nursery shouldn't have any
  orphaned tasks that keep running after the stream ends...


