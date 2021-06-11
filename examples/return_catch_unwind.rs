//! We can use catch_unwind to keep a panicking task from crashing the application.
//! This is useful if resilience is needed for a long running application.
//!
//! You have to assert that your task is unwind safe before doing this!
//!
//! In this example we still bail out of sibling tasks if any spawned task panics,
//! but you can also keep them running by useing `next` instead of `try_next`. You
//! can then capture all the results in a collection with `collect` to inspect them.
//!
//! Expected output:
//!
//! $ cargo run --example return_catch_unwind
//!
//! INFO [return_catch_unwind] nursery created
//! INFO [return_catch_unwind] spawned slow.
//! INFO [return_catch_unwind] spawned wrong.
//! ERROR [return_catch_unwind] panicked at 'I don't like waiting.', examples/return_catch_unwind.rs:72:5
//! INFO [return_catch_unwind] nursery created
//! INFO [return_catch_unwind] spawned slow.
//! INFO [return_catch_unwind] spawned wrong.
//! ERROR [return_catch_unwind] panicked at 'I don't like waiting.', examples/return_catch_unwind.rs:72:5
//! INFO [return_catch_unwind] nursery created
//! INFO [return_catch_unwind] spawned slow.
//! INFO [return_catch_unwind] spawned wrong.
//! //! ... a 100 times.
//!
mod common;

use
{
	async_executors :: { AsyncStd                } ,
	async_nursery   :: { Nursery, NurseExt       } ,
	common          :: { setup_tracing           } ,
	futures         :: { FutureExt, TryStreamExt } ,
	futures_timer   :: { Delay                   } ,
	std             :: { time::Duration          } ,
	tracing_crate   :: { info, error             } ,
};


async fn return_catch_unwind() -> std::thread::Result<()>
{
	let (nursery, mut output) = Nursery::new( AsyncStd ); info!( "nursery created" );

	nursery.nurse( slow()                 )?;
	nursery.nurse( wrong().catch_unwind() )?;

	// This is necessary. Since we could keep spawning tasks even after starting to poll
	// the output, it can't know that we are done, unless we drop all senders or call
	// `close_nursery`.
	//
	// Of course if an error happens, it wouldn't matter, but if no error happens it would
	// make the while loop below hang indefinitely.
	//
	drop(nursery);

	// Can't do it with TryFuture because we would need specialization.
	// Use TryStreamExt. This will return as soon as an error happens in
	// any spawned task and thus drop all the others when the current function
	// returns.
	//
	while output.try_next().await?.is_some() {};

	unreachable!( "drop Nursery and NurseryStream" );
}



// Linger.
//
async fn slow() -> std::thread::Result<()>
{
	info!( "spawned slow." );

	Delay::new( Duration::from_secs(5) ).await;

	error!( "I managed to stall you all." );

	Ok(())
}



// This will return an error.
//
async fn wrong()
{
	info!( "spawned wrong." );

	panic!( "I don't like waiting." )
}



#[ async_std::main ]
//
async fn main()
{
	setup_tracing();

	// log errors instead of returning them on the stderr.
	//
	std::panic::set_hook( Box::new( |e| error!( "{}", e ) ) );

	for _ in 0..100
	{
		let err = return_catch_unwind().await;
		assert!( err.is_err() );
	}
}
