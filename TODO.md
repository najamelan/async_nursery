# TODO


## Chores

- nursery for instrumented and withdispatch (You can just feed it an instrumented executor, but then you don't always control creation)

## Questions

- can we make it no-std?
- compare unicycle to FuturesUnordered. -> It might be good in itself, but the problem it solves doesn't matter to us. Main issue right now is the overhead of the channel. We could profile to see how much of the overhead comes from FuturesUnordered, or at least compare the benches against unicycle.

## Tests

- check code coverage and add tests.
- test what happens if spawner is not 'static
- verify drop behavior of futures unordered

## Features

- add timeout support.
- consider being channel agnostic?
- cooperative canceling


