//! The point of cooperative cancellation is to not be potentially canceled at each await point
//! but to be able to do cleanup. Another reason for coop cancellation is when there are not enough
//! await points. Drop canceling only works at await points, and if a task does to much work
//! in between, it cannot be canceled unless it regularly checks a cancellation token. It is however
//! already an anti-pattern to do to much sync work in an async task because it will block the thread
//! and thus other tasks running on it.
//!
//! One possible implementation of cooperative cancellation. I haven't found any really convenient way.
//! Ideally one has a broadcast channel on which the receiver can be cloned to pass it to an arbitrary
//! number of tasks running concurrently.
//!
//! Here we just use pharos which has some limitations. The Events stream doesn't implement `Clone`
//! and doesn't have a `try_recv` method to just use a loop and check for the cancellation without
//! blocking the task.
//!
//! Other options are using a futures oneshot or tokio::sync::Notify for each task. Async drop will
//! also provide a way to do cleanup when canceled.
//!
//! This shows tasks that wait for an increasing number of seconds up to 5. After 3
//! seconds we cancel them. So 3, 4 and 5 never complete but return early after cancellation.
//!
//! Expected output in 3 seconds:
//!
//! $ cargo run --example cancel_coop
//!
//! INFO [cancel_coop] nursery created
//! INFO [cancel_coop] spawned slow: 1
//! INFO [cancel_coop] spawned slow: 2
//! INFO [cancel_coop] spawned slow: 3
//! INFO [cancel_coop] spawned slow: 5
//! INFO [cancel_coop] spawned slow: 4
//! INFO [cancel_coop] end of cancel_coop.
//! INFO [cancel_coop] ended slow: 1
//! INFO [cancel_coop] ended slow: 2
//! INFO [cancel_coop] canceling
//! INFO [cancel_coop] slow 5 doing cleanup
//! INFO [cancel_coop] slow 3 doing cleanup
//! INFO [cancel_coop] slow 4 doing cleanup
//!
mod common;

use
{
	async_executors :: { AsyncStd                                  } ,
	async_nursery   :: { Nursery, Nurse, NurseExt                  } ,
	log             :: { info                                      } ,
	std             :: { time::Duration                            } ,
	futures_timer   :: { Delay                                     } ,
	futures         :: { select, StreamExt, FutureExt, SinkExt     } ,
	common          :: { DynResult                                 } ,
	pharos          :: { Pharos, Events, ObserveConfig, Observable } ,
};



fn cancel_coop( amount: usize, nursery: impl Nurse<()> ) -> DynResult<Pharos<()>>
{
	// Because of limitations in the pharos API needs to be available where spawning to be
	// able to observe. Events is not Clone.
	//
	let mut pharos = Pharos::default();

	for i in 1..=amount
	{
		nursery.nurse( slow(i, pharos.observe( ObserveConfig::default() )? ) )?;
	}

	info!( "end of cancel_coop." );
	Ok(pharos)
}



// Try to sleep i times, but respect a cancellation request. This particular code
// is not very realistic. The point of coop cancellation is to do cleanup, but that
// is not shown here.
//
async fn slow( i: usize, cancel: Events<()> )
{
	info!( "spawned slow: {}", i );

	let mut cancel = cancel.fuse();
	let mut counter = 0usize;

	// Do an action i times, but respect a cancel request, checking it between
	// each iteration.
	//
	while counter < i
	{
		let mut wait = Delay::new( Duration::from_secs(1) ).fuse();

		select!
		{
			// Allows detecting the cancellation rather than just being dropped, so
			// we could do some cleanup here instead of just disappearing.
			//
			_ = cancel.next() =>
			{
				info!( "slow {} doing cleanup", i );
				return;
			}

			_ = wait          => counter += 1,
		}
	}


	info!( "ended slow: {}", i );
}



#[ async_std::main ]
//
async fn main() -> DynResult<()>
{
	flexi_logger::Logger::with_str( "debug, async_std=warn" ).start().unwrap();

	let (nursery, output) = Nursery::new( AsyncStd ); info!( "nursery created" );

	// cancel_coop will be able to spawn tasks that outlive it's own lifetime,
	// and if its async, we can just spawn it on the nursery as well.
	//
	let mut pharos = cancel_coop( 5, nursery.clone() )?;

	// cancel after 3 seconds.
	//
	Delay::new( Duration::from_secs(3) ).await;

	info!( "canceling" );

	pharos.send(()).await?;

	// This is necessary. Since we could keep spawning tasks even after starting to poll
	// the output, it can't know that we are done, unless we drop all senders or call
	// `close_nursery`.
	//
	nursery.close_nursery();

	output.await;

	Ok(())
}
