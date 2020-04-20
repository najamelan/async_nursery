# TODO

- what happens when people use try_collect. The TryCollect from futures stops polling as soon as an error happens, but the nursery shouldn't have any
  orphaned tasks that keep running after the stream ends...

- cooperative canceling

- verify drop behavior of futures unordered
