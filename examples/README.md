# Examples

These examples go over a number of different scenarios you might need in developing applications with structured concurrency. The links in this document point to examples illustrating the use case.

1. [resource_drop](resource_drop.rs): All tasks get canceled when the `NurseryStream` goes out of scope. Works in functions and spawned tasks.
1. [resource_await](resource_await.rs): Wait for all tasks to finish. Works in functions and spawned tasks.
1. [resource_outlive](resource_outlive.rs): Let functions spawn on a nursery that outlives them.
1. [cancel_coop](cancel_coop.rs): Cooperative cancellation, if you can't afford being dropped at await points.
1. [cancel_coop_all](cancel_coop_all.rs): Cooperative cancellation through closing the `Nursery` for a special case.
1. [return_value](return_value.rs): Use stream to evaluate all returned values.
1. [return_progress](return_progress.rs): Use stream to evaluate all returned values. A progress bar.
1. [return_error](return_error.rs): Use TryStreamExt to bail as soon as one error happens.
1. [return_catch_unwind](return_catch_unwind.rs): Bail if any task panics, without panicking the current thread.
1. [subtask_ref](subtask_ref.rs): Pass references into function calls instead of cloning the `Nursery`.
1. [subtask_spawn](subtask_spawn.rs): Let spawned tasks spawn subtasks on a nursery passed in.
1. [single-thread](single_thread.rs): It all works single threaded too. Spawn !Send tasks.
1. [wasm](wasm): It all works in wasm too.
1. [type_bound](type_bound.rs): Shows how you can save a nursery on a struct, so it's life and that of all spawned tasks is bound to it. The example doesn't do anything when run.
