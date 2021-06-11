//! The point of cooperative cancellation is to not be potentially canceled at each await point
//! but to be able to do cleanup.
//!
//! This is but one possible implementation of cooperative cancellation. This just uses an AtomicBool to indicate
//! that tasks should stop.
//!
//! Async drop should also provide a way to do cleanup when canceled, but doesn't exist yet.
//!
//! This shows tasks that wait for an increasing number of seconds up to 5. After 2
//! seconds we cancel them. So 3, 4 and 5 never complete but return early after cancellation.
//!
//! Expected output in 2 seconds:
//!
//! $ cargo run --example cancel_coop
//!
//! INFO cancel_coop: nursery created
//! INFO cancel_coop: end of cancel_coop.
//! INFO cancel_coop: spawned slow: 1
//! INFO cancel_coop: spawned slow: 5
//! INFO cancel_coop: spawned slow: 4
//! INFO cancel_coop: spawned slow: 2
//! INFO cancel_coop: spawned slow: 3
//! INFO cancel_coop: ended slow: 1
//! INFO cancel_coop: canceling
//! INFO cancel_coop: ended slow: 2
//! INFO cancel_coop: slow 4 doing cleanup
//! INFO cancel_coop: slow 3 doing cleanup
//! INFO cancel_coop: slow 5 doing cleanup
//!
mod common;

use
{
	async_executors :: { AsyncStd                                                               } ,
	async_nursery   :: { Nursery, Nurse, NurseExt                                               } ,
	common          :: { DynResult, setup_tracing                                               } ,
	futures_timer   :: { Delay                                                                  } ,
	std             :: { time::Duration, sync::{ Arc, atomic::{ AtomicBool, Ordering::SeqCst }} } ,
	tracing_crate   :: { info                                                                   } ,
};



fn cancel_coop( amount: usize, nursery: impl Nurse<()>, cancel: Arc<AtomicBool>  ) -> DynResult<()>
{
	for i in 1..=amount
	{
		nursery.nurse( slow(i, cancel.clone()) )?;
	}

	info!( "end of cancel_coop." );

	Ok(())
}



// Try to sleep i times, but respect a cancellation request.
//
// NOTE: This is just an example of a coop cancellation mechanism with async_nursery.
// The function below doesn't do any cleanup. In practice, if you are fine with potentially
// being dropped at an await point, you don't need cooperative cancellation.
//
async fn slow( i: usize, cancel: Arc<AtomicBool> )
{
	info!( "spawned slow: {}", i );

	// Do an action i times, but respect a cancel request, checking it between
	// each iteration.
	//
	for _ in 0..i
	{
		if cancel.load( SeqCst )
		{
			info!( "slow {} doing cleanup", i );
			return;
		}

		Delay::new( Duration::from_secs(1) ).await;
	}

	info!( "ended slow: {}", i );
}



#[ async_std::main ]
//
async fn main() -> DynResult<()>
{
	setup_tracing();

	let (nursery, output) = Nursery::new( AsyncStd ); info!( "nursery created" );

	// cancel_coop will be able to spawn tasks that outlive it's own lifetime,
	// and if its async, we can just spawn it on the nursery as well.
	//
	let cancel = Arc::new( AtomicBool::new( false ) );
	cancel_coop( 5, nursery.clone(), cancel.clone() )?;

	// cancel after 2 seconds.
	//
	Delay::new( Duration::from_secs(2) ).await;

	info!( "canceling" );
	cancel.store( true, SeqCst );

	// This is necessary. Since we could keep spawning tasks even after starting to poll
	// the output, it can't know that we are done, unless we drop all senders or call
	// `close_nursery`.
	//
	drop(nursery);

	// main will wait for subtasks to do cleanup. Of course you could select this
	// with a timeout.
	//
	output.await;

	Ok(())
}
