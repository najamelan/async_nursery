# Examples

These examples go over a number of different scenarios you can need in developing applications with structured concurrency. The links in this document point to examples illustrating the use case.

## Avoiding resource leakage

When the nursery goes out of scope, we guarantee no spawned tasks are running anymore. It can be achieved in two ways:

- Drop the nursery. All spawned tasks will be woken up to be dropped. If a task is running, note that it will only be dropped
  the next time it yields.
- Wait for all spawned tasks to complete naturally.
- Cooperative cancellation. There is no straight forward universal mechanism for cooperative cancellation in Rust, so it comes down to manually pass in a cancellation token to spawned tasks that they will check (eg. on every iteration of a loop). The Nursery itself supports cooperative cancellation in that you can "close it". Now any task trying to spawn a future on it will get an error and no new tasks will be accepted while allowing existing tasks to terminate. If you need more fine grained cancellation, you will need to implement it manually.

- within a function, split to concurrent execution but guarantee all paths have joined before returning.

- pass nursery to called functions
  - sync
  - async
  - by reference or by clone

- pass the nursery to spawned tasks.

If you don't care about the return types, `NurseryStream` implements future so you can just `await` it until all tasks are done.


## Error propagation

In the simpler spawn model, tasks go out in a void. If you want to know anything about them you need to communicate with them through channels.
The arrival of executors that return a `JoinHandle` opened the path for at least retrieving return values, which can be `Result`s. This allows
propagating errors back up the stack.

Manually managing `JoinHandle`s can be a hurdle. It doesn't scale well, especially if tasks can complete in random order. FuturesUnordered can
help to manage them, but:

`FuturesUnordered` allows only working with two distinct phases. First you spawn your tasks and then you use it as a `Stream` of outputs. The `Nursery` is more flexible here by letting you mix both uses. It creates the potential for a long running `Nursery` eg. tied to the lifetime of a network connection that spawns incoming requests which may fail. You can listen to their outputs while continuing to spawn new requests on them. The stream will end when all tasks have completed and all nurseries have been dropped. Alternatively to dropping them, you can call `close_nursery` to let the stream finish without dropping all Nursery clones.

The functionality of `TryStreamExt::try_next` can be used to bail early if all concurrent tasks need to complete successfully. You can now drop the `NurseryStream` and cancel all running sibling tasks.

### Panics

Nursery has no special handling of panics. If your task panics, it depends on the executor what happens. Currently _tokio_ is different from other executors in that it will `catch_unwind` your spawned tasks. Other executors propagate the panic to the thread that awaits the `JoinHandle`s (eg. that has the `NurseryStream`). If you want a resilient application that works on all executors, use the `catch_unwind` combinator from the futures library. Again using `TryStreamExt::try_next` you can bail early if one task panics.

## Recover other return types

It's possible to return useful data sometimes from spawned tasks. You can effectively see them as function calls or closures that can run concurrently. The nursery let's you recover these as you go. It could be used to implement a progress bar for example.

Another possibility is using `collect` to gain a collection of all returned values when everything is done.


## Examples

1. [resource_drop](resource_drop.rs): All tasks get canceled when the `NurseryStream` goes out of scope. Works in functions and spawned tasks.
1. [resource_await](resource_await.rs): Wait for all tasks to finish. Works in functions and spawned tasks.
1. [resource_outlive](resource_outlive.rs)[]: Let functions spawn on a nursery that outlives them.
1. [cancel_coop](cancel_coop.rs): Cooperative cancellation.
1. [cancel_coop_all](cancel_coop_all.rs): Cooperative cancellation through closing the `Nursery`.
1. [return_value](return_value.rs): Use stream to evaluate all returned values.
1. [return_progress](return_progress.rs): Use stream to evaluate all returned values. A progress bar.
1. [return_error](return_error.rs): Use TryStreamExt to bail as soon as one error happens.
1. [return_catch_unwind](return_catch_unwind.rs): Bail if any task panics, without panicking the current thread.
1. [return_catch_unwind_all](return_catch_unwind_all.rs): Bail if any task panics, without panicking the current thread, use catch_unwind on the NurserySteam if you don't need output values.
1. [subtask_ref](subtask_ref.rs): Pass references into function calls instead of cloning the `Nursery`.
1. [subtask_spawn](subtask_spawn.rs): Let spawned tasks spawn subtasks on a nursery passed in.
1. [single-thread](single_thread.rs): It all works single threaded too. Spawn !Send tasks.
1. [wasm](wasm): It all works in wasm too.
1. [type_bound](type_bound.rs): Shows how you can save a nursery on a struct, so it's life and that of all spawned tasks is bound to it. Doesn't do anything when run.
