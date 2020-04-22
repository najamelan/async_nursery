# TODO

- make Sink::poll_close wait for the in_flight to be 0?

- deadlock in mutex. we use block_on in size_hint.

- futures mutex doesn't poison. check unwind safety.

- error type for send failing in NurseryHandle.

- synchronize, so you cannot spawn a task on a handle and just at that moment the stream on unordered thinks it's done, before
  the call to push... and everything get's dropped while this new task is in transit.

- what happens when people use try_collect. The TryCollect from futures stops polling as soon as an error happens, but the nursery shouldn't have any
  orphaned tasks that keep running after the stream ends...

- cooperative canceling

- verify drop behavior of futures unordered
